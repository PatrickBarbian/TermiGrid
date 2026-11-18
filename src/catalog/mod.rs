pub mod accessories;

use serde::{Deserialize, Serialize};

use crate::catalog::accessories::{resolve_accessories, TerminalAccessories};
use crate::model::{ConnectionPoint, NumberingOrder, TerminalBlock, TierBehavior, TierColor};

/// Embedded hardcoded database of common WAGO, Phoenix Contact, and Weidmüller terminal blocks.
const EMBEDDED_SPECS_JSON: &str = include_str!("../../assets/terminal_specs.json");

/// Blueprint for a single tier within a catalog specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TierSpec {
    pub behavior: TierBehavior,
    pub color: TierColor,
}

/// Technical specification of a terminal block type from the catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalSpec {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub series: String,
    pub levels: usize,
    pub width_tenths_mm: u16,
    pub numbering_order: NumberingOrder,
    pub description_de: String,
    pub description_en: String,
    pub tiers: Vec<TierSpec>,
    #[serde(default)]
    pub accessories: Option<TerminalAccessories>,
    /// Jumper chambers per tier (`Some(1)` = single slot, `Some(2)` = double slot; `None` infers from series).
    #[serde(default)]
    pub jumper_slots_per_tier: Option<u8>,
    /// Maximum comb-jumper poles available in standard accessories (e.g., 10, 5, 20).
    #[serde(default)]
    pub max_jumper_poles: Option<u8>,
    /// Optional manufacturer order / article number (e.g., "3209510", "1521850000").
    #[serde(default)]
    pub article_number: Option<String>,
    /// Optional explicit manufacturer product page URL.
    #[serde(default)]
    pub product_url: Option<String>,
    /// Optional explicit direct manufacturer image CDN URL.
    #[serde(default)]
    pub image_url: Option<String>,
    /// Optional front-view vertical height across the TS 35 DIN rail in tenths of a millimeter (e.g., `485` = `48.5 mm`).
    #[serde(default)]
    pub height_tenths_mm: Option<u16>,
    /// Optional vertical center offset relative to the TS 35 DIN rail center in tenths of a millimeter
    /// (positive values shift the housing upward above the rail center; `0` = symmetric on TS 35).
    #[serde(default)]
    pub rail_offset_tenths_mm: Option<i16>,
}

impl TerminalSpec {
    /// Instantiates a new `TerminalBlock` with the given ID from this specification.
    pub fn instantiate_block(&self, block_id: u64) -> TerminalBlock {
        let tiers = self
            .tiers
            .iter()
            .enumerate()
            .map(|(idx, ts)| ConnectionPoint::new(idx, ts.behavior.clone(), ts.color))
            .collect();

        TerminalBlock {
            id: block_id,
            spec_id: self.id.clone(),
            spec_name: self.name.clone(),
            manufacturer: self.manufacturer.clone(),
            width_tenths_mm: self.width_tenths_mm,
            numbering_order: self.numbering_order,
            tiers,
        }
    }

    /// Counts how many tiers in this spec consume a sequential number.
    pub fn sequential_tier_count(&self) -> usize {
        self.tiers
            .iter()
            .filter(|t| t.behavior.consumes_sequence())
            .count()
    }

    /// Returns a compact pattern summary such as "L / L", "PE / L / N", or "N".
    pub fn pattern_summary(&self) -> String {
        self.tiers
            .iter()
            .map(|t| match &t.behavior {
                TierBehavior::Sequential => "L".to_string(),
                TierBehavior::Fixed(s) => s.clone(),
                TierBehavior::ProtectiveEarth => "PE".to_string(),
                TierBehavior::Unlabeled => "—".to_string(),
            })
            .collect::<Vec<_>>()
            .join(" / ")
    }

    /// Number of jumper/bridge chambers per tier (`1` = single chamber, `2` = dual chamber).
    pub fn effective_jumper_slots(&self) -> u8 {
        if let Some(slots) = self.jumper_slots_per_tier {
            return slots.clamp(1, 2);
        }
        let id = self.id.as_str();
        // Single-chamber series: compact 1.5mm², installation multi-tier (PTI/AITB/2003/2005/DLI),
        // disconnect/fuse/sensor terminals, high-current blocks, and legacy 280/281.
        if id.contains("2000-")
            || id.contains("2001-")
            || id.contains("2003-")
            || id.contains("2005-")
            || id.contains("2200-")
            || id.contains("2201-")
            || id.contains("2203-")
            || id.contains("2205-")
            || id.contains("2010-")
            || id.contains("2016-")
            || id.contains("2110-")
            || id.contains("2116-")
            || id.contains("2210-")
            || id.contains("2216-")
            || id.contains("2022-")
            || id.contains("2007-")
            || id.contains("280-")
            || id.contains("281-")
            || id.contains("285-")
            || id.contains("-161")
            || id.contains("-166")
            || id.contains("-168")
            || id.contains("-171")
            || id.contains("-181")
            || id.contains("-198")
            || id.contains("-260")
            || id.contains("-296")
            || id.contains("15s")
            || id.contains("pti-")
            || id.contains("ptio-")
            || id.contains("sti-")
            || id.contains("uti-")
            || id.contains("ptpower")
            || id.contains("pt-10")
            || id.contains("pt-16")
            || id.contains("pt-35")
            || id.contains("ptme")
            || id.contains("-hesi")
            || id.contains("-mt")
            || id.contains("-tg")
            || id.contains("uk-")
            || id.contains("uslkg")
            || id.contains("a2c-15")
            || id.contains("a3c-15")
            || id.contains("a4c-15")
            || id.contains("a2c-10")
            || id.contains("a2c-16")
            || id.contains("a2c-35")
            || id.contains("aio")
            || id.contains("aitb")
            || id.contains("dli")
            || id.contains("zdl")
            || id.contains("wdu-10")
            || id.contains("wdu-16")
            || id.contains("wdu-35")
            || id.contains("wtr")
            || id.contains("wsi")
        {
            1
        } else {
            2
        }
    }

    /// Maximum consecutive poles offered in standard comb jumper accessories for this block.
    pub fn effective_max_jumper_poles(&self) -> u8 {
        if let Some(poles) = self.max_jumper_poles {
            return poles.max(2);
        }
        let id = self.id.as_str();
        if id.contains("285-")
            || id.contains("ptpower")
            || id.contains("2010-")
            || id.contains("2016-")
            || id.contains("2110-")
            || id.contains("2116-")
            || id.contains("2210-")
            || id.contains("2216-")
            || id.contains("pt-10")
            || id.contains("pt-16")
            || id.contains("pt-35")
            || id.contains("ut-10")
            || id.contains("ut-16")
            || id.contains("a2c-10")
            || id.contains("a2c-16")
            || id.contains("a2c-35")
            || id.contains("wdu-10")
            || id.contains("wdu-16")
            || id.contains("wdu-35")
            || id.contains("wpe-16")
            || id.contains("wpe-35")
        {
            2
        } else if id.contains("2006-")
            || id.contains("2106-")
            || id.contains("2206-")
            || id.contains("pt-6")
            || id.contains("pti-6")
            || id.contains("ut-6")
            || id.contains("a2c-6")
            || id.contains("aitb-6")
            || id.contains("wdu-6")
            || id.contains("zdu-6")
        {
            5
        } else {
            10
        }
    }

    /// Compatibility key for horizontal comb jumpers (manufacturer + pitch + jumper family).
    pub fn jumper_family_key(&self) -> String {
        let family = match self.manufacturer.as_str() {
            "WAGO" => {
                if self.id.starts_with("wago-28") {
                    "280"
                } else {
                    "topjob"
                }
            }
            "Phoenix Contact" => {
                if self.id.contains("-ut") || self.id.contains("-uk") || self.id.contains("-uslkg") {
                    "fbs-ut"
                } else {
                    "fbs"
                }
            }
            "Weidmüller" => {
                if self.id.contains("wdu") || self.id.contains("wpe") || self.id.contains("wdk") || self.id.contains("dli") {
                    "wqv"
                } else {
                    "zqv"
                }
            }
            _ => "generic",
        };
        format!("{}:{}:{}", self.manufacturer, family, self.width_tenths_mm)
    }

    /// Comb jumper part number hint for `poles` poles on this terminal specification.
    pub fn jumper_part_hint(&self, poles: usize) -> String {
        let id = self.id.as_str();
        match self.manufacturer.as_str() {
            "WAGO" => {
                let prefix = if id.contains("2000") || id.contains("2200") {
                    "2000"
                } else if id.contains("2001") || id.contains("2201") {
                    "2001"
                } else if id.contains("2004")
                    || id.contains("2005")
                    || id.contains("2104")
                    || id.contains("2204")
                    || id.contains("2205")
                {
                    "2004"
                } else if id.contains("2006") || id.contains("2106") || id.contains("2206") {
                    "2006"
                } else if id.contains("2010") || id.contains("2110") || id.contains("2210") {
                    "2010"
                } else if id.contains("2016") || id.contains("2116") || id.contains("2216") {
                    "2016"
                } else {
                    "2002"
                };
                format!("{prefix}-4{poles:02} ({poles}-fach)")
            }
            "Phoenix Contact" => {
                let pitch = if self.width_tenths_mm <= 38 {
                    "3,5"
                } else if self.width_tenths_mm <= 55 {
                    "5"
                } else if self.width_tenths_mm <= 65 {
                    "6"
                } else if self.width_tenths_mm <= 85 {
                    "8"
                } else {
                    "10"
                };
                format!("FBS {poles}-{pitch} ({poles}-fach)")
            }
            "Weidmüller" => {
                if id.contains("wdu") || id.contains("wpe") || id.contains("wdk") {
                    format!("WQV 2.5/{poles} ({poles}-fach)")
                } else if self.width_tenths_mm >= 60 {
                    format!("ZQV 4N/{poles} ({poles}-fach)")
                } else {
                    format!("ZQV 2.5N/{poles} ({poles}-fach)")
                }
            }
            _ => format!("{poles}-fach Kammbrücke"),
        }
    }

    /// Effective manufacturer article / order number (`Art.-Nr.`).
    pub fn effective_article_number(&self) -> Option<&str> {
        if let Some((art, _, _)) = resolve_manufacturer_meta(&self.id) {
            return Some(art);
        }
        if let Some(art) = self.article_number.as_deref()
            && !art.is_empty()
        {
            return Some(art);
        }
        None
    }

    /// Clean manufacturer product lookup URL for opening in desktop or mobile browser.
    pub fn effective_product_url(&self) -> String {
        if let Some((_, url, _)) = resolve_manufacturer_meta(&self.id) {
            return url.to_string();
        }
        if let Some(url) = &self.product_url
            && !url.is_empty()
        {
            return url.clone();
        }

        let short = self.name.split(" (").next().unwrap_or(&self.name).trim();
        let article = self.effective_article_number().unwrap_or(short);

        match self.manufacturer.as_str() {
            "WAGO" => {
                let clean_art = short.split('/').next().unwrap_or(short).trim();
                format!("https://www.wago.com/{clean_art}")
            }
            "Phoenix Contact" => {
                let q = url_encode(article);
                format!("https://www.phoenixcontact.com/de-de/suche?q={q}")
            }
            "Weidmüller" => {
                let q = url_encode(article);
                format!("https://eshop.weidmueller.com/de/search/?text={q}")
            }
            _ => format!("https://www.google.com/search?q={}", url_encode(short)),
        }
    }

    /// Direct, unblocked manufacturer media CDN URL (`caas.phoenixcontact.com` / `assets.dam.weidmueller.com`)
    /// when available so product photos load directly without HTML scraping or WAF blocks.
    pub fn effective_image_url(&self) -> Option<String> {
        if let Some((_, _, img)) = resolve_manufacturer_meta(&self.id) {
            return Some(img.to_string());
        }
        if let Some(url) = &self.image_url
            && !url.is_empty()
        {
            return Some(url.clone());
        }
        // Fallback by manufacturer + series/color for user-created custom specs
        match self.manufacturer.as_str() {
            "Phoenix Contact" => Some(phoenix_caas_fallback(self)),
            "Weidmüller" => Some(weidmuller_dam_fallback(self)),
            _ => None,
        }
    }

    /// Front-view vertical height across the TS 35 DIN rail in tenths of a millimeter (`Bauhöhe` quer zur Hutschiene, e.g., `485` = `48.5 mm`).
    pub fn effective_height_tenths_mm(&self) -> u16 {
        if let Some(h) = self.height_tenths_mm
            && h > 0
        {
            return h;
        }
        let id = self.id.as_str();
        match self.manufacturer.as_str() {
            "WAGO" => {
                if id.starts_with("wago-285") {
                    if id.contains("-135") || id.contains("-137") {
                        860
                    } else {
                        1070 // 285-150 / 285-195 (50-95 mm² POWER)
                    }
                } else if id.starts_with("wago-281") {
                    if id.contains("-611") || id.contains("-623") {
                        840
                    } else {
                        590
                    }
                } else if id.starts_with("wago-280") {
                    if self.levels >= 3 || id.contains("-560") {
                        830
                    } else if id.contains("-833") || id.contains("-681") {
                        640
                    } else {
                        500
                    }
                } else if id.starts_with("wago-202") {
                    525 // 2022 X-COM S-SYSTEM
                } else if id.starts_with("wago-2016")
                    || id.starts_with("wago-2116")
                    || id.starts_with("wago-2216")
                {
                    if id.contains("-7714") {
                        997
                    } else if id.contains("-7614") {
                        857
                    } else {
                        698
                    }
                } else if id.starts_with("wago-2010")
                    || id.starts_with("wago-2110")
                    || id.starts_with("wago-2210")
                {
                    678
                } else if id.starts_with("wago-2007") {
                    996
                } else if id.starts_with("wago-2006")
                    || id.starts_with("wago-2106")
                    || id.starts_with("wago-2206")
                {
                    if id.contains("-161") || id.contains("-168") {
                        963
                    } else if id.contains("-130") {
                        733
                    } else {
                        575
                    }
                } else if id.starts_with("wago-2005") || id.starts_with("wago-2205") {
                    1050
                } else if id.starts_with("wago-2004")
                    || id.starts_with("wago-2104")
                    || id.starts_with("wago-2204")
                {
                    if id.contains("-140") {
                        787
                    } else if id.contains("-130") {
                        655
                    } else {
                        523
                    }
                } else if id.starts_with("wago-2003") || id.starts_with("wago-2203") {
                    960
                } else if self.levels >= 3 {
                    if id.starts_with("wago-2000") {
                        811
                    } else {
                        933
                    }
                } else if self.levels == 2 {
                    if id.starts_with("wago-2000") {
                        615
                    } else {
                        697
                    }
                } else if id.contains("-161")
                    || id.contains("-166")
                    || id.contains("-181")
                    || id.contains("-198")
                {
                    661
                } else if id.contains("-140") {
                    if id.starts_with("wago-2000") {
                        679
                    } else {
                        699
                    }
                } else if id.contains("-130") || id.contains("-171") {
                    if id.starts_with("wago-2000") {
                        582
                    } else {
                        592
                    }
                } else {
                    485 // Standard TOPJOB S 2-conductor (2000/2001/2002/2102/2202)
                }
            }
            "Phoenix Contact" => {
                if id.contains("ptpower-95") {
                    1055
                } else if id.contains("ptpower-50") {
                    1010
                } else if id.contains("ptpower-35") || id.contains("pt-35") {
                    916
                } else if id.contains("pti-") || id.contains("sti-") || id.contains("uti-") {
                    1010
                } else if id.contains("-3l") || id.contains("-peln") || id.contains("-pell") {
                    if id.contains("15s") {
                        972
                    } else {
                        1020
                    }
                } else if id.contains("pttb-4") {
                    835
                } else if id.contains("uttb") {
                    699
                } else if id.contains("pttb-15s") {
                    654
                } else if id.contains("pttb") || id.contains("sttb") || id.contains("pttbs") {
                    680
                } else if id.contains("pt-16") || id.contains("ut-16") {
                    754
                } else if id.contains("pt-10") || id.contains("ut-10") {
                    677
                } else if id.contains("pt-6") || id.contains("ut-6") || id.contains("ptme-6") {
                    577
                } else if id.contains("ptio") {
                    745
                } else if id.contains("pt-4") || id.contains("ut-4") || id.contains("ptv-4") {
                    if id.contains("-hesi") {
                        625
                    } else if id.contains("-quattro") {
                        770
                    } else if id.contains("-twin") {
                        665
                    } else {
                        560
                    }
                } else if id.contains("15s") {
                    if id.contains("-quattro") {
                        632
                    } else if id.contains("-twin") {
                        540
                    } else if id.contains("-mt") {
                        589
                    } else {
                        450
                    }
                } else if id.contains("-quattro") {
                    722
                } else if id.contains("-twin") {
                    604
                } else if id.contains("-hesi") {
                    625
                } else if id.contains("-mt") || id.contains("-tg") {
                    620
                } else if id.contains("st-25") {
                    485
                } else if id.contains("ut-25") || id.contains("uk-5n") || id.contains("uslkg") {
                    477
                } else {
                    486 // Standard PT 2,5 / PTV 2,5
                }
            }
            "Weidmüller" => {
                if id.contains("a3t") {
                    1160
                } else if id.contains("aitb") {
                    1030
                } else if id.contains("zdl") {
                    950
                } else if id.contains("a2t-4") {
                    1000
                } else if id.contains("a2t") || id.contains("zdk") {
                    900
                } else if id.contains("wdk") {
                    690
                } else if id.contains("dli") {
                    660
                } else if id.contains("aio") {
                    800
                } else if id.contains("a2c-35") {
                    1015
                } else if id.contains("a2c-16") {
                    805
                } else if id.contains("a2c-10") {
                    740
                } else if id.contains("a2c-6") {
                    665
                } else if id.contains("a3c-4") {
                    740
                } else if id.contains("a2c-4") || id.contains("afs-4") {
                    600
                } else if id.contains("a4c-25") {
                    775
                } else if id.contains("a3c-25") {
                    665
                } else if id.contains("a4c-15") {
                    675
                } else if id.contains("a3c-15") {
                    615
                } else if id.contains("a2c-15")
                    || id.contains("a2c-25")
                    || id.contains("amc-25")
                    || id.contains("adt-25")
                {
                    550
                } else if id.contains("zdu-4") || id.contains("zpe-4") {
                    640
                } else if id.contains("zdu") || id.contains("zpe") {
                    595
                } else {
                    600 // Standard WDU / WPE / WTR / WSI
                }
            }
            _ => match self.levels {
                1 => 485,
                2 => 697,
                _ => 960,
            },
        }
    }

    /// Vertical center offset relative to the TS 35 DIN rail center in tenths of a millimeter
    /// (`> 0` shifts the housing upward above the rail center; `0` = symmetric on TS 35).
    pub fn effective_rail_offset_tenths_mm(&self) -> i16 {
        if let Some(off) = self.rail_offset_tenths_mm {
            return off;
        }
        let id = self.id.as_str();
        // Multi-level N-busbar installation terminals sit asymmetrically higher above the TS 35 rail
        if id.contains("wago-2005") || id.contains("wago-2205") {
            135
        } else if id.contains("pti-")
            || id.contains("sti-")
            || id.contains("uti-")
            || id.contains("aitb")
        {
            130
        } else if id.contains("wago-2003") || id.contains("wago-2203") || id.contains("-7714") {
            125
        } else if id.contains("zdl") {
            120
        } else if id.contains("dli") {
            60
        } else if id.contains("ptio") || id.contains("aio") || id.contains("280-560") {
            80
        } else if id.contains("-hesi")
            || id.contains("wsi")
            || id.contains("afs-4")
            || id.contains("2006-161")
            || id.contains("2006-168")
            || id.contains("2002-161")
            || id.contains("281-611")
            || id.contains("281-623")
        {
            45
        } else {
            0
        }
    }

    /// Returns `true` if this specification is a 16 mm² N-busbar supply terminal (`N-Einspeiseklemme`),
    /// such as WAGO `2016-7714` / `2216-7714` / `2016-7614`, Phoenix `PTI 16-NLS-FI`, or Weidmüller `AITB 16 BB NDT`.
    pub fn is_n_supply_terminal(&self) -> bool {
        matches!(
            self.id.as_str(),
            "wago-2016-7714"
                | "wago-2216-7714"
                | "wago-2016-7614"
                | "phoenix-pti-16-nls-bu"
                | "phoenix-pti-16-nls"
                | "weidmuller-aitb-16-ndt-bl"
                | "weidmuller-aitb-16-nlo-bl"
        )
    }

    /// Returns `true` if this terminal is an `NT` installation terminal with N-disconnect slide (`N-Trennschlitten`).
    pub fn is_nt_disconnect_terminal(&self) -> bool {
        if self.is_n_supply_terminal() {
            return false;
        }
        let id = self.id.as_str();
        self.name.contains("NT")
            || self.name.contains("NDT")
            || id.ends_with("-lnt")
            || id.ends_with("-pelnt")
            || id.ends_with("-ntlp")
            || id.ends_with("-7640")
            || id.ends_with("-7641")
            || id.ends_with("-6640")
            || id.ends_with("-6641")
    }

    /// Returns `true` if this terminal is a building-installation terminal (`2003`, `2203`, `2005`, `2205`,
    /// `PTI`, `STI`, `UTI`, `AITB`, `ZDL`, `DLI`) or any `NT` terminal that should prompt for NT / N-Einspeiseklemme.
    pub fn is_installation_or_nt_terminal(&self) -> bool {
        if self.is_n_supply_terminal() {
            return false;
        }
        let id = self.id.as_str();
        id.starts_with("wago-2003-")
            || id.starts_with("wago-2203-")
            || id.starts_with("wago-2005-")
            || id.starts_with("wago-2205-")
            || id.starts_with("phoenix-pti-")
            || id.starts_with("phoenix-sti-")
            || id.starts_with("phoenix-uti-")
            || id.starts_with("weidmuller-aitb-")
            || id.starts_with("weidmuller-zdl-")
            || id.starts_with("weidmuller-dli-")
            || self.is_nt_disconnect_terminal()
    }

    /// If this terminal is an installation terminal without `NT` (e.g. `PE/L/N`, `PE/L/L`, `L/L`, `L/N` in
    /// WAGO `2003`/`2203`/`2005`/`2205`, Phoenix `PTI`, or Weidmüller `AITB`), returns the matching `NT`
    /// specification ID so the user can switch to the `NT` variant directly in the prompt dialog.
    pub fn matching_nt_spec_id(&self) -> Option<&'static str> {
        if self.is_nt_disconnect_terminal() || self.is_n_supply_terminal() {
            return None;
        }
        let id = self.id.as_str();
        if id.starts_with("wago-2003-66") {
            Some("wago-2003-6640")
        } else if id.starts_with("wago-2003-") {
            if self.levels >= 3 {
                Some("wago-2003-7641")
            } else {
                Some("wago-2003-7640")
            }
        } else if id.starts_with("wago-2203-66") {
            Some("wago-2203-6640")
        } else if id.starts_with("wago-2203-") {
            if self.levels >= 3 {
                Some("wago-2203-7641")
            } else {
                Some("wago-2203-7640")
            }
        } else if id.starts_with("wago-2005-") {
            if self.levels >= 3 {
                Some("wago-2005-7641")
            } else {
                Some("wago-2005-7640")
            }
        } else if id.starts_with("wago-2205-") {
            if self.levels >= 3 {
                Some("wago-2205-7641")
            } else {
                Some("wago-2205-7640")
            }
        } else if id.starts_with("phoenix-pti-25-") {
            if self.levels >= 3 {
                Some("phoenix-pti-25-pelnt")
            } else {
                Some("phoenix-pti-25-lnt")
            }
        } else if id.starts_with("phoenix-pti-4-") {
            Some("phoenix-pti-4-pelnt")
        } else if id.starts_with("phoenix-pti-6-") {
            Some("phoenix-pti-6-pelnt")
        } else if id.starts_with("weidmuller-aitb-25-") {
            Some("weidmuller-aitb-25-ntlp")
        } else if id.starts_with("weidmuller-aitb-4-") {
            Some("weidmuller-aitb-4-ntlp")
        } else {
            None
        }
    }

    /// Returns the default recommended 16 mm² N-supply terminal (`N-Einspeiseklemme`) for this installation block.
    /// For WAGO (`2003`, `2005`, `2203`, `2205`), defaults to `"wago-2016-7714"` (`2016-7714`).
    pub fn recommended_n_supply_spec_id(&self) -> &'static str {
        match self.manufacturer.as_str() {
            "Phoenix Contact" => "phoenix-pti-16-nls-bu",
            "Weidmüller" => "weidmuller-aitb-16-ndt-bl",
            _ => "wago-2016-7714",
        }
    }

    /// Returns all compatible 16 mm² N-supply terminal specification IDs for this installation block's manufacturer.
    pub fn compatible_n_supply_spec_ids(&self) -> &'static [&'static str] {
        match self.manufacturer.as_str() {
            "Phoenix Contact" => &[
                "phoenix-pti-16-nls-bu",
                "phoenix-pti-16-nls",
                "wago-2016-7714",
            ],
            "Weidmüller" => &[
                "weidmuller-aitb-16-ndt-bl",
                "weidmuller-aitb-16-nlo-bl",
                "wago-2016-7714",
            ],
            _ => &[
                "wago-2016-7714",
                "wago-2216-7714",
                "wago-2016-7614",
            ],
        }
    }
}

/// Verified `(article_number, direct_product_url, direct_cdn_image_url)` table for Phoenix Contact
/// (`caas.phoenixcontact.com`) and Weidmüller (`assets.dam.weidmueller.com`) catalog items.
fn resolve_manufacturer_meta(spec_id: &str) -> Option<(&'static str, &'static str, &'static str)> {
    match spec_id {
        // --- PHOENIX CONTACT ---
        "phoenix-pt-15s" => Some(("3208100", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-15s-3208100", "https://caas.phoenixcontact.com/caas/v1/stable/media/213926/full/b408?format=jpg")),
        "phoenix-pt-15s-bu" => Some(("3208126", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-15s-bu-3208126", "https://caas.phoenixcontact.com/caas/v1/stable/media/254872/full/b408?format=jpg")),
        "phoenix-pt-15s-pe" => Some(("3208139", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-15s-pe-3208139", "https://caas.phoenixcontact.com/caas/v1/stable/media/212966/full/b408?format=jpg")),
        "phoenix-pt-15s-twin" => Some(("3208155", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-15s-twin-3208155", "https://caas.phoenixcontact.com/caas/v1/stable/media/265146/full/b408?format=jpg")),
        "phoenix-pt-15s-twin-pe" => Some(("3208171", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-15s-twin-pe-3208171", "https://caas.phoenixcontact.com/caas/v1/stable/media/212966/full/b408?format=jpg")),
        "phoenix-pt-15s-quattro" => Some(("3208197", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-15s-quattro-3208197", "https://caas.phoenixcontact.com/caas/v1/stable/media/213926/full/b408?format=jpg")),
        "phoenix-pt-15s-mt" => Some(("3210301", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-15s-mt-3210301", "https://caas.phoenixcontact.com/caas/v1/stable/media/31502/full/b408?format=jpg")),
        "phoenix-pt-15s-3l" => Some(("3213713", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pt-15s-3l-3213713", "https://caas.phoenixcontact.com/caas/v1/stable/media/202242/full/b408?format=jpg")),
        "phoenix-pttb-15s" => Some(("3208511", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-15s-3208511", "https://caas.phoenixcontact.com/caas/v1/stable/media/220810/full/b408?format=jpg")),
        "phoenix-pttb-15s-pe" => Some(("3208537", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pttb-15s-pe-3208537", "https://caas.phoenixcontact.com/caas/v1/stable/media/235702/full/b408?format=jpg")),
        "phoenix-ptio-15s-3" => Some(("3244410", "https://www.phoenixcontact.com/de-de/produkte/sensor-aktor-klemme-ptio-15s3-3244410", "https://caas.phoenixcontact.com/caas/v1/stable/media/210966/full/b408?format=jpg")),
        "phoenix-pt-25" => Some(("3209510", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-25-3209510", "https://caas.phoenixcontact.com/caas/v1/stable/media/231482/full/b408?format=jpg")),
        "phoenix-pt-25-bu" => Some(("3209523", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-25-bu-3209523", "https://caas.phoenixcontact.com/caas/v1/stable/media/117936/full/b408?format=jpg")),
        "phoenix-pt-25-pe" => Some(("3209536", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-25-pe-3209536", "https://caas.phoenixcontact.com/caas/v1/stable/media/121018/full/b408?format=jpg")),
        "phoenix-pt-25-twin" => Some(("3209549", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-25-twin-3209549", "https://caas.phoenixcontact.com/caas/v1/stable/media/226530/full/b408?format=jpg")),
        "phoenix-pt-25-twin-bu" => Some(("3209552", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-25-twin-bu-3209552", "https://caas.phoenixcontact.com/caas/v1/stable/media/117936/full/b408?format=jpg")),
        "phoenix-pt-25-twin-pe" => Some(("3209565", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-25-twin-pe-3209565", "https://caas.phoenixcontact.com/caas/v1/stable/media/127104/full/b408?format=jpg")),
        "phoenix-pt-25-quattro" => Some(("3209578", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-25-quattro-3209578", "https://caas.phoenixcontact.com/caas/v1/stable/media/200178/full/b408?format=jpg")),
        "phoenix-pt-25-quattro-bu" => Some(("3209581", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-25-quattro-bu-3209581", "https://caas.phoenixcontact.com/caas/v1/stable/media/117936/full/b408?format=jpg")),
        "phoenix-pt-25-quattro-pe" => Some(("3209594", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-25-quattro-pe-3209594", "https://caas.phoenixcontact.com/caas/v1/stable/media/127104/full/b408?format=jpg")),
        "phoenix-pt-25-mt" => Some(("3210156", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-25-mt-3210156", "https://caas.phoenixcontact.com/caas/v1/stable/media/31502/full/b408?format=jpg")),
        "phoenix-pt-25-tg" => Some(("3210185", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-25-tg-3210185", "https://caas.phoenixcontact.com/caas/v1/stable/media/31696/full/b408?format=jpg")),
        "phoenix-pt-25-hesi" => Some(("3211861", "https://www.phoenixcontact.com/de-de/produkte/sicherungsklemme-pt-4-hesi-5x20-3211861", "https://caas.phoenixcontact.com/caas/v1/stable/media/31572/full/b408?format=jpg")),
        "phoenix-pttb-25" => Some(("3210567", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-25-3210567", "https://caas.phoenixcontact.com/caas/v1/stable/media/212446/full/b408?format=jpg")),
        "phoenix-pttb-25-ln" => Some(("3210570", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-25-bu-3210570", "https://caas.phoenixcontact.com/caas/v1/stable/media/212446/full/b408?format=jpg")),
        "phoenix-pttb-25-pe" => Some(("3210596", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pttb-25-pe-3210596", "https://caas.phoenixcontact.com/caas/v1/stable/media/235702/full/b408?format=jpg")),
        "phoenix-pttbs-25" => Some(("3209604", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttbs-25-3209604", "https://caas.phoenixcontact.com/caas/v1/stable/media/262156/full/b408?format=jpg")),
        "phoenix-pt-25-3l" => Some(("3210499", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pt-25-3l-3210499", "https://caas.phoenixcontact.com/caas/v1/stable/media/202242/full/b408?format=jpg")),
        "phoenix-pt-25-peln" => Some(("3210538", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-25-peln-3210538", "https://caas.phoenixcontact.com/caas/v1/stable/media/213800/full/b408?format=jpg")),
        "phoenix-pt-25-pell" => Some(("3210541", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-25-pell-3210541", "https://caas.phoenixcontact.com/caas/v1/stable/media/213800/full/b408?format=jpg")),
        "phoenix-pti-16-nls-bu" => Some(("1030131", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-16-nls-fi-bu-1030131", "https://caas.phoenixcontact.com/caas/v1/stable/media/253860/full/b408?format=jpg")),
        "phoenix-pti-16-nls" => Some(("1030130", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-16-nls-fi-1030130", "https://caas.phoenixcontact.com/caas/v1/stable/media/265136/full/b408?format=jpg")),
        "phoenix-pti-25-peln" => Some(("3213950", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-25-peln-3213950", "https://caas.phoenixcontact.com/caas/v1/stable/media/244572/full/b408?format=jpg")),
        "phoenix-pti-25-pelnt" => Some(("3213946", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-25-pelnt-3213946", "https://caas.phoenixcontact.com/caas/v1/stable/media/123154/full/b408?format=jpg")),
        "phoenix-pti-25-pell" => Some(("3213949", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-25-pell-3213949", "https://caas.phoenixcontact.com/caas/v1/stable/media/212236/full/b408?format=jpg")),
        "phoenix-pti-25-ll" => Some(("3213953", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-25-ll-3213953", "https://caas.phoenixcontact.com/caas/v1/stable/media/212236/full/b408?format=jpg")),
        "phoenix-pti-25-ln" => Some(("3213952", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-25-n-3213952", "https://caas.phoenixcontact.com/caas/v1/stable/media/123154/full/b408?format=jpg")),
        "phoenix-pti-25-lnt" => Some(("3213948", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-25-llt-3213948", "https://caas.phoenixcontact.com/caas/v1/stable/media/123154/full/b408?format=jpg")),
        "phoenix-pti-4-peln" => Some(("3214049", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-4-peln-3214049", "https://caas.phoenixcontact.com/caas/v1/stable/media/121390/full/b408?format=jpg")),
        "phoenix-pti-4-pelnt" => Some(("3214047", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-4-pelnt-3214047", "https://caas.phoenixcontact.com/caas/v1/stable/media/198188/full/b408?format=jpg")),
        "phoenix-pti-4-pell" => Some(("3214050", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-4-pell-3214050", "https://caas.phoenixcontact.com/caas/v1/stable/media/227746/full/b408?format=jpg")),
        "phoenix-pti-4-ln" => Some(("3214052", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-4-ll-3214052", "https://caas.phoenixcontact.com/caas/v1/stable/media/198188/full/b408?format=jpg")),
        "phoenix-pti-6-peln" => Some(("3214049", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-4-peln-3214049", "https://caas.phoenixcontact.com/caas/v1/stable/media/121390/full/b408?format=jpg")),
        "phoenix-pti-6-pelnt" => Some(("3214047", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-4-pelnt-3214047", "https://caas.phoenixcontact.com/caas/v1/stable/media/198188/full/b408?format=jpg")),
        "phoenix-pti-6-pell" => Some(("3214050", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-4-pell-3214050", "https://caas.phoenixcontact.com/caas/v1/stable/media/227746/full/b408?format=jpg")),
        "phoenix-pt-4" => Some(("3211757", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-4-3211757", "https://caas.phoenixcontact.com/caas/v1/stable/media/270446/full/b408?format=jpg")),
        "phoenix-pt-4-bu" => Some(("3211760", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-4-bu-3211760", "https://caas.phoenixcontact.com/caas/v1/stable/media/203340/full/b408?format=jpg")),
        "phoenix-pt-4-pe" => Some(("3211766", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-4-pe-3211766", "https://caas.phoenixcontact.com/caas/v1/stable/media/223636/full/b408?format=jpg")),
        "phoenix-pt-4-twin" => Some(("3211771", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-4-twin-3211771", "https://caas.phoenixcontact.com/caas/v1/stable/media/257492/full/b408?format=jpg")),
        "phoenix-pt-4-twin-pe" => Some(("3211780", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-4-twin-pe-3211780", "https://caas.phoenixcontact.com/caas/v1/stable/media/223636/full/b408?format=jpg")),
        "phoenix-pt-4-quattro" => Some(("3211797", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-4-quattro-3211797", "https://caas.phoenixcontact.com/caas/v1/stable/media/231084/full/b408?format=jpg")),
        "phoenix-pt-4-hesi" => Some(("3211861", "https://www.phoenixcontact.com/de-de/produkte/sicherungsklemme-pt-4-hesi-5x20-3211861", "https://caas.phoenixcontact.com/caas/v1/stable/media/31572/full/b408?format=jpg")),
        "phoenix-pt-4-mt" => Some(("3211933", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-4-mt-3211933", "https://caas.phoenixcontact.com/caas/v1/stable/media/31502/full/b408?format=jpg")),
        "phoenix-pttb-4" => Some(("3211786", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-4-3211786", "https://caas.phoenixcontact.com/caas/v1/stable/media/230754/full/b408?format=jpg")),
        "phoenix-pttb-4-bu" => Some(("3211793", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-4-bu-3211793", "https://caas.phoenixcontact.com/caas/v1/stable/media/230754/full/b408?format=jpg")),
        "phoenix-pttb-4-pe" => Some(("3211854", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pttb-4-pe-3211854", "https://caas.phoenixcontact.com/caas/v1/stable/media/262544/full/b408?format=jpg")),
        "phoenix-pt-6" => Some(("3211813", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-6-3211813", "https://caas.phoenixcontact.com/caas/v1/stable/media/231218/full/b408?format=jpg")),
        "phoenix-pt-6-bu" => Some(("3211819", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-6-bu-3211819", "https://caas.phoenixcontact.com/caas/v1/stable/media/203340/full/b408?format=jpg")),
        "phoenix-pt-6-pe" => Some(("3211822", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-6-pe-3211822", "https://caas.phoenixcontact.com/caas/v1/stable/media/269426/full/b408?format=jpg")),
        "phoenix-pt-6-t" => Some(("3212160", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-6-mt-3212160", "https://caas.phoenixcontact.com/caas/v1/stable/media/263612/full/b408?format=jpg")),
        "phoenix-ptme-6" => Some(("3212170", "https://www.phoenixcontact.com/de-de/produkte/messwandlertrennklemme-ptme-6-3212170", "https://caas.phoenixcontact.com/caas/v1/stable/media/263612/full/b408?format=jpg")),
        "phoenix-pt-10" => Some(("3212120", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-10-3212120", "https://caas.phoenixcontact.com/caas/v1/stable/media/214944/full/b408?format=jpg")),
        "phoenix-pt-16n" => Some(("3212138", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-16-n-3212138", "https://caas.phoenixcontact.com/caas/v1/stable/media/229934/full/b408?format=jpg")),
        "phoenix-pt-35" => Some(("3212064", "https://www.phoenixcontact.com/de-de/produkte/hochstromklemme-ptpower-35-3212064", "https://caas.phoenixcontact.com/caas/v1/stable/media/228830/full/b408?format=jpg")),
        "phoenix-ptpower-35" => Some(("3212064", "https://www.phoenixcontact.com/de-de/produkte/hochstromklemme-ptpower-35-3212064", "https://caas.phoenixcontact.com/caas/v1/stable/media/228830/full/b408?format=jpg")),
        "phoenix-ptpower-50" => Some(("3260050", "https://www.phoenixcontact.com/de-de/produkte/hochstromklemme-ptpower-50-3260050", "https://caas.phoenixcontact.com/caas/v1/stable/media/251280/full/b408?format=jpg")),
        "phoenix-ptpower-95" => Some(("3260100", "https://www.phoenixcontact.com/de-de/produkte/hochstromklemme-ptpower-95-3260100", "https://caas.phoenixcontact.com/caas/v1/stable/media/251280/full/b408?format=jpg")),
        "phoenix-ptv-25" => Some(("1078960", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ptv-25-1078960", "https://caas.phoenixcontact.com/caas/v1/stable/media/125318/full/b408?format=jpg")),
        "phoenix-ptv-25-bu" => Some(("1078962", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ptv-25-bu-1078962", "https://caas.phoenixcontact.com/caas/v1/stable/media/125318/full/b408?format=jpg")),
        "phoenix-ptv-25-pe" => Some(("1078963", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ptv-25-pe-1078963", "https://caas.phoenixcontact.com/caas/v1/stable/media/125318/full/b408?format=jpg")),
        "phoenix-ptv-4" => Some(("1088728", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ptv-4-1088728", "https://caas.phoenixcontact.com/caas/v1/stable/media/108504/full/b408?format=jpg")),
        "phoenix-ptv-4-pe" => Some(("1088730", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ptv-4-pe-1088730", "https://caas.phoenixcontact.com/caas/v1/stable/media/108504/full/b408?format=jpg")),
        "phoenix-st-25" => Some(("3031212", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-st-25-3031212", "https://caas.phoenixcontact.com/caas/v1/stable/media/246770/full/b408?format=jpg")),
        "phoenix-st-25-bu" => Some(("3031225", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-st-25-bu-3031225", "https://caas.phoenixcontact.com/caas/v1/stable/media/246770/full/b408?format=jpg")),
        "phoenix-st-25-pe" => Some(("3031238", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-st-25-pe-3031238", "https://caas.phoenixcontact.com/caas/v1/stable/media/246770/full/b408?format=jpg")),
        "phoenix-sttb-25" => Some(("3031270", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-sttb-25-3031270", "https://caas.phoenixcontact.com/caas/v1/stable/media/229202/full/b408?format=jpg")),
        "phoenix-sti-25-peln" => Some(("3031827", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-sti-25-pelnt-3031827", "https://caas.phoenixcontact.com/caas/v1/stable/media/124280/full/b408?format=jpg")),
        "phoenix-ut-25" => Some(("3044076", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-25-3044076", "https://caas.phoenixcontact.com/caas/v1/stable/media/209542/full/b408?format=jpg")),
        "phoenix-ut-25-pe" => Some(("3044092", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ut-25-pe-3044092", "https://caas.phoenixcontact.com/caas/v1/stable/media/253550/full/b408?format=jpg")),
        "phoenix-ut-4" => Some(("3044102", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-4-3044102", "https://caas.phoenixcontact.com/caas/v1/stable/media/195454/full/b408?format=jpg")),
        "phoenix-ut-4-hesi" => Some(("3046032", "https://www.phoenixcontact.com/de-de/produkte/sicherungsklemme-ut-4-hesi-5x20-3046032", "https://caas.phoenixcontact.com/caas/v1/stable/media/31572/full/b408?format=jpg")),
        "phoenix-ut-6" => Some(("3044131", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-6-3044131", "https://caas.phoenixcontact.com/caas/v1/stable/media/195702/full/b408?format=jpg")),
        "phoenix-ut-10" => Some(("3044160", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-10-3044160", "https://caas.phoenixcontact.com/caas/v1/stable/media/119830/full/b408?format=jpg")),
        "phoenix-ut-16" => Some(("3044199", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-16-3044199", "https://caas.phoenixcontact.com/caas/v1/stable/media/49680/full/b408?format=jpg")),
        "phoenix-uttb-25" => Some(("3044636", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-uttb-25-3044636", "https://caas.phoenixcontact.com/caas/v1/stable/media/255718/full/b408?format=jpg")),
        "phoenix-uttb-25-pe" => Some(("3044665", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-uttb-25-pe-3044665", "https://caas.phoenixcontact.com/caas/v1/stable/media/255718/full/b408?format=jpg")),
        "phoenix-uti-25-peln" => Some(("3076028", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-uti-25-pelnt-3076028", "https://caas.phoenixcontact.com/caas/v1/stable/media/236244/full/b408?format=jpg")),
        "phoenix-uk-5n" => Some(("3004362", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-uk-5-n-3004362", "https://caas.phoenixcontact.com/caas/v1/stable/media/268498/full/b408?format=jpg")),
        "phoenix-uslkg-5" => Some(("0441504", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-uslkg-5-0441504", "https://caas.phoenixcontact.com/caas/v1/stable/media/253550/full/b408?format=jpg")),

        // --- WEIDMÜLLER ---
        "weidmuller-a2c-15" => Some(("1552790000", "https://eshop.weidmueller.com/de/a2c-1-5/p/1552790000", "https://assets.dam.weidmueller.com/assets/api/e97bfd80-8178-485e-b52e-869943c7b95c/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-15-bl" => Some(("1552820000", "https://eshop.weidmueller.com/de/a2c-1-5-bl/p/1552820000", "https://assets.dam.weidmueller.com/assets/api/80a54677-0c4a-41a5-865f-fbd26c3ae934/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-15-pe" => Some(("1552680000", "https://eshop.weidmueller.com/de/a2c-1-5-pe/p/1552680000", "https://assets.dam.weidmueller.com/assets/api/24634b68-2c52-4973-a8b2-39f4369157f1/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3c-15" => Some(("1552740000", "https://eshop.weidmueller.com/de/a3c-1-5/p/1552740000", "https://assets.dam.weidmueller.com/assets/api/ad53394d-84ad-4f47-91bd-49fcf385a87c/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a4c-15" => Some(("1552690000", "https://eshop.weidmueller.com/de/a4c-1-5/p/1552690000", "https://assets.dam.weidmueller.com/assets/api/88ff371f-0680-453c-b9b7-a634a3667b96/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-25" => Some(("1521850000", "https://eshop.weidmueller.com/de/a2c-2-5/p/1521850000", "https://assets.dam.weidmueller.com/assets/api/2d76620f-5915-4c72-afd5-0fffe2de88ab/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-25-bl" => Some(("1521880000", "https://eshop.weidmueller.com/de/a2c-2-5-bl/p/1521880000", "https://assets.dam.weidmueller.com/assets/api/80a54677-0c4a-41a5-865f-fbd26c3ae934/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-25-pe" => Some(("1521680000", "https://eshop.weidmueller.com/de/a2c-2-5-pe/p/1521680000", "https://assets.dam.weidmueller.com/assets/api/e4f70efa-ed27-42a1-98c6-372ac81cbe30/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3c-25" => Some(("1521740000", "https://eshop.weidmueller.com/de/a3c-2-5/p/1521740000", "https://assets.dam.weidmueller.com/assets/api/ad53394d-84ad-4f47-91bd-49fcf385a87c/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3c-25-bl" => Some(("1521780000", "https://eshop.weidmueller.com/de/a3c-2-5-bl/p/1521780000", "https://assets.dam.weidmueller.com/assets/api/80a54677-0c4a-41a5-865f-fbd26c3ae934/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3c-25-pe" => Some(("1521670000", "https://eshop.weidmueller.com/de/a3c-2-5-pe/p/1521670000", "https://assets.dam.weidmueller.com/assets/api/e4f70efa-ed27-42a1-98c6-372ac81cbe30/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a4c-25" => Some(("1521690000", "https://eshop.weidmueller.com/de/a4c-2-5/p/1521690000", "https://assets.dam.weidmueller.com/assets/api/88ff371f-0680-453c-b9b7-a634a3667b96/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a4c-25-bl" => Some(("1521700000", "https://eshop.weidmueller.com/de/a4c-2-5-bl/p/1521700000", "https://assets.dam.weidmueller.com/assets/api/80a54677-0c4a-41a5-865f-fbd26c3ae934/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a4c-25-pe" => Some(("1521540000", "https://eshop.weidmueller.com/de/a4c-2-5-pe/p/1521540000", "https://assets.dam.weidmueller.com/assets/api/e4f70efa-ed27-42a1-98c6-372ac81cbe30/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2t-25" => Some(("1547610000", "https://eshop.weidmueller.com/de/a2t-2-5/p/1547610000", "https://assets.dam.weidmueller.com/assets/api/d2054db7-d5a2-42b1-80ce-c5e41a701c86/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2t-25-n" => Some(("1547650000", "https://eshop.weidmueller.com/de/a2t-2-5-vl/p/1547650000", "https://assets.dam.weidmueller.com/assets/api/d2054db7-d5a2-42b1-80ce-c5e41a701c86/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2t-25-pe" => Some(("1547680000", "https://eshop.weidmueller.com/de/a2t-2-5-pe/p/1547680000", "https://assets.dam.weidmueller.com/assets/api/d2054db7-d5a2-42b1-80ce-c5e41a701c86/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3t-25" => Some(("2428510000", "https://eshop.weidmueller.com/de/a3t-2-5/p/2428510000", "https://assets.dam.weidmueller.com/assets/api/b22971ca-7cfb-40f7-b90e-89872195a387/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3t-25-peln" => Some(("2428560000", "https://eshop.weidmueller.com/de/a3t-2-5-n-l-pe/p/2428560000", "https://assets.dam.weidmueller.com/assets/api/b22971ca-7cfb-40f7-b90e-89872195a387/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-4" => Some(("2051180000", "https://eshop.weidmueller.com/de/a2c-4/p/2051180000", "https://assets.dam.weidmueller.com/assets/api/9fb65787-f740-4082-816c-95507b13365b/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-4-bl" => Some(("2051210000", "https://eshop.weidmueller.com/de/a2c-4-bl/p/2051210000", "https://assets.dam.weidmueller.com/assets/api/9fb65787-f740-4082-816c-95507b13365b/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-4-pe" => Some(("2051360000", "https://eshop.weidmueller.com/de/a2c-4-pe/p/2051360000", "https://assets.dam.weidmueller.com/assets/api/28026b79-d38b-406a-9bd6-b853f9a1fdf8/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3c-4" => Some(("2051240000", "https://eshop.weidmueller.com/de/a3c-4/p/2051240000", "https://assets.dam.weidmueller.com/assets/api/3d70aacc-4cef-404f-a4f6-16bcb207700f/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3c-4-pe" => Some(("2051410000", "https://eshop.weidmueller.com/de/a3c-4-pe/p/2051410000", "https://assets.dam.weidmueller.com/assets/api/28026b79-d38b-406a-9bd6-b853f9a1fdf8/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2t-4" => Some(("2539970000", "https://eshop.weidmueller.com/de/a2t-4/p/2539970000", "https://assets.dam.weidmueller.com/assets/api/5cda7b26-655c-46d3-bb03-5e43816e57ac/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-6" => Some(("1992110000", "https://eshop.weidmueller.com/de/a2c-6/p/1992110000", "https://assets.dam.weidmueller.com/assets/api/73f75663-597b-42df-8b20-cfeb0d8e26c6/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-6-bl" => Some(("1991790000", "https://eshop.weidmueller.com/de/a2c-6-bl/p/1991790000", "https://assets.dam.weidmueller.com/assets/api/73f75663-597b-42df-8b20-cfeb0d8e26c6/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-6-pe" => Some(("1991810000", "https://eshop.weidmueller.com/de/a2c-6-pe/p/1991810000", "https://assets.dam.weidmueller.com/assets/api/27c0e626-d361-4552-98e5-3a883e588c43/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-10" => Some(("2490360000", "https://eshop.weidmueller.com/de/a2c-10/p/2490360000", "https://assets.dam.weidmueller.com/assets/api/0c4ad510-744d-4b30-be81-d4c6af5fbf5f/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-10-bl" => Some(("2490370000", "https://eshop.weidmueller.com/de/a2c-10-bl/p/2490370000", "https://assets.dam.weidmueller.com/assets/api/0c4ad510-744d-4b30-be81-d4c6af5fbf5f/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-10-pe" => Some(("2490440000", "https://eshop.weidmueller.com/de/a2c-10-pe/p/2490440000", "https://assets.dam.weidmueller.com/assets/api/27c0e626-d361-4552-98e5-3a883e588c43/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-16" => Some(("2494000000", "https://eshop.weidmueller.com/de/a2c-16/p/2494000000", "https://assets.dam.weidmueller.com/assets/api/b095a3d6-2f03-431c-b533-183241e1913c/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-35" => Some(("2551510000", "https://eshop.weidmueller.com/de/a2c-35/p/2551510000", "https://assets.dam.weidmueller.com/assets/api/0821071c-e508-4d31-bd3a-acc38d69e453/DAP_Master_PP,w_800,q_80")),
        "weidmuller-amc-25" => Some(("2434340000", "https://eshop.weidmueller.com/de/amc-2-5/p/2434340000", "https://assets.dam.weidmueller.com/assets/api/617cbd15-c168-4f24-b8f7-657d3c3c19b5/DAP_Master_PP,w_800,q_80")),
        "weidmuller-adt-25" => Some(("1989820000", "https://eshop.weidmueller.com/de/adt-2-5-2c-or/p/1989820000", "https://assets.dam.weidmueller.com/assets/api/617cbd15-c168-4f24-b8f7-657d3c3c19b5/DAP_Master_PP,w_800,q_80")),
        "weidmuller-afs-4" => Some(("2429860000", "https://eshop.weidmueller.com/de/afs-4-2c-bk/p/2429860000", "https://assets.dam.weidmueller.com/assets/api/7c4b5800-6c04-4b32-a267-63f4c09c89c6/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aio21" => Some(("1992260000", "https://eshop.weidmueller.com/de/aio21-1-5-si/p/1992260000", "https://assets.dam.weidmueller.com/assets/api/07381d74-59de-4526-8590-ef8cfab48422/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-16-ndt-bl" => Some(("2668770000", "https://eshop.weidmueller.com/de/aitb-16-bb-ndt-bl/p/2668770000", "https://assets.dam.weidmueller.com/assets/api/49458f66-30a0-4a55-ae92-01e97380e2bc/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-16-nlo-bl" => Some(("2669140000", "https://eshop.weidmueller.com/de/aitb-16-bb-nlo-bl/p/2669140000", "https://assets.dam.weidmueller.com/assets/api/d577c648-f962-4ac6-b5ca-3bba4407791a/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-25-ntlp" => Some(("2668950000", "https://eshop.weidmueller.com/de/aitb-2-5-bb-ndt-l-pe/p/2668950000", "https://assets.dam.weidmueller.com/assets/api/3d42f558-d204-4df1-a36e-6e08cb5ee8c4/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-25-nlp" => Some(("2668940000", "https://eshop.weidmueller.com/de/aitb-2-5-bb-n-l-pe/p/2668940000", "https://assets.dam.weidmueller.com/assets/api/e281c782-7fa1-4d4f-b663-47c8fcd43a7a/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-25-llp" => Some(("2668930000", "https://eshop.weidmueller.com/de/aitb-2-5-bb-l-l-pe/p/2668930000", "https://assets.dam.weidmueller.com/assets/api/e281c782-7fa1-4d4f-b663-47c8fcd43a7a/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-25-ll" => Some(("2668930000", "https://eshop.weidmueller.com/de/aitb-2-5-bb-l-l-pe/p/2668930000", "https://assets.dam.weidmueller.com/assets/api/e281c782-7fa1-4d4f-b663-47c8fcd43a7a/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-25-nl" => Some(("2668940000", "https://eshop.weidmueller.com/de/aitb-2-5-bb-n-l-pe/p/2668940000", "https://assets.dam.weidmueller.com/assets/api/e281c782-7fa1-4d4f-b663-47c8fcd43a7a/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-4-ntlp" => Some(("2669100000", "https://eshop.weidmueller.com/de/aitb-4-bb-ndt-l-pe/p/2669100000", "https://assets.dam.weidmueller.com/assets/api/d763a83c-329c-4358-bb01-a2dffa7b157b/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-4-nlp" => Some(("2669090000", "https://eshop.weidmueller.com/de/aitb-4-bb-n-l-pe/p/2669090000", "https://assets.dam.weidmueller.com/assets/api/d763a83c-329c-4358-bb01-a2dffa7b157b/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-4-llp" => Some(("2669080000", "https://eshop.weidmueller.com/de/aitb-4-bb-l-l-pe/p/2669080000", "https://assets.dam.weidmueller.com/assets/api/ad0808fe-accb-4a1b-ae29-c579cd18f80b/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-6-nlp" => Some(("2669090000", "https://eshop.weidmueller.com/de/aitb-4-bb-n-l-pe/p/2669090000", "https://assets.dam.weidmueller.com/assets/api/d763a83c-329c-4358-bb01-a2dffa7b157b/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-25" => Some(("1020000000", "https://eshop.weidmueller.com/de/wdu-2-5/p/1020000000", "https://assets.dam.weidmueller.com/assets/api/39e06f09-d8ee-4912-beb4-700715a2acf1/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-25-bl" => Some(("1020080000", "https://eshop.weidmueller.com/de/wdu-2-5-bl/p/1020080000", "https://assets.dam.weidmueller.com/assets/api/953044f8-26dc-4809-87f3-3d76eee0107b/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wpe-25" => Some(("1010000000", "https://eshop.weidmueller.com/de/wpe-2-5/p/1010000000", "https://assets.dam.weidmueller.com/assets/api/8fdcddfc-2a0e-47db-8a7b-025cd551ecce/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-4" => Some(("1020100000", "https://eshop.weidmueller.com/de/wdu-4/p/1020100000", "https://assets.dam.weidmueller.com/assets/api/14db5102-fbfb-4b13-bcd3-afb8cdf5e16d/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-4-bl" => Some(("1020180000", "https://eshop.weidmueller.com/de/wdu-4-bl/p/1020180000", "https://assets.dam.weidmueller.com/assets/api/243b3997-cbb2-4996-8161-77f5ab935a69/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wpe-4" => Some(("1010100000", "https://eshop.weidmueller.com/de/wpe-4/p/1010100000", "https://assets.dam.weidmueller.com/assets/api/06ebc68a-59a5-4aef-ad33-d664be533d82/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-6" => Some(("1020200000", "https://eshop.weidmueller.com/de/wdu-6/p/1020200000", "https://assets.dam.weidmueller.com/assets/api/748df442-2268-4cc8-a823-c247cb16b75c/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-6-bl" => Some(("1020280000", "https://eshop.weidmueller.com/de/wdu-6-bl/p/1020280000", "https://assets.dam.weidmueller.com/assets/api/243b3997-cbb2-4996-8161-77f5ab935a69/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wpe-6" => Some(("1010200000", "https://eshop.weidmueller.com/de/wpe-6/p/1010200000", "https://assets.dam.weidmueller.com/assets/api/4b44dec8-d063-4a5d-8f7f-c182fcf8c356/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-10" => Some(("1020300000", "https://eshop.weidmueller.com/de/wdu-10/p/1020300000", "https://assets.dam.weidmueller.com/assets/api/9819ba81-56c6-48bf-9104-ecf827e950a8/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-16" => Some(("1020400000", "https://eshop.weidmueller.com/de/wdu-16/p/1020400000", "https://assets.dam.weidmueller.com/assets/api/5c783010-f1ec-44e6-b6a8-bd24e8df60bc/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-35" => Some(("1020500000", "https://eshop.weidmueller.com/de/wdu-35/p/1020500000", "https://assets.dam.weidmueller.com/assets/api/04aa8d0b-1ae7-43b1-a6c3-c7ecd78e3136/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdk-25" => Some(("1021500000", "https://eshop.weidmueller.com/de/wdk-2-5/p/1021500000", "https://assets.dam.weidmueller.com/assets/api/e78a7db7-cca5-4f7e-b43e-93c9d20a79ab/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdk-25-n" => Some(("1021580000", "https://eshop.weidmueller.com/de/wdk-2-5-bl/p/1021580000", "https://assets.dam.weidmueller.com/assets/api/e78a7db7-cca5-4f7e-b43e-93c9d20a79ab/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdk-25-pe" => Some(("1036300000", "https://eshop.weidmueller.com/de/wdk-2-5pe/p/1036300000", "https://assets.dam.weidmueller.com/assets/api/e78a7db7-cca5-4f7e-b43e-93c9d20a79ab/DAP_Master_PP,w_800,q_80")),
        "weidmuller-dli-25" => Some(("1783820000", "https://eshop.weidmueller.com/de/dli-2-5/p/1783820000", "https://assets.dam.weidmueller.com/assets/api/986a5bb7-ca57-4e14-9c59-9fc5492655c6/DAP_Master_PP,w_800,q_80")),
        "weidmuller-dli-25-pell" => Some(("1783820000", "https://eshop.weidmueller.com/de/dli-2-5/p/1783820000", "https://assets.dam.weidmueller.com/assets/api/986a5bb7-ca57-4e14-9c59-9fc5492655c6/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wtr-25" => Some(("1855610000", "https://eshop.weidmueller.com/de/wtr-2-5/p/1855610000", "https://assets.dam.weidmueller.com/assets/api/908ccf95-2dca-4a86-91d7-b68d52aa0fde/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wtr-4" => Some(("7910180000", "https://eshop.weidmueller.com/de/wtr-4/p/7910180000", "https://assets.dam.weidmueller.com/assets/api/908ccf95-2dca-4a86-91d7-b68d52aa0fde/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wsi-6" => Some(("1011000000", "https://eshop.weidmueller.com/de/wsi-6/p/1011000000", "https://assets.dam.weidmueller.com/assets/api/b53a71f7-6f24-4252-89ce-b718e0d8cd6b/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zdu-25" => Some(("1608510000", "https://eshop.weidmueller.com/de/zdu-2-5/p/1608510000", "https://assets.dam.weidmueller.com/assets/api/bb83d5f1-0304-4ee6-baf9-aefd6df87aec/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zpe-25" => Some(("1608640000", "https://eshop.weidmueller.com/de/zpe-2-5/p/1608640000", "https://assets.dam.weidmueller.com/assets/api/20dbdc64-2c36-4806-89fe-28cd650bc6b8/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zdk-25" => Some(("1674300000", "https://eshop.weidmueller.com/de/zdk-2-5/p/1674300000", "https://assets.dam.weidmueller.com/assets/api/46401129-1a26-4206-825c-25fc0676e8bd/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zdk-25-n" => Some(("1678630000", "https://eshop.weidmueller.com/de/zdk-2-5-v/p/1678630000", "https://assets.dam.weidmueller.com/assets/api/46401129-1a26-4206-825c-25fc0676e8bd/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zdu-4" => Some(("1632050000", "https://eshop.weidmueller.com/de/zdu-4/p/1632050000", "https://assets.dam.weidmueller.com/assets/api/9e643106-5f17-4374-b941-e9f49e6fa908/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zpe-4" => Some(("1632080000", "https://eshop.weidmueller.com/de/zpe-4/p/1632080000", "https://assets.dam.weidmueller.com/assets/api/20dbdc64-2c36-4806-89fe-28cd650bc6b8/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zdl-25" => Some(("1720810000", "https://eshop.weidmueller.com/de/zdl-2-5-sllpe/p/1720810000", "https://assets.dam.weidmueller.com/assets/api/46401129-1a26-4206-825c-25fc0676e8bd/DAP_Master_PP,w_800,q_80")),
        _ => None,
    }
}

fn phoenix_caas_fallback(spec: &TerminalSpec) -> String {
    let has_pe = spec
        .tiers
        .iter()
        .any(|t| matches!(t.behavior, TierBehavior::ProtectiveEarth));
    let has_blue = spec.tiers.iter().any(|t| t.color == TierColor::BlueNeutral);
    let media_id = if spec.levels >= 3 {
        "123154"
    } else if spec.levels == 2 {
        if has_pe { "235702" } else { "212446" }
    } else if has_pe {
        "121018"
    } else if has_blue {
        "117936"
    } else {
        "231482"
    };
    format!("https://caas.phoenixcontact.com/caas/v1/stable/media/{media_id}/full/b408?format=jpg")
}

fn weidmuller_dam_fallback(spec: &TerminalSpec) -> String {
    let has_pe = spec
        .tiers
        .iter()
        .any(|t| matches!(t.behavior, TierBehavior::ProtectiveEarth));
    let has_blue = spec.tiers.iter().any(|t| t.color == TierColor::BlueNeutral);
    let uuid = if spec.levels >= 3 {
        "e281c782-7fa1-4d4f-b663-47c8fcd43a7a"
    } else if spec.levels == 2 {
        "d2054db7-d5a2-42b1-80ce-c5e41a701c86"
    } else if has_pe {
        "e4f70efa-ed27-42a1-98c6-372ac81cbe30"
    } else if has_blue {
        "80a54677-0c4a-41a5-865f-fbd26c3ae934"
    } else {
        "2d76620f-5915-4c72-afd5-0fffe2de88ab"
    };
    format!("https://assets.dam.weidmueller.com/assets/api/{uuid}/DAP_Master_PP,w_800,q_80")
}

fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' => out.push(b as char),
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

/// In-memory catalog loaded from the embedded JSON terminal database.
#[derive(Debug, Clone)]
pub struct TerminalCatalog {
    specs: Vec<TerminalSpec>,
}

impl Default for TerminalCatalog {
    fn default() -> Self {
        Self::load_embedded()
    }
}

impl TerminalCatalog {
    /// Loads the embedded JSON terminal catalog sorted by manufacturer and then by type name.
    pub fn load_embedded() -> Self {
        let specs: Vec<TerminalSpec> = serde_json::from_str(EMBEDDED_SPECS_JSON)
            .expect("Embedded terminal_specs.json must be valid JSON");
        let mut catalog = Self { specs };
        catalog.sort_by_manufacturer_and_type();
        catalog
    }

    /// Rebuilds the catalog from embedded specs plus project-specific custom specs.
    pub fn reload_with_custom(&mut self, custom_specs: &[TerminalSpec]) {
        *self = Self::load_embedded();
        for spec in custom_specs {
            self.add_custom_spec(spec.clone());
        }
        self.sort_by_manufacturer_and_type();
    }

    /// Adds or replaces a custom terminal specification in the active catalog, keeping sort order.
    pub fn add_custom_spec(&mut self, spec: TerminalSpec) {
        if let Some(existing) = self.specs.iter_mut().find(|s| s.id == spec.id) {
            *existing = spec;
        } else {
            self.specs.push(spec);
        }
        self.sort_by_manufacturer_and_type();
    }

    /// Sorts all specifications alphabetically by manufacturer and naturally by terminal type name.
    pub fn sort_by_manufacturer_and_type(&mut self) {
        self.specs.sort_by(|a, b| {
            a.manufacturer
                .to_lowercase()
                .cmp(&b.manufacturer.to_lowercase())
                .then_with(|| natural_cmp(&a.name, &b.name))
                .then_with(|| a.id.cmp(&b.id))
        });
    }

    /// Returns all available terminal specifications.
    pub fn all(&self) -> &[TerminalSpec] {
        &self.specs
    }

    /// Finds a specification by its unique ID.
    pub fn find_by_id(&self, id: &str) -> Option<&TerminalSpec> {
        self.specs.iter().find(|s| s.id == id)
    }

    /// Returns the distinct manufacturer names in catalog order.
    pub fn manufacturers(&self) -> Vec<&str> {
        let mut out = Vec::new();
        for spec in &self.specs {
            if !out.contains(&spec.manufacturer.as_str()) {
                out.push(spec.manufacturer.as_str());
            }
        }
        out
    }

    /// Resolves matching accessories (end plate, jumpers, markers, end bracket) for `spec`.
    pub fn accessories_for(&self, spec: &TerminalSpec) -> TerminalAccessories {
        spec.accessories
            .clone()
            .unwrap_or_else(|| resolve_accessories(spec))
    }

    /// Applies a new specification to an existing `TerminalBlock`, preserving its `id` and replacing its physical tiers.
    pub fn apply_spec_to_block(block: &mut TerminalBlock, spec: &TerminalSpec) {
        block.spec_id = spec.id.clone();
        block.spec_name = spec.name.clone();
        block.manufacturer = spec.manufacturer.clone();
        block.width_tenths_mm = spec.width_tenths_mm;
        block.numbering_order = spec.numbering_order;
        block.tiers = spec
            .tiers
            .iter()
            .enumerate()
            .map(|(idx, ts)| ConnectionPoint::new(idx, ts.behavior.clone(), ts.color))
            .collect();
    }
}

/// Natural case-insensitive comparison (`PT 2,5` < `PT 4` < `PT 10`, `280` < `2001`).
fn natural_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let mut ia = a.chars().peekable();
    let mut ib = b.chars().peekable();

    loop {
        match (ia.peek().copied(), ib.peek().copied()) {
            (None, None) => return std::cmp::Ordering::Equal,
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (Some(_), None) => return std::cmp::Ordering::Greater,
            (Some(ca), Some(cb)) if ca.is_ascii_digit() && cb.is_ascii_digit() => {
                let mut na = String::new();
                while let Some(&c) = ia.peek() {
                    if c.is_ascii_digit() {
                        na.push(c);
                        ia.next();
                    } else {
                        break;
                    }
                }
                let mut nb = String::new();
                while let Some(&c) = ib.peek() {
                    if c.is_ascii_digit() {
                        nb.push(c);
                        ib.next();
                    } else {
                        break;
                    }
                }
                let va: u64 = na.parse().unwrap_or(0);
                let vb: u64 = nb.parse().unwrap_or(0);
                let ord = va.cmp(&vb).then_with(|| na.len().cmp(&nb.len()));
                if ord != std::cmp::Ordering::Equal {
                    return ord;
                }
            }
            (Some(_), Some(_)) => {
                let ca = ia.next().unwrap().to_ascii_lowercase();
                let cb = ib.next().unwrap().to_ascii_lowercase();
                let ord = ca.cmp(&cb);
                if ord != std::cmp::Ordering::Equal {
                    return ord;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_is_sorted_by_manufacturer_then_natural_type() {
        let catalog = TerminalCatalog::load_embedded();
        let mut prev_mfg = "";
        for w in catalog.all().windows(2) {
            let a = &w[0];
            let b = &w[1];
            let ma = a.manufacturer.to_lowercase();
            let mb = b.manufacturer.to_lowercase();
            assert!(
                ma <= mb,
                "Manufacturer out of order: {} vs {}",
                a.manufacturer,
                b.manufacturer
            );
            if ma == mb {
                assert!(
                    natural_cmp(&a.name, &b.name) != std::cmp::Ordering::Greater,
                    "Type out of order in {}: '{}' before '{}'",
                    a.manufacturer,
                    a.name,
                    b.name
                );
            }
            prev_mfg = &a.manufacturer;
        }
        assert!(!prev_mfg.is_empty());
    }

    #[test]
    fn every_phoenix_and_weidmuller_spec_has_direct_product_and_image_url() {
        let catalog = TerminalCatalog::load_embedded();
        for spec in catalog.all() {
            if spec.manufacturer == "Phoenix Contact" || spec.manufacturer == "Weidmüller" {
                let art = spec.effective_article_number();
                assert!(
                    art.is_some_and(|a| !a.is_empty()),
                    "Missing article number for {}",
                    spec.id
                );
                let prod_url = spec.effective_product_url();
                assert!(
                    !prod_url.contains("/suche?") && !prod_url.contains("/search/"),
                    "Product URL for {} fell back to search overview: {}",
                    spec.id,
                    prod_url
                );
                for bad in [
                    "dreistockklemme-",
                    "messertrennklemme-",
                    "trenngrundklemme-",
                    "initiatoraektorklemme-",
                    "installationsschutzleiterklemme-",
                    "sicherungsreihenklemme-",
                    "messwandlerreihenklemme-",
                    "3211755",
                ] {
                    assert!(
                        !prod_url.contains(bad),
                        "Invalid URL slug '{bad}' in {}: {prod_url}",
                        spec.id
                    );
                }
                let img_url = spec.effective_image_url();
                assert!(
                    img_url.as_deref().is_some_and(|u| {
                        u.starts_with("https://caas.phoenixcontact.com/")
                            || u.starts_with("https://assets.dam.weidmueller.com/")
                    }),
                    "Missing direct CDN image URL for {}: {:?}",
                    spec.id,
                    img_url
                );
            }
        }

        let pt15_3l = catalog.find_by_id("phoenix-pt-15s-3l").unwrap();
        assert_eq!(
            pt15_3l.effective_product_url(),
            "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pt-15s-3l-3213713"
        );
        assert_eq!(
            catalog.accessories_for(pt15_3l).end_plate,
            "D-PT 1,5/S-3L (3113771)"
        );
    }

    #[test]
    fn all_specs_have_realistic_bauhoehe_height_tenths_mm() {
        let catalog = TerminalCatalog::load_embedded();
        for spec in catalog.all() {
            let h = spec.effective_height_tenths_mm();
            assert!(
                (420..=1200).contains(&h),
                "Unrealistic front-view Bauhöhe {} for {} ({})",
                h,
                spec.name,
                spec.id
            );
        }
        assert_eq!(
            catalog.find_by_id("wago-2002-1201").unwrap().effective_height_tenths_mm(),
            485
        );
        assert_eq!(
            catalog.find_by_id("wago-2002-2201").unwrap().effective_height_tenths_mm(),
            697
        );
        assert_eq!(
            catalog.find_by_id("wago-2003-7641").unwrap().effective_height_tenths_mm(),
            960
        );
        assert_eq!(
            catalog.find_by_id("phoenix-pt-25").unwrap().effective_height_tenths_mm(),
            486
        );
        assert_eq!(
            catalog.find_by_id("weidmuller-a2c-25").unwrap().effective_height_tenths_mm(),
            550
        );
    }

    #[test]
    fn rail_offsets_distinguish_symmetric_and_asymmetric_installation_blocks() {
        let catalog = TerminalCatalog::load_embedded();
        assert_eq!(
            catalog.find_by_id("phoenix-pt-25").unwrap().effective_rail_offset_tenths_mm(),
            0
        );
        assert_eq!(
            catalog.find_by_id("phoenix-pt-25-quattro").unwrap().effective_rail_offset_tenths_mm(),
            0
        );
        assert_eq!(
            catalog.find_by_id("phoenix-ptpower-50").unwrap().effective_rail_offset_tenths_mm(),
            0
        );
        assert_eq!(
            catalog.find_by_id("wago-2002-1201").unwrap().effective_rail_offset_tenths_mm(),
            0
        );
        assert_eq!(
            catalog.find_by_id("phoenix-pti-25-ln").unwrap().effective_rail_offset_tenths_mm(),
            130
        );
        assert_eq!(
            catalog.find_by_id("wago-2003-7641").unwrap().effective_rail_offset_tenths_mm(),
            125
        );
        assert_eq!(
            catalog.find_by_id("weidmuller-aitb-25-nlp").unwrap().effective_rail_offset_tenths_mm(),
            130
        );
    }
}
