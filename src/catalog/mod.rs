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
    /// True when this specification was manually edited/overridden by the user at runtime.
    #[serde(default, skip_serializing_if = "is_false")]
    pub user_modified: bool,
}

fn is_false(value: &bool) -> bool {
    !*value
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

    /// Number of jumper/bridge chambers per tier (`0` = no comb jumper, `1` = single chamber, `2` = dual chamber).
    pub fn effective_jumper_slots(&self) -> u8 {
        if let Some(slots) = self.jumper_slots_per_tier {
            return slots.min(2);
        }
        let id = self.id.as_str();
        if id.contains("uslkg")
            || id.contains("ptrv")
            || id.contains("wnt-")
            || id.contains("ptn-")
            || id.contains("utn-")
            || id.contains("stn-")
        {
            return 0;
        }
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
            || id.contains("2006-161")
            || id.contains("2002-166")
            || id.contains("2006-168")
            || id.contains("-198")
            || id.contains("-260")
            || id.contains("-296")
            || id.contains("15s")
            || id.contains("pti-")
            || id.contains("ptio-")
            || id.contains("sti-")
            || id.contains("uti-")
            || id.contains("ptpower")
            || id.contains("ptfix")
            || id.contains("pt-10")
            || id.contains("pt-16")
            || id.contains("pt-35")
            || id.contains("ptme")
            || id.contains("-hesi")
            || id.contains("-mt")
            || id.contains("-tg")
            || id.contains("uk-")
            || id.contains("ukk-")
            || id.contains("udmtk")
            || id.contains("a2c-15")
            || id.contains("a3c-15")
            || id.contains("a4c-15")
            || id.contains("a2c-10")
            || id.contains("a2c-16")
            || id.contains("a2c-35")
            || id.contains("aio")
            || id.contains("aitb")
            || id.contains("dli")
            || id.contains("dld")
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

    /// Maximum consecutive poles offered in standard comb jumper accessories for this block (`0` if non-bridgeable).
    pub fn effective_max_jumper_poles(&self) -> u8 {
        if let Some(poles) = self.max_jumper_poles {
            return if poles == 0 { 0 } else { poles.max(2) };
        }
        let id = self.id.as_str();
        if id.contains("uslkg")
            || id.contains("ptrv")
            || id.contains("wnt-")
            || id.contains("ptn-")
            || id.contains("utn-")
            || id.contains("stn-")
        {
            0
        } else if id.contains("285-")
            || id.contains("2035-")
            || id.contains("ptpower")
            || id.contains("ptfix")
            || id.contains("2010-")
            || id.contains("2016-")
            || id.contains("2110-")
            || id.contains("2116-")
            || id.contains("2210-")
            || id.contains("2216-")
            || id.contains("pt-10")
            || id.contains("pt-16")
            || id.contains("pt-35")
            || id.contains("ptv-10")
            || id.contains("ptv-16")
            || id.contains("st-10")
            || id.contains("st-16")
            || id.contains("ut-10")
            || id.contains("ut-16")
            || id.contains("ut-35")
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
            || id.contains("ptv-6")
            || id.contains("st-6")
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
                if id.contains("2035-") {
                    return format!("2035-402 ({poles}-fach)");
                }
                if id.starts_with("wago-285") {
                    return format!("285-435 / 285-450 / 285-495 ({poles}-fach)");
                }
                let prefix = if id.contains("2000") || id.contains("2200") {
                    "2000"
                } else if id.contains("2001") || id.contains("2201") {
                    "2001"
                } else if id.contains("2004")
                    || id.contains("2005")
                    || id.contains("2104")
                    || id.contains("2204")
                    || id.contains("2205")
                    || self.width_tenths_mm == 62
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
        if self.user_modified {
            return self
                .article_number
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty());
        }
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
        if self.user_modified
            && let Some(url) = &self.product_url
            && !url.trim().is_empty()
        {
            return url.trim().to_string();
        }
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
                format!("https://www.wago.com/de/p/{clean_art}")
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
        if self.user_modified
            && let Some(url) = &self.image_url
            && !url.trim().is_empty()
        {
            return Some(url.trim().to_string());
        }
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
                    if id.contains("-1185") {
                        1300
                    } else if id.contains("-134") || id.contains("-135") || id.contains("-137") {
                        860
                    } else {
                        1070 // 285-150 / 285-195 (50-95 mm² POWER)
                    }
                } else if id.starts_with("wago-2035") {
                    1010
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
                    697
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
            || (id.contains("aitb") && !id.contains("-bb"))
        {
            130
        } else if id.contains("wago-2003") || id.contains("wago-2203") || id.contains("-7714") {
            125
        } else if id.contains("zdl") {
            120
        } else if id.contains("dli") || id.contains("dld") {
            60
        } else if id.contains("ptio") || id.contains("aio") || id.contains("280-560") {
            80
        } else if id.contains("-hesi")
            || id.contains("wsi")
            || id.contains("afs-4")
            || id.contains("2006-161")
            || id.contains("2006-168")
            || id.contains("2002-161")
            || id.contains("2202-161")
            || id.contains("2002-181")
            || id.contains("281-611")
            || id.contains("281-623")
        {
            45
        } else {
            0
        }
    }

    /// Returns `true` if this specification is a DIN-rail N-busbar carrier / support block
    /// (`Sammelschienenträger` / `Auflagebock`), such as WAGO `2009-305` / `2009-304`,
    /// Phoenix Contact `AB-*`, or Weidmüller `AEB ITB*`.
    pub fn is_busbar_carrier(&self) -> bool {
        let id = self.id.as_str();
        id.starts_with("wago-2009-30")
            || id.starts_with("phoenix-ab-")
            || id.starts_with("weidmuller-aeb-itb-")
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
        if self.is_n_supply_terminal() || self.is_busbar_carrier() {
            return false;
        }
        let id = self.id.as_str();
        self.name.contains("NT")
            || self.name.contains("NDT")
            || id.starts_with("phoenix-ptn-")
            || id.starts_with("phoenix-utn-")
            || id.starts_with("phoenix-stn-")
            || id.ends_with("-lnt")
            || id.ends_with("-pelnt")
            || id.ends_with("-ntlp")
            || id.ends_with("-7640")
            || id.ends_with("-7641")
            || id.ends_with("-6640")
            || id.ends_with("-6641")
    }

    /// Returns `true` if this terminal is a building-installation terminal (`2003`, `2203`, `2005`, `2205`,
    /// `PTI`, `PTN`, `STI`, `STN`, `UTI`, `UTN`, `AITB`, `ZDL`, `DLI`) or any `NT` terminal that should prompt for NT / N-Einspeiseklemme.
    pub fn is_installation_or_nt_terminal(&self) -> bool {
        if self.is_n_supply_terminal() || self.is_busbar_carrier() {
            return false;
        }
        let id = self.id.as_str();
        id.starts_with("wago-2003-")
            || id.starts_with("wago-2203-")
            || id.starts_with("wago-2005-")
            || id.starts_with("wago-2205-")
            || id.starts_with("phoenix-pti-")
            || id.starts_with("phoenix-ptn-")
            || id.starts_with("phoenix-sti-")
            || id.starts_with("phoenix-stn-")
            || id.starts_with("phoenix-uti-")
            || id.starts_with("phoenix-utn-")
            || id.starts_with("weidmuller-aitb-")
            || id.starts_with("weidmuller-zdl-")
            || id.starts_with("weidmuller-dli-")
            || self.is_nt_disconnect_terminal()
    }

    /// If this terminal is an installation terminal without `NT` (e.g. `PE/L/N`, `PE/L/L`, `L/L`, `L/N` in
    /// WAGO `2003`/`2203`/`2005`/`2205`, Phoenix `PTI`, or Weidmüller `AITB`), returns the matching `NT`
    /// specification ID so the user can switch to the `NT` variant directly in the prompt dialog.
    pub fn matching_nt_spec_id(&self) -> Option<&'static str> {
        if self.is_nt_disconnect_terminal()
            || self.is_n_supply_terminal()
            || self.is_busbar_carrier()
            || !self.has_neutral_tier()
        {
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
        } else if id == "phoenix-pti-25-bu" {
            Some("phoenix-ptn-25")
        } else if id.starts_with("phoenix-pti-25-") {
            if self.levels >= 3 {
                Some("phoenix-pti-25-pelnt")
            } else if self.levels == 2 {
                Some("phoenix-pti-25-lnt")
            } else {
                None
            }
        } else if id == "phoenix-pti-4-bu" {
            Some("phoenix-ptn-4")
        } else if id.starts_with("phoenix-pti-4-") {
            if self.levels >= 2 {
                Some("phoenix-pti-4-pelnt")
            } else {
                None
            }
        } else if id == "phoenix-pti-6-bu" {
            Some("phoenix-ptn-6")
        } else if id == "phoenix-pti-16s-bu" {
            Some("phoenix-ptn-16s")
        } else if id.starts_with("weidmuller-aitb-25-") {
            if self.levels >= 3 {
                Some("weidmuller-aitb-25-ntlp")
            } else if self.levels == 2 {
                Some("weidmuller-aitb-25-ndt")
            } else {
                None
            }
        } else if id.starts_with("weidmuller-aitb-4-") {
            if self.levels >= 3 {
                Some("weidmuller-aitb-4-ntlp")
            } else if self.levels == 2 {
                Some("weidmuller-aitb-4-ndt")
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns `true` if at least one tier of this terminal carries an `N` (Neutral) potential.
    pub fn has_neutral_tier(&self) -> bool {
        self.tiers.iter().any(|t| t.behavior.is_neutral())
    }

    /// Maps an `N` terminal (installation N/PE/L/N or single-tier N) to its closest matching `NT`
    /// disconnect counterpart within the same manufacturer & connection technology, or `None` if already `NT`
    /// or no compatible `NT` terminal exists in the catalog.
    pub fn matching_nt_equivalent_id(&self) -> Option<&'static str> {
        if self.is_nt_disconnect_terminal()
            || self.is_n_supply_terminal()
            || self.is_busbar_carrier()
            || !self.has_neutral_tier()
        {
            return None;
        }
        match self.id.as_str() {
            // WAGO 2003 / 2203 / 2005 / 2205 (installation N <-> NT preserving conductor levels)
            "wago-2003-7646" => Some("wago-2003-7641"),
            "wago-2003-7649" => Some("wago-2003-7640"),
            "wago-2003-6646" => Some("wago-2003-6640"),
            "wago-2203-7646" => Some("wago-2203-7641"),
            "wago-2203-7649" => Some("wago-2203-7640"),
            "wago-2203-6646" => Some("wago-2203-6640"),
            "wago-2005-7646" => Some("wago-2005-7641"),
            "wago-2005-7649" => Some("wago-2005-7640"),
            "wago-2205-7646" => Some("wago-2205-7641"),
            "wago-2205-7649" => Some("wago-2205-7640"),

            // WAGO 2002 / 2202 / 2004 / 2204 N -> nearest TOPJOB S NT equivalent
            "wago-2002-3209" => Some("wago-2003-7641"),
            "wago-2002-2204" | "wago-2002-1204" | "wago-2002-1304" | "wago-2002-1404" => {
                Some("wago-2003-7640")
            }
            "wago-2202-2204" | "wago-2202-1204" | "wago-2202-1304" | "wago-2202-1404" => {
                Some("wago-2203-7640")
            }
            "wago-2004-1204" | "wago-2004-1304" | "wago-2004-1404" => Some("wago-2005-7640"),
            "wago-2204-1204" | "wago-2204-1304" | "wago-2204-1404" => Some("wago-2205-7640"),

            // Phoenix Contact PTI / PTN / PT / ST / STN / UT / UTN N -> NT
            "phoenix-pti-25-peln" | "phoenix-pt-25-peln" => Some("phoenix-pti-25-pelnt"),
            "phoenix-pti-25-ln" | "phoenix-pttb-25-ln-real" => Some("phoenix-pti-25-lnt"),
            "phoenix-pti-25-bu"
            | "phoenix-pt-25-bu"
            | "phoenix-pt-25-twin-bu"
            | "phoenix-pt-25-quattro-bu" => Some("phoenix-ptn-25"),
            "phoenix-pti-4-peln" | "phoenix-pti-4-ln" => Some("phoenix-pti-4-pelnt"),
            "phoenix-pti-4-bu"
            | "phoenix-pt-4-bu"
            | "phoenix-pt-4-twin-bu"
            | "phoenix-pt-4-quattro-bu" => Some("phoenix-ptn-4"),
            "phoenix-pti-6-bu"
            | "phoenix-pt-6-bu"
            | "phoenix-pt-6-twin-bu"
            | "phoenix-pt-6-quattro-bu" => Some("phoenix-ptn-6"),
            "phoenix-pti-16s-bu" | "phoenix-pt-16n-bu" | "phoenix-pt-16-twin-n-bu" => {
                Some("phoenix-ptn-16s")
            }
            "phoenix-st-25-bu" | "phoenix-st-25-twin-bu" | "phoenix-st-25-quattro-bu" => {
                Some("phoenix-stn-25")
            }
            "phoenix-st-4-bu" => Some("phoenix-stn-4"),
            "phoenix-ut-25-bu" | "phoenix-ut-25-twin-bu" | "phoenix-ut-25-quattro-bu" => {
                Some("phoenix-utn-25")
            }
            "phoenix-ut-4-bu" | "phoenix-ut-4-twin-bu" => Some("phoenix-utn-4"),
            "phoenix-ut-6-bu" => Some("phoenix-utn-6"),
            "phoenix-ut-10-bu" => Some("phoenix-utn-10"),
            "phoenix-ut-16-bu" => Some("phoenix-utn-16"),
            "phoenix-ut-35-bu" => Some("phoenix-utn-35"),

            // Weidmüller AITB / A-Series / W-Series / Z-Series N -> NT
            "weidmuller-aitb-25-nlp" | "weidmuller-a3t-25-peln" => Some("weidmuller-aitb-25-ntlp"),
            "weidmuller-aitb-25-nl"
            | "weidmuller-aitb-25-bb-bl"
            | "weidmuller-a2c-25-bl"
            | "weidmuller-a3c-25-bl"
            | "weidmuller-a4c-25-bl" => Some("weidmuller-aitb-25-ndt"),
            "weidmuller-aitb-4-nlp" => Some("weidmuller-aitb-4-ntlp"),
            "weidmuller-aitb-4-nl"
            | "weidmuller-aitb-4-bb-bl"
            | "weidmuller-a2c-4-bl"
            | "weidmuller-a3c-4-bl"
            | "weidmuller-a4c-4-bl" => Some("weidmuller-aitb-4-ndt"),
            "weidmuller-wdu-10-bl" => Some("weidmuller-wnt-10"),
            "weidmuller-wdu-16-bl" => Some("weidmuller-wnt-16n"),
            "weidmuller-zdu-25-bl" | "weidmuller-zdu-25-3an-bl" | "weidmuller-zdu-25-4an-bl" => {
                Some("weidmuller-zdl-25")
            }
            _ => None,
        }
    }

    /// Maps an `NT` disconnect terminal back to its matching standard `N` counterpart with the same
    /// conductor count (`PE/L/NT -> PE/L/N`, `L/NT -> L/N`, `PTN -> PTI BU`, `UTN -> UT BU`, `STN -> ST BU`, `WNT -> WDU BL`), preserving all L/PE tiers.
    pub fn matching_n_from_nt_id(&self) -> Option<&'static str> {
        if !self.is_nt_disconnect_terminal() {
            return None;
        }
        match self.id.as_str() {
            "wago-2003-7641" => Some("wago-2003-7646"),
            "wago-2003-7640" => Some("wago-2003-7649"),
            "wago-2003-6640" | "wago-2003-6641" => Some("wago-2003-6646"),
            "wago-2203-7641" => Some("wago-2203-7646"),
            "wago-2203-7640" => Some("wago-2203-7649"),
            "wago-2203-6640" | "wago-2203-6641" => Some("wago-2203-6646"),
            "wago-2005-7641" => Some("wago-2005-7646"),
            "wago-2005-7640" => Some("wago-2005-7649"),
            "wago-2205-7641" => Some("wago-2205-7646"),
            "wago-2205-7640" => Some("wago-2205-7649"),
            "phoenix-pti-25-pelnt" => Some("phoenix-pti-25-peln"),
            "phoenix-pti-25-lnt" => Some("phoenix-pti-25-ln"),
            "phoenix-pti-4-pelnt" => Some("phoenix-pti-4-peln"),
            "phoenix-ptn-25" => Some("phoenix-pti-25-bu"),
            "phoenix-ptn-4" => Some("phoenix-pti-4-bu"),
            "phoenix-ptn-6" => Some("phoenix-pti-6-bu"),
            "phoenix-ptn-16s" => Some("phoenix-pti-16s-bu"),
            "phoenix-utn-25" => Some("phoenix-ut-25-bu"),
            "phoenix-utn-4" => Some("phoenix-ut-4-bu"),
            "phoenix-utn-6" => Some("phoenix-ut-6-bu"),
            "phoenix-utn-10" => Some("phoenix-ut-10-bu"),
            "phoenix-utn-16" => Some("phoenix-ut-16-bu"),
            "phoenix-utn-35" => Some("phoenix-ut-35-bu"),
            "phoenix-stn-25" => Some("phoenix-st-25-bu"),
            "phoenix-stn-4" => Some("phoenix-st-4-bu"),
            "phoenix-stn-10" => Some("phoenix-ptv-10-bu"),
            "phoenix-stn-16" => Some("phoenix-ptv-16-bu"),
            "weidmuller-aitb-25-ntlp" => Some("weidmuller-aitb-25-nlp"),
            "weidmuller-aitb-25-ndt" => Some("weidmuller-aitb-25-nl"),
            "weidmuller-aitb-4-ntlp" => Some("weidmuller-aitb-4-nlp"),
            "weidmuller-aitb-4-ndt" => Some("weidmuller-aitb-4-nl"),
            "weidmuller-wnt-10" => Some("weidmuller-wdu-10-bl"),
            "weidmuller-wnt-16n" => Some("weidmuller-wdu-16-bl"),
            _ => None,
        }
    }

    /// Logical terminal series/profile key used to group compatible terminal blocks together in dropdowns.
    pub fn compatibility_family(&self) -> String {
        let id = self.id.as_str();
        match self.manufacturer.as_str() {
            "WAGO" => {
                if id.starts_with("wago-2000-") {
                    "wago:2000".to_string()
                } else if id.starts_with("wago-2001-") {
                    "wago:2001".to_string()
                } else if id.starts_with("wago-2002-") {
                    "wago:2002".to_string()
                } else if id.starts_with("wago-2003-") {
                    "wago:2003".to_string()
                } else if id.starts_with("wago-2004-") {
                    "wago:2004".to_string()
                } else if id.starts_with("wago-2005-") {
                    "wago:2005".to_string()
                } else if id.starts_with("wago-2006-") {
                    "wago:2006".to_string()
                } else if id.starts_with("wago-2007-") {
                    "wago:2007".to_string()
                } else if id.starts_with("wago-2009-") {
                    "wago:2009".to_string()
                } else if id.starts_with("wago-2010-") {
                    "wago:2010".to_string()
                } else if id.starts_with("wago-2016-") {
                    "wago:2016".to_string()
                } else if id.starts_with("wago-2022-") {
                    "wago:2022".to_string()
                } else if id.starts_with("wago-2035-") {
                    "wago:2035".to_string()
                } else if id.starts_with("wago-2102-") {
                    "wago:2102".to_string()
                } else if id.starts_with("wago-2104-") {
                    "wago:2104".to_string()
                } else if id.starts_with("wago-2106-") {
                    "wago:2106".to_string()
                } else if id.starts_with("wago-2110-") {
                    "wago:2110".to_string()
                } else if id.starts_with("wago-2116-") {
                    "wago:2116".to_string()
                } else if id.starts_with("wago-2200-") {
                    "wago:2200".to_string()
                } else if id.starts_with("wago-2201-") {
                    "wago:2201".to_string()
                } else if id.starts_with("wago-2202-") {
                    "wago:2202".to_string()
                } else if id.starts_with("wago-2203-") {
                    "wago:2203".to_string()
                } else if id.starts_with("wago-2204-") {
                    "wago:2204".to_string()
                } else if id.starts_with("wago-2205-") {
                    "wago:2205".to_string()
                } else if id.starts_with("wago-2206-") {
                    "wago:2206".to_string()
                } else if id.starts_with("wago-2210-") {
                    "wago:2210".to_string()
                } else if id.starts_with("wago-2216-") {
                    "wago:2216".to_string()
                } else if id.starts_with("wago-280-") {
                    "wago:280".to_string()
                } else if id.starts_with("wago-281-") {
                    "wago:281".to_string()
                } else if id.starts_with("wago-285-") {
                    "wago:285".to_string()
                } else {
                    format!("wago:{}", self.series)
                }
            }
            "Phoenix Contact" => {
                if id.starts_with("phoenix-pt-15s") || id.starts_with("phoenix-pttb-15s") {
                    "phx:pt-1.5s".to_string()
                } else if id.starts_with("phoenix-pt-25")
                    || id.starts_with("phoenix-pttb-25")
                    || id.starts_with("phoenix-pttbs-25")
                {
                    "phx:pt-2.5".to_string()
                } else if id.starts_with("phoenix-pt-4") || id.starts_with("phoenix-pttb-4") {
                    "phx:pt-4".to_string()
                } else if id.starts_with("phoenix-pt-6") || id.starts_with("phoenix-ptme") {
                    "phx:pt-6".to_string()
                } else if id.starts_with("phoenix-pt-10") {
                    "phx:pt-10".to_string()
                } else if id.starts_with("phoenix-pt-16") {
                    "phx:pt-16".to_string()
                } else if id.starts_with("phoenix-ptv-25") || id.starts_with("phoenix-ptvtb-25") {
                    "phx:ptv-2.5".to_string()
                } else if id.starts_with("phoenix-ptv-4") || id.starts_with("phoenix-ptvtb-4") {
                    "phx:ptv-4".to_string()
                } else if id.starts_with("phoenix-ptv-6") {
                    "phx:ptv-6".to_string()
                } else if id.starts_with("phoenix-ptv-10") {
                    "phx:ptv-10".to_string()
                } else if id.starts_with("phoenix-ptv-16") {
                    "phx:ptv-16".to_string()
                } else if id.starts_with("phoenix-pti-25")
                    || id.starts_with("phoenix-ptn-25")
                    || id == "phoenix-ab-pti"
                    || id == "phoenix-ab-pti-3"
                {
                    "phx:pti-2.5".to_string()
                } else if id.starts_with("phoenix-pti-4")
                    || id.starts_with("phoenix-ptn-4")
                    || id == "phoenix-ab-pti-4-3"
                {
                    "phx:pti-4".to_string()
                } else if id.starts_with("phoenix-pti-6")
                    || id.starts_with("phoenix-ptn-6")
                    || id == "phoenix-ab-pti-6"
                {
                    "phx:pti-6".to_string()
                } else if id.starts_with("phoenix-pti-16")
                    || id.starts_with("phoenix-ptn-16")
                    || id.starts_with("phoenix-ab-pti-16")
                {
                    "phx:pti-16".to_string()
                } else if id.starts_with("phoenix-sti-") || id.starts_with("phoenix-ab-sti") {
                    "phx:sti".to_string()
                } else if id.starts_with("phoenix-uti-") || id.starts_with("phoenix-ab-uti") {
                    "phx:uti".to_string()
                } else if id.starts_with("phoenix-st-25")
                    || id.starts_with("phoenix-stn-25")
                    || id.starts_with("phoenix-sttb-25")
                {
                    "phx:st-2.5".to_string()
                } else if id.starts_with("phoenix-st-4") || id.starts_with("phoenix-stn-4") {
                    "phx:st-4".to_string()
                } else if id.starts_with("phoenix-st-6") {
                    "phx:st-6".to_string()
                } else if id.starts_with("phoenix-st-10") || id.starts_with("phoenix-stn-10") {
                    "phx:st-10".to_string()
                } else if id.starts_with("phoenix-st-16") || id.starts_with("phoenix-stn-16") {
                    "phx:st-16".to_string()
                } else if id.starts_with("phoenix-ut-25")
                    || id.starts_with("phoenix-utn-25")
                    || id.starts_with("phoenix-uttb-25")
                {
                    "phx:ut-2.5".to_string()
                } else if id.starts_with("phoenix-ut-4")
                    || id.starts_with("phoenix-utn-4")
                    || id.starts_with("phoenix-uttb-4")
                {
                    "phx:ut-4".to_string()
                } else if id.starts_with("phoenix-ut-6") || id.starts_with("phoenix-utn-6") {
                    "phx:ut-6".to_string()
                } else if id.starts_with("phoenix-ut-10") || id.starts_with("phoenix-utn-10") {
                    "phx:ut-10".to_string()
                } else if id.starts_with("phoenix-ut-16") || id.starts_with("phoenix-utn-16") {
                    "phx:ut-16".to_string()
                } else if id.starts_with("phoenix-ut-35") || id.starts_with("phoenix-utn-35") {
                    "phx:ut-35".to_string()
                } else if id.starts_with("phoenix-uk-")
                    || id.starts_with("phoenix-uslkg")
                    || id.starts_with("phoenix-udmtk")
                    || id.starts_with("phoenix-ukk")
                {
                    "phx:uk".to_string()
                } else if id.starts_with("phoenix-ptpower") {
                    "phx:ptpower".to_string()
                } else if id.starts_with("phoenix-ptfix") {
                    "phx:ptfix".to_string()
                } else if id.starts_with("phoenix-ptrv") {
                    "phx:ptrv".to_string()
                } else if id.starts_with("phoenix-ptio") {
                    "phx:ptio".to_string()
                } else {
                    format!("phx:{}", self.series)
                }
            }
            "Weidmüller" => {
                if id.contains("a2c-15")
                    || id.contains("a3c-15")
                    || id.contains("a4c-15")
                    || id.contains("a2t-15")
                    || id.contains("a3t-15")
                {
                    "wm:a-1.5".to_string()
                } else if id.contains("a2c-25")
                    || id.contains("a3c-25")
                    || id.contains("a4c-25")
                    || id.contains("a2t-25")
                    || id.contains("a3t-25")
                    || id.contains("amc-25")
                    || id.contains("adt-25")
                {
                    "wm:a-2.5".to_string()
                } else if id.contains("a2c-4")
                    || id.contains("a3c-4")
                    || id.contains("a4c-4")
                    || id.contains("a2t-4")
                    || id.contains("amc-4")
                    || id.contains("adt-4")
                    || id.contains("afs-4")
                {
                    "wm:a-4".to_string()
                } else if id.contains("a2c-6") || id.contains("a3c-6") {
                    "wm:a-6".to_string()
                } else if id.contains("a2c-10") || id.contains("a3c-10") {
                    "wm:a-10".to_string()
                } else if id.contains("a2c-16") || id.contains("a3c-16") {
                    "wm:a-16".to_string()
                } else if id.contains("a2c-35") {
                    "wm:a-35".to_string()
                } else if id.contains("a2c-50-70") {
                    "wm:a-70".to_string()
                } else if id.contains("a2c-95-120") {
                    "wm:a-120".to_string()
                } else if id.contains("aio") {
                    "wm:aio".to_string()
                } else if id.starts_with("weidmuller-aitb-25") {
                    "wm:aitb-2.5".to_string()
                } else if id.starts_with("weidmuller-aitb-4") {
                    "wm:aitb-4".to_string()
                } else if id.starts_with("weidmuller-aitb-6") {
                    "wm:aitb-6".to_string()
                } else if id.starts_with("weidmuller-aitb-10") {
                    "wm:aitb-10".to_string()
                } else if id.starts_with("weidmuller-aitb-16") {
                    "wm:aitb-16".to_string()
                } else if id.starts_with("weidmuller-aeb-itb") {
                    "wm:aitb-acc".to_string()
                } else if id.contains("wdu-15")
                    || id.contains("wpe-15")
                    || id.contains("wdu-25")
                    || id.contains("wpe-25")
                    || id.contains("wtr-25")
                    || id.contains("wdk-25")
                    || id.contains("dli-25")
                    || id.contains("dld-25")
                {
                    "wm:w-2.5".to_string()
                } else if id.contains("wdu-4")
                    || id.contains("wpe-4")
                    || id.contains("wtr-4")
                    || id.contains("wdk-4n")
                {
                    "wm:w-4".to_string()
                } else if id.contains("wdu-6")
                    || id.contains("wpe-6")
                    || id.contains("wsi-6")
                    || id.contains("wtl-6")
                    || id.contains("wtd-6")
                {
                    "wm:w-6".to_string()
                } else if id.contains("wdu-10")
                    || id.contains("wpe-10")
                    || id.contains("wnt-10")
                {
                    "wm:w-10".to_string()
                } else if id.contains("wdu-16")
                    || id.contains("wpe-16")
                    || id.contains("wnt-16")
                {
                    "wm:w-16".to_string()
                } else if id.contains("wdu-35") || id.contains("wpe-35") {
                    "wm:w-35".to_string()
                } else if id.contains("50n") || id.contains("70n") {
                    "wm:w-50-70".to_string()
                } else if id.contains("zdu-15") || id.contains("zpe-15") {
                    "wm:z-1.5".to_string()
                } else if id.contains("zdu-25")
                    || id.contains("zpe-25")
                    || id.contains("zdk-25")
                    || id.contains("zdl-25")
                    || id.contains("ztr-25")
                {
                    "wm:z-2.5".to_string()
                } else if id.contains("zdu-4") || id.contains("zpe-4") {
                    "wm:z-4".to_string()
                } else if id.contains("zdu-6") || id.contains("zpe-6") {
                    "wm:z-6".to_string()
                } else if id.contains("zdu-10") {
                    "wm:z-10".to_string()
                } else if id.contains("zdu-16") {
                    "wm:z-16".to_string()
                } else {
                    format!("wm:{}", self.series)
                }
            }
            other => format!("{other}:{}", self.series),
        }
    }

    fn connection_subbucket(&self) -> u8 {
        let id = self.id.as_str();
        if id.contains("-quattro")
            || id.contains("-4an")
            || id.contains("-4c")
            || id.contains("-140")
            || id.contains("a4c-")
        {
            3 // 4-conductor
        } else if id.contains("-twin")
            || id.contains("-3an")
            || id.contains("-3c")
            || id.contains("-130")
            || id.contains("a3c-")
        {
            2 // 3-conductor
        } else if id.contains("-120") || id.contains("a2c-") || self.levels == 1 {
            1 // 2-conductor standard
        } else {
            0
        }
    }

    /// Computes how compatible `self` is as a replacement/companion for `reference` in dropdowns.
    /// Lower tuple values sort first (`0` = exact same type, `1` = same family & levels, `2` = same family, ...).
    pub fn compatibility_rank(&self, reference: &TerminalSpec) -> (u8, u8) {
        if self.id == reference.id {
            return (0, 0);
        }
        let same_fam = self.compatibility_family() == reference.compatibility_family();
        let sub_diff = u8::from(self.connection_subbucket() != reference.connection_subbucket());
        let same_mfg = self.manufacturer == reference.manufacturer;
        let same_levels = self.levels == reference.levels;
        let same_width = self.width_tenths_mm == reference.width_tenths_mm;

        if same_fam && same_levels {
            return (1, sub_diff);
        }
        if same_fam {
            return (2, sub_diff);
        }
        if (reference.is_installation_or_nt_terminal() || reference.is_n_supply_terminal())
            && same_mfg
            && (self.is_installation_or_nt_terminal()
                || self.is_n_supply_terminal()
                || self.is_busbar_carrier())
            && same_width
        {
            return (3, u8::from(!same_levels));
        }
        if self.jumper_family_key() == reference.jumper_family_key() && same_levels {
            return (3, sub_diff);
        }
        if same_mfg && same_levels && same_width {
            return (4, sub_diff);
        }
        if same_mfg && same_levels {
            return (5, sub_diff);
        }
        if same_mfg {
            return (6, sub_diff);
        }
        if same_levels && same_width {
            return (7, sub_diff);
        }
        (8, 0)
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
                "phoenix-ptn-16s",
            ],
            "Weidmüller" => &[
                "weidmuller-aitb-16-ndt-bl",
                "weidmuller-aitb-16-nlo-bl",
            ],
            "WAGO" => &[
                "wago-2016-7714",
                "wago-2216-7714",
                "wago-2016-7614",
            ],
            _ => &[],
        }
    }

    /// Returns the recommended outer DIN-rail busbar carrier (`Sammelschienenträger` / `Auflagebock`)
    /// placed before the N-supply terminal and after the last installation/NT block.
    pub fn recommended_outer_busbar_carrier_spec_id(&self) -> Option<&'static str> {
        if !(self.is_installation_or_nt_terminal() || self.is_n_supply_terminal()) {
            return None;
        }
        let id = self.id.as_str();
        match self.manufacturer.as_str() {
            "WAGO" => Some("wago-2009-305"),
            "Phoenix Contact" => {
                if id.starts_with("phoenix-uti-6") {
                    Some("phoenix-ab-uti-6-3")
                } else if id.starts_with("phoenix-uti-") || id.starts_with("phoenix-utn-") {
                    Some("phoenix-ab-uti-3")
                } else if id.starts_with("phoenix-sti-") || id.starts_with("phoenix-stn-") {
                    Some("phoenix-ab-sti-3")
                } else if id.starts_with("phoenix-pti-16") || id.starts_with("phoenix-ptn-16") {
                    Some("phoenix-ab-pti-16-nls-bu")
                } else if id.starts_with("phoenix-pti-6") || id.starts_with("phoenix-ptn-6") {
                    Some("phoenix-ab-pti-6")
                } else if id.starts_with("phoenix-pti-4") {
                    if self.levels <= 1 {
                        Some("phoenix-ab-pti")
                    } else {
                        Some("phoenix-ab-pti-4-3")
                    }
                } else if id.starts_with("phoenix-ptn-") {
                    Some("phoenix-ab-pti")
                } else if id.starts_with("phoenix-pti-") {
                    if self.levels <= 1 {
                        Some("phoenix-ab-pti")
                    } else {
                        Some("phoenix-ab-pti-3")
                    }
                } else {
                    Some("phoenix-ab-ss")
                }
            }
            "Weidmüller" => {
                if id.contains("aitb-16") {
                    Some("weidmuller-aeb-itb-16-bl")
                } else if id.contains("aitb") {
                    Some("weidmuller-aeb-itb-25-4-bl")
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns the recommended intermediate busbar support (`Zwischenstütze`) inserted every 200 mm
    /// on long installation terminal rows (e.g., WAGO `2009-304` 1.5 mm, Phoenix Contact `AB-*` 200 mm pitch).
    pub fn recommended_intermediate_busbar_support_spec_id(&self) -> Option<&'static str> {
        let outer = self.recommended_outer_busbar_carrier_spec_id()?;
        match self.manufacturer.as_str() {
            "WAGO" => Some("wago-2009-304"),
            _ => Some(outer),
        }
    }

    /// Returns all compatible busbar carrier / support bracket IDs for the prompt dropdown.
    pub fn compatible_busbar_carrier_spec_ids(&self) -> &'static [&'static str] {
        match self.manufacturer.as_str() {
            "WAGO" => &["wago-2009-305", "wago-2009-304"],
            "Phoenix Contact" => &[
                "phoenix-ab-pti-3",
                "phoenix-ab-pti-4-3",
                "phoenix-ab-pti",
                "phoenix-ab-pti-6",
                "phoenix-ab-pti-16-nls-bu",
                "phoenix-ab-uti-3",
                "phoenix-ab-uti-6-3",
                "phoenix-ab-sti-3",
                "phoenix-ab-ss",
            ],
            "Weidmüller" => &["weidmuller-aeb-itb-25-4-bl", "weidmuller-aeb-itb-16-bl"],
            _ => &[],
        }
    }

    /// Returns `true` if `self` matches a lowercase search query (supports multi-word plain-text
    /// queries such as `"3 Stock klemme"`, `"NT klemme"`, `"2,5mm² blau"`, `"Auflagebock"`, or article numbers).
    pub fn matches_search_query(&self, q_lower: &str) -> bool {
        let parsed = parse_search_query(q_lower);
        self.score_search_query(&parsed).is_some()
    }

    /// Computes a relevance score if `self` matches `parsed`, or `None` if it does not match.
    pub fn score_search_query(&self, parsed: &ParsedSearchQuery) -> Option<u32> {
        if parsed.tokens.is_empty() {
            return Some(0);
        }

        let name_fold = fold_search_text(&self.name);
        let series_fold = fold_search_text(&self.series);
        let mfg_fold = fold_search_text(&self.manufacturer);
        let id_fold = fold_search_text(&self.id);
        let pat_fold = fold_search_text(&self.pattern_summary());
        let desc_de_fold = fold_search_text(&self.description_de);
        let desc_en_fold = fold_search_text(&self.description_en);
        let art_fold = self
            .effective_article_number()
            .map(fold_search_text)
            .unwrap_or_default();
        let tags_fold = self.domain_search_tags();

        let primary_text = format!("{name_fold} {art_fold} {series_fold} {id_fold} {pat_fold}");
        let full_text = format!(
            "{primary_text} {mfg_fold} {desc_de_fold} {desc_en_fold} {tags_fold}"
        );

        let primary_words = extract_search_words(&primary_text);
        let secondary_words = extract_search_words(&format!(
            "{mfg_fold} {desc_de_fold} {desc_en_fold} {tags_fold}"
        ));
        let compact_primary = compact_alnum(&primary_text);
        let compact_full = compact_alnum(&full_text);

        let mut score: u32 = 1;
        for tok in &parsed.tokens {
            match tok {
                QueryToken::Level(n) => {
                    if self.levels != *n {
                        return None;
                    }
                    score += 80;
                }
                QueryToken::MultiLevel => {
                    if self.levels < 2 {
                        return None;
                    }
                    score += 60;
                }
                QueryToken::GenericClamp => {
                    // Every catalog entry is a clamp/rail component; prefer actual terminal blocks over accessories
                    if !self.is_busbar_carrier() {
                        score += 20;
                    }
                }
                QueryToken::Term(term) => {
                    let is_short_alpha =
                        term.len() <= 2 && term.chars().all(|c| c.is_ascii_alphabetic());

                    let match_in_words = |words: &[String], exact_only_short: bool| -> bool {
                        if is_short_alpha {
                            words
                                .iter()
                                .any(|w| w == term || (!exact_only_short && w.starts_with(term)))
                        } else {
                            words.iter().any(|w| w.contains(term))
                        }
                    };

                    let in_primary = if is_short_alpha {
                        match_in_words(&primary_words, false)
                    } else {
                        primary_text.contains(term)
                            || match_in_words(&primary_words, false)
                            || (term.len() >= 3 && compact_primary.contains(term))
                    };

                    let in_secondary = if in_primary {
                        false
                    } else if is_short_alpha {
                        // Avoid noisy substrings like "nt" inside "Phoenix Contact" / "potential"
                        match_in_words(&secondary_words, false)
                    } else {
                        full_text.contains(term)
                            || match_in_words(&secondary_words, false)
                            || (term.len() >= 3 && compact_full.contains(term))
                    };

                    if !in_primary && !in_secondary {
                        return None;
                    }

                    if in_primary {
                        score += 120;
                        if primary_words.iter().any(|w| w == term) {
                            score += 60;
                        }
                    } else {
                        score += 40;
                    }

                    if term == "nt" && self.is_nt_disconnect_terminal() {
                        score += 90;
                    }
                }
            }
        }

        // Exact / prefix boosts for fast article-number & type lookup
        if !parsed.folded_query.is_empty() {
            if art_fold == parsed.folded_query
                || name_fold == parsed.folded_query
                || id_fold == parsed.folded_query
            {
                score += 1000;
            } else if (!art_fold.is_empty() && art_fold.starts_with(&parsed.folded_query))
                || name_fold.starts_with(&parsed.folded_query)
            {
                score += 450;
            } else if name_fold.contains(&parsed.folded_query)
                || art_fold.contains(&parsed.folded_query)
            {
                score += 200;
            }
        }

        Some(score)
    }

    fn domain_search_tags(&self) -> String {
        let mut tags = Vec::with_capacity(18);
        match self.levels {
            1 => tags.push("1 etage stock einstock durchgang standard"),
            2 => tags.push("2 etagen stock zweistock doppelstock mehrstock"),
            3 => tags.push("3 etagen stock dreistock mehrstock"),
            4 => tags.push("4 etagen stock vierstock mehrstock"),
            _ => tags.push("mehrstock"),
        }

        if self.is_nt_disconnect_terminal() {
            tags.push("nt trennschlitten nullleitertrenn installationsklemme");
        }
        if self.is_n_supply_terminal() {
            tags.push("einspeisung einspeiseklemme sammelschieneneinspeisung nl nls");
        }
        if self.is_busbar_carrier() {
            tags.push("sammelschienentraeger auflagebock traeger stuetze zwischenstuetze");
        }
        if self.is_installation_or_nt_terminal() {
            tags.push("installation installationsklemme etagenklemme");
        }

        let pe_count = self.tiers.iter().filter(|t| t.behavior.is_pe()).count();
        if pe_count > 0 {
            tags.push("pe schutzleiter erde erdklemme erdungsklemme gruengelb");
            if pe_count == self.levels && self.levels >= 2 {
                tags.push("mehrstockpe doppelstockpe dreistockpe");
            }
        }
        if self
            .tiers
            .iter()
            .any(|t| t.behavior.is_neutral() || t.color == crate::model::TierColor::BlueNeutral)
        {
            tags.push("neutral neutralleiter blau blue nullleiter");
        }

        let lower_id = self.id.as_str();
        let lower_name = self.name.to_lowercase();
        if lower_name.contains("twin") || lower_id.contains("-3c") || lower_id.contains("a3c") {
            tags.push("twin 3leiter dreileiter mehrleiter");
        }
        if lower_name.contains("quattro") || lower_id.contains("-4c") || lower_id.contains("a4c") {
            tags.push("quattro 4leiter vierleiter mehrleiter");
        }
        if lower_id.contains("hesi")
            || lower_id.contains("wsi")
            || lower_id.contains("afs")
            || lower_id.contains("zsi")
            || lower_id.contains("-161")
            || lower_name.contains("sicher")
        {
            tags.push("sicherung sicherungsklemme feinsicherung fuse 5x20");
        }
        if lower_id.contains("-mt")
            || lower_id.contains("-tg")
            || lower_id.contains("wtr")
            || lower_id.contains("wtl")
            || lower_id.contains("udmtk")
            || lower_id.contains("ptme")
            || lower_id.contains("-166")
            || lower_id.contains("-186")
            || lower_id.contains("-296")
            || lower_name.contains("trenn")
        {
            tags.push("trenn trennklemme messertrenn messwandler trennschlitten");
        }
        if lower_id.contains("ptio")
            || lower_id.contains("dikd")
            || lower_id.contains("dli")
            || lower_id.contains("dld")
            || lower_id.contains("aio")
            || lower_id.contains("-53")
            || lower_id.contains("-54")
        {
            tags.push("sensor aktor aktorklemme sensorklemme initiator initiatorklemme");
        }
        if lower_id.contains("ptpower")
            || lower_id.contains("-285-")
            || lower_id.contains("a2c-35")
            || lower_id.contains("a2c-50")
            || lower_id.contains("a2c-95")
            || lower_id.contains("wdu-35")
            || lower_id.contains("50n")
            || lower_id.contains("70n")
        {
            tags.push("hochstrom hochstromklemme starkstrom");
        }
        if lower_id.starts_with("phoenix-pt")
            || lower_id.starts_with("wago-20")
            || lower_id.starts_with("wago-21")
            || lower_id.starts_with("wago-22")
            || lower_id.starts_with("weidmuller-a")
        {
            tags.push("pushin push-in steckklemme federkraft");
        } else if lower_id.starts_with("phoenix-st")
            || lower_id.starts_with("weidmuller-z")
        {
            tags.push("zugfeder federzug");
        } else if lower_id.starts_with("phoenix-ut")
            || lower_id.starts_with("phoenix-uk")
            || lower_id.starts_with("phoenix-udmtk")
            || lower_id.starts_with("phoenix-uslkg")
            || lower_id.starts_with("weidmuller-w")
        {
            tags.push("schraub schraubklemme");
        }

        tags.join(" ")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryToken {
    Level(usize),
    MultiLevel,
    GenericClamp,
    Term(String),
}

#[derive(Debug, Clone)]
pub struct ParsedSearchQuery {
    pub folded_query: String,
    pub tokens: Vec<QueryToken>,
}

fn fold_search_text(input: &str) -> String {
    let lower = input.trim().to_lowercase();
    let mut folded = String::with_capacity(lower.len() + 8);
    let chars: Vec<char> = lower.chars().collect();
    for (idx, &ch) in chars.iter().enumerate() {
        match ch {
            'ä' => folded.push_str("ae"),
            'ö' => folded.push_str("oe"),
            'ü' => folded.push_str("ue"),
            'ß' => folded.push_str("ss"),
            '²' => folded.push('2'),
            '³' => folded.push('3'),
            ',' => {
                let prev_digit = idx > 0 && chars[idx - 1].is_ascii_digit();
                let next_digit = idx + 1 < chars.len() && chars[idx + 1].is_ascii_digit();
                if prev_digit && next_digit {
                    folded.push('.');
                } else {
                    folded.push(' ');
                }
            }
            _ => folded.push(ch),
        }
    }

    // Separate trailing "mm" / "mm2" glued to a digit (e.g. "2.5mm2" -> "2.5 mm2", "6mm" -> "6 mm")
    let out_chars: Vec<char> = folded.chars().collect();
    let mut spaced = String::with_capacity(folded.len() + 8);
    let mut i = 0;
    while i < out_chars.len() {
        if i > 0
            && out_chars[i - 1].is_ascii_digit()
            && out_chars[i] == 'm'
            && i + 1 < out_chars.len()
            && out_chars[i + 1] == 'm'
        {
            let after_mm = if i + 2 < out_chars.len() && out_chars[i + 2] == '2' {
                i + 3
            } else {
                i + 2
            };
            if after_mm == out_chars.len() || !out_chars[after_mm].is_ascii_alphanumeric() {
                spaced.push(' ');
            }
        }
        spaced.push(out_chars[i]);
        i += 1;
    }
    spaced
}

fn extract_search_words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut cur = String::new();
    let chars: Vec<char> = text.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        let keep_dot = c == '.'
            && i > 0
            && i + 1 < chars.len()
            && chars[i - 1].is_ascii_digit()
            && chars[i + 1].is_ascii_digit();
        if c.is_ascii_alphanumeric() || keep_dot {
            cur.push(c);
        } else if !cur.is_empty() {
            words.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        words.push(cur);
    }
    words
}

fn compact_alnum(text: &str) -> String {
    text.chars().filter(|c| c.is_ascii_alphanumeric()).collect()
}

pub fn parse_search_query(raw: &str) -> ParsedSearchQuery {
    let folded_query = fold_search_text(raw);
    if folded_query.is_empty() {
        return ParsedSearchQuery {
            folded_query,
            tokens: Vec::new(),
        };
    }

    let mut normalized = folded_query.clone();
    for (pat, rep) in [
        ("stockklemmen", "stock klemme"),
        ("stockklemme", "stock klemme"),
        ("etagenklemmen", "etagen klemme"),
        ("etagenklemme", "etagen klemme"),
        ("mehrstockklemmen", "mehrstock klemme"),
        ("mehrstockklemme", "mehrstock klemme"),
        ("dreistoeckige", "3 stock"),
        ("dreistoeckig", "3 stock"),
        ("dreistock", "3 stock"),
        ("3stock", "3 stock"),
        ("zweistoeckige", "2 stock"),
        ("zweistoeckig", "2 stock"),
        ("zweistock", "2 stock"),
        ("doppelstock", "2 stock"),
        ("2stock", "2 stock"),
        ("einstoeckige", "1 stock"),
        ("einstoeckig", "1 stock"),
        ("einstock", "1 stock"),
        ("1stock", "1 stock"),
        ("vierstoeckige", "4 stock"),
        ("vierstoeckig", "4 stock"),
        ("vierstock", "4 stock"),
        ("4stock", "4 stock"),
        ("ntklemmen", "nt klemme"),
        ("ntklemme", "nt klemme"),
        ("peklemmen", "pe klemme"),
        ("peklemme", "pe klemme"),
        ("sicherungsklemmen", "sicherung klemme"),
        ("sicherungsklemme", "sicherung klemme"),
        ("trennklemmen", "trenn klemme"),
        ("trennklemme", "trenn klemme"),
        ("durchgangsklemmen", "durchgang klemme"),
        ("durchgangsklemme", "durchgang klemme"),
        ("schutzleiterklemmen", "schutzleiter klemme"),
        ("schutzleiterklemme", "schutzleiter klemme"),
        ("installationsklemmen", "installation klemme"),
        ("installationsklemme", "installation klemme"),
        ("einspeiseklemmen", "einspeisung klemme"),
        ("einspeiseklemme", "einspeisung klemme"),
    ] {
        if normalized.contains(pat) {
            normalized = normalized.replace(pat, rep);
        }
    }

    let words = extract_search_words(&normalized);
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < words.len() {
        let w = words[i].as_str();
        if matches!(w, "1" | "2" | "3" | "4")
            && i + 1 < words.len()
            && matches!(
                words[i + 1].as_str(),
                "stock" | "stoeckig" | "stoeckige" | "etage" | "etagen" | "ebene" | "ebenen"
            )
            && let Ok(lvl) = w.parse::<usize>()
        {
            tokens.push(QueryToken::Level(lvl));
            i += 2;
            continue;
        }
        match w {
            "mehrstock" | "mehrstoeckig" | "mehrstoeckige" | "etagenklemme" => {
                tokens.push(QueryToken::MultiLevel);
            }
            "klemme" | "klemmen" | "reihenklemme" | "reihenklemmen" => {
                tokens.push(QueryToken::GenericClamp);
            }
            _ => {
                tokens.push(QueryToken::Term(words[i].clone()));
            }
        }
        i += 1;
    }

    ParsedSearchQuery {
        folded_query,
        tokens,
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
        "phoenix-pt-15s-twin-pe" => Some(("3208171", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-15s-twin-pe-3208171", "https://caas.phoenixcontact.com/caas/v1/stable/media/271870/full/b408?format=jpg")),
        "phoenix-pt-15s-quattro" => Some(("3208197", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-15s-quattro-3208197", "https://caas.phoenixcontact.com/caas/v1/stable/media/262456/full/b408?format=jpg")),
        "phoenix-pt-15s-mt" => Some(("3210301", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-15s-mt-3210301", "https://caas.phoenixcontact.com/caas/v1/stable/media/202920/full/b408?format=jpg")),
        "phoenix-pt-15s-3l" => Some(("3213713", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pt-15s-3l-3213713", "https://caas.phoenixcontact.com/caas/v1/stable/media/203652/full/b408?format=jpg")),
        "phoenix-pt-15s-3pe" => Some(("3213739", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-15s-3pe-3213739", "https://caas.phoenixcontact.com/caas/v1/stable/media/262574/full/b408?format=jpg")),
        "phoenix-pttb-15s" => Some(("3208511", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-15s-3208511", "https://caas.phoenixcontact.com/caas/v1/stable/media/220810/full/b408?format=jpg")),
        "phoenix-pttb-15s-pe" => Some(("3208537", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pttb-15s-pe-3208537", "https://caas.phoenixcontact.com/caas/v1/stable/media/208664/full/b408?format=jpg")),
        "phoenix-ptio-15s-3" => Some(("3244410", "https://www.phoenixcontact.com/de-de/produkte/sensor-aktor-klemme-ptio-15s3-3244410", "https://caas.phoenixcontact.com/caas/v1/stable/media/210966/full/b408?format=jpg")),
        "phoenix-pt-25" => Some(("3209510", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-25-3209510", "https://caas.phoenixcontact.com/caas/v1/stable/media/231482/full/b408?format=jpg")),
        "phoenix-pt-25-bu" => Some(("3209523", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-25-bu-3209523", "https://caas.phoenixcontact.com/caas/v1/stable/media/117936/full/b408?format=jpg")),
        "phoenix-pt-25-pe" => Some(("3209536", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-25-pe-3209536", "https://caas.phoenixcontact.com/caas/v1/stable/media/108618/full/b408?format=jpg")),
        "phoenix-pt-25-twin" => Some(("3209549", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-25-twin-3209549", "https://caas.phoenixcontact.com/caas/v1/stable/media/226530/full/b408?format=jpg")),
        "phoenix-pt-25-twin-bu" => Some(("3209552", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-25-twin-bu-3209552", "https://caas.phoenixcontact.com/caas/v1/stable/media/218680/full/b408?format=jpg")),
        "phoenix-pt-25-twin-pe" => Some(("3209565", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-25-twin-pe-3209565", "https://caas.phoenixcontact.com/caas/v1/stable/media/127104/full/b408?format=jpg")),
        "phoenix-pt-25-quattro" => Some(("3209578", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-25-quattro-3209578", "https://caas.phoenixcontact.com/caas/v1/stable/media/200178/full/b408?format=jpg")),
        "phoenix-pt-25-quattro-bu" => Some(("3209581", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-25-quattro-bu-3209581", "https://caas.phoenixcontact.com/caas/v1/stable/media/126568/full/b408?format=jpg")),
        "phoenix-pt-25-quattro-pe" => Some(("3209594", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-25-quattro-pe-3209594", "https://caas.phoenixcontact.com/caas/v1/stable/media/260232/full/b408?format=jpg")),
        "phoenix-pt-25-mt" => Some(("3210156", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-25-mt-3210156", "https://caas.phoenixcontact.com/caas/v1/stable/media/31502/full/b408?format=jpg")),
        "phoenix-pt-25-tg" => Some(("3210185", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-25-tg-3210185", "https://caas.phoenixcontact.com/caas/v1/stable/media/31696/full/b408?format=jpg")),
        "phoenix-pttb-25" => Some(("3210567", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-25-3210567", "https://caas.phoenixcontact.com/caas/v1/stable/media/212446/full/b408?format=jpg")),
        "phoenix-pttb-25-pe" => Some(("3210596", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pttb-25-pe-3210596", "https://caas.phoenixcontact.com/caas/v1/stable/media/235702/full/b408?format=jpg")),
        "phoenix-pttb-25-pel" => Some(("3210978", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pttb-25-pel-3210978", "https://caas.phoenixcontact.com/caas/v1/stable/media/264750/full/b408?format=jpg")),
        "phoenix-pttbs-25" => Some(("3209604", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttbs-25-3209604", "https://caas.phoenixcontact.com/caas/v1/stable/media/262156/full/b408?format=jpg")),
        "phoenix-pt-25-3l" => Some(("3210499", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pt-25-3l-3210499", "https://caas.phoenixcontact.com/caas/v1/stable/media/202242/full/b408?format=jpg")),
        "phoenix-pt-25-3pe" => Some(("3210525", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-25-3pe-3210525", "https://caas.phoenixcontact.com/caas/v1/stable/media/201896/full/b408?format=jpg")),
        "phoenix-pt-25-peln" => Some(("3210538", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-25-peln-3210538", "https://caas.phoenixcontact.com/caas/v1/stable/media/213800/full/b408?format=jpg")),
        "phoenix-pt-25-pell" => Some(("3210541", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-25-pell-3210541", "https://caas.phoenixcontact.com/caas/v1/stable/media/270960/full/b408?format=jpg")),
        "phoenix-pti-16-nls-bu" => Some(("1030131", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-16-nls-fi-bu-1030131", "https://caas.phoenixcontact.com/caas/v1/stable/media/253860/full/b408?format=jpg")),
        "phoenix-pti-16-nls" => Some(("1030130", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-16-nls-fi-1030130", "https://caas.phoenixcontact.com/caas/v1/stable/media/265136/full/b408?format=jpg")),
        "phoenix-ptn-25" => Some(("3213963", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-ptn-25-3213963", "https://caas.phoenixcontact.com/caas/v1/stable/media/212566/full/b408?format=jpg")),
        "phoenix-ptn-4" => Some(("3213965", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-ptn-4-3213965", "https://caas.phoenixcontact.com/caas/v1/stable/media/216676/full/b408?format=jpg")),
        "phoenix-ptn-6" => Some(("3213967", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-ptn-6-3213967", "https://caas.phoenixcontact.com/caas/v1/stable/media/205572/full/b408?format=jpg")),
        "phoenix-ptn-16s" => Some(("3214025", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-ptn-16s-3214025", "https://caas.phoenixcontact.com/caas/v1/stable/media/198908/full/b408?format=jpg")),
        "phoenix-pti-16s" => Some(("3214029", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-16s-3214029", "https://caas.phoenixcontact.com/caas/v1/stable/media/198028/full/b408?format=jpg")),
        "phoenix-pti-16s-bu" => Some(("3214023", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-16s-bu-3214023", "https://caas.phoenixcontact.com/caas/v1/stable/media/196908/full/b408?format=jpg")),
        "phoenix-pti-16s-pe" => Some(("3214024", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-16s-pe-3214024", "https://caas.phoenixcontact.com/caas/v1/stable/media/216274/full/b408?format=jpg")),
        "phoenix-utn-25" => Some(("3245011", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-utn-25-3245011", "https://caas.phoenixcontact.com/caas/v1/stable/media/243838/full/b408?format=jpg")),
        "phoenix-utn-4" => Some(("3245024", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-utn-4-3245024", "https://caas.phoenixcontact.com/caas/v1/stable/media/236014/full/b408?format=jpg")),
        "phoenix-utn-6" => Some(("3245037", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-utn-6-3245037", "https://caas.phoenixcontact.com/caas/v1/stable/media/200156/full/b408?format=jpg")),
        "phoenix-utn-10" => Some(("3245040", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-utn-10-3245040", "https://caas.phoenixcontact.com/caas/v1/stable/media/204904/full/b408?format=jpg")),
        "phoenix-utn-16" => Some(("3245053", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-utn-16-3245053", "https://caas.phoenixcontact.com/caas/v1/stable/media/259262/full/b408?format=jpg")),
        "phoenix-utn-35" => Some(("3245066", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-utn-35-3245066", "https://caas.phoenixcontact.com/caas/v1/stable/media/244166/full/b408?format=jpg")),
        "phoenix-stn-25" => Some(("3031940", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-stn-25-3031940", "https://caas.phoenixcontact.com/caas/v1/stable/media/233464/full/b408?format=jpg")),
        "phoenix-stn-4" => Some(("3031979", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-stn-4-3031979", "https://caas.phoenixcontact.com/caas/v1/stable/media/108630/full/b408?format=jpg")),
        "phoenix-stn-10" => Some(("3038244", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-stn-10-3038244", "https://caas.phoenixcontact.com/caas/v1/stable/media/233464/full/b408?format=jpg")),
        "phoenix-stn-16" => Some(("3038286", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-stn-16-3038286", "https://caas.phoenixcontact.com/caas/v1/stable/media/108630/full/b408?format=jpg")),
        "phoenix-pti-25-peln" => Some(("3213950", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-25-peln-3213950", "https://caas.phoenixcontact.com/caas/v1/stable/media/244572/full/b408?format=jpg")),
        "phoenix-pti-25-pelnt" => Some(("3213946", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-25-pelnt-3213946", "https://caas.phoenixcontact.com/caas/v1/stable/media/123154/full/b408?format=jpg")),
        "phoenix-pti-25-pell" => Some(("3213949", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-25-pell-3213949", "https://caas.phoenixcontact.com/caas/v1/stable/media/212236/full/b408?format=jpg")),
        "phoenix-pti-25-ll" => Some(("3213953", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-25-ll-3213953", "https://caas.phoenixcontact.com/caas/v1/stable/media/214438/full/b408?format=jpg")),
        "phoenix-pti-25-ln" => Some(("3213952", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-25-n-3213952", "https://caas.phoenixcontact.com/caas/v1/stable/media/208590/full/b408?format=jpg")),
        "phoenix-pti-25-lnt" => Some(("3213947", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-25-lnt-3213947", "https://caas.phoenixcontact.com/caas/v1/stable/media/272210/full/b408?format=jpg")),
        "phoenix-pti-4-peln" => Some(("3214049", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-4-peln-3214049", "https://caas.phoenixcontact.com/caas/v1/stable/media/121390/full/b408?format=jpg")),
        "phoenix-pti-4-pelnt" => Some(("3214047", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-4-pelnt-3214047", "https://caas.phoenixcontact.com/caas/v1/stable/media/198188/full/b408?format=jpg")),
        "phoenix-pti-4-pell" => Some(("3214050", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-4-pell-3214050", "https://caas.phoenixcontact.com/caas/v1/stable/media/227746/full/b408?format=jpg")),
        "phoenix-pti-4-ln" => Some(("3214051", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-4-ln-3214051", "https://caas.phoenixcontact.com/caas/v1/stable/media/251806/full/b408?format=jpg")),
        "phoenix-pti-6" => Some(("3213972", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-6-3213972", "https://caas.phoenixcontact.com/caas/v1/stable/media/217232/full/b408?format=jpg")),
        "phoenix-pti-6-bu" => Some(("3213973", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-6-bu-3213973", "https://caas.phoenixcontact.com/caas/v1/stable/media/263182/full/b408?format=jpg")),
        "phoenix-pti-6-pe" => Some(("3213966", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-6-pe-3213966", "https://caas.phoenixcontact.com/caas/v1/stable/media/230828/full/b408?format=jpg")),
        "phoenix-pt-4" => Some(("3211757", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-4-3211757", "https://caas.phoenixcontact.com/caas/v1/stable/media/270446/full/b408?format=jpg")),
        "phoenix-pt-4-bu" => Some(("3211760", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-4-bu-3211760", "https://caas.phoenixcontact.com/caas/v1/stable/media/203340/full/b408?format=jpg")),
        "phoenix-pt-4-pe" => Some(("3211766", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-4-pe-3211766", "https://caas.phoenixcontact.com/caas/v1/stable/media/223636/full/b408?format=jpg")),
        "phoenix-pt-4-twin" => Some(("3211771", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-4-twin-3211771", "https://caas.phoenixcontact.com/caas/v1/stable/media/257492/full/b408?format=jpg")),
        "phoenix-pt-4-twin-pe" => Some(("3211780", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-4-twin-pe-3211780", "https://caas.phoenixcontact.com/caas/v1/stable/media/232950/full/b408?format=jpg")),
        "phoenix-pt-4-quattro" => Some(("3211797", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-4-quattro-3211797", "https://caas.phoenixcontact.com/caas/v1/stable/media/231084/full/b408?format=jpg")),
        "phoenix-pt-4-hesi" => Some(("3211861", "https://www.phoenixcontact.com/de-de/produkte/sicherungsklemme-pt-4-hesi-5x20-3211861", "https://caas.phoenixcontact.com/caas/v1/stable/media/31572/full/b408?format=jpg")),
        "phoenix-pt-4-mt" => Some(("3211933", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-4-mt-3211933", "https://caas.phoenixcontact.com/caas/v1/stable/media/28302/full/b408?format=jpg")),
        "phoenix-pttb-4" => Some(("3211786", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-4-3211786", "https://caas.phoenixcontact.com/caas/v1/stable/media/230754/full/b408?format=jpg")),
        "phoenix-pttb-4-bu" => Some(("3211793", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-4-bu-3211793", "https://caas.phoenixcontact.com/caas/v1/stable/media/267796/full/b408?format=jpg")),
        "phoenix-pttb-4-pe" => Some(("3211854", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pttb-4-pe-3211854", "https://caas.phoenixcontact.com/caas/v1/stable/media/262544/full/b408?format=jpg")),
        "phoenix-pt-6" => Some(("3211813", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-6-3211813", "https://caas.phoenixcontact.com/caas/v1/stable/media/231218/full/b408?format=jpg")),
        "phoenix-pt-6-bu" => Some(("3211819", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-6-bu-3211819", "https://caas.phoenixcontact.com/caas/v1/stable/media/233840/full/b408?format=jpg")),
        "phoenix-pt-6-pe" => Some(("3211822", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-6-pe-3211822", "https://caas.phoenixcontact.com/caas/v1/stable/media/269426/full/b408?format=jpg")),
        "phoenix-pt-6-t" => Some(("1028589", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-6-t-pp-hv-1028589", "https://caas.phoenixcontact.com/caas/v1/stable/media/263362/full/b408?format=jpg")),
        "phoenix-ptme-6" => Some(("3212170", "https://www.phoenixcontact.com/de-de/produkte/messwandlertrennklemme-ptme-6-3212170", "https://caas.phoenixcontact.com/caas/v1/stable/media/263612/full/b408?format=jpg")),
        "phoenix-pt-10" => Some(("3212120", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-10-3212120", "https://caas.phoenixcontact.com/caas/v1/stable/media/214944/full/b408?format=jpg")),
        "phoenix-pt-16n" => Some(("3212138", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-16-n-3212138", "https://caas.phoenixcontact.com/caas/v1/stable/media/229934/full/b408?format=jpg")),
        "phoenix-ptpower-35" => Some(("3212064", "https://www.phoenixcontact.com/de-de/produkte/hochstromklemme-ptpower-35-3212064", "https://caas.phoenixcontact.com/caas/v1/stable/media/228830/full/b408?format=jpg")),
        "phoenix-ptpower-50" => Some(("3260050", "https://www.phoenixcontact.com/de-de/produkte/hochstromklemme-ptpower-50-3260050", "https://caas.phoenixcontact.com/caas/v1/stable/media/251280/full/b408?format=jpg")),
        "phoenix-ptpower-95" => Some(("3260100", "https://www.phoenixcontact.com/de-de/produkte/hochstromklemme-ptpower-95-3260100", "https://caas.phoenixcontact.com/caas/v1/stable/media/251280/full/b408?format=jpg")),
        "phoenix-ptv-25" => Some(("1078960", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ptv-25-1078960", "https://caas.phoenixcontact.com/caas/v1/stable/media/125318/full/b408?format=jpg")),
        "phoenix-ptv-25-bu" => Some(("1078962", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ptv-25-bu-1078962", "https://caas.phoenixcontact.com/caas/v1/stable/media/113492/full/b408?format=jpg")),
        "phoenix-ptv-25-pe" => Some(("1078963", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ptv-25-pe-1078963", "https://caas.phoenixcontact.com/caas/v1/stable/media/122446/full/b408?format=jpg")),
        "phoenix-ptv-4" => Some(("1088728", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ptv-4-1088728", "https://caas.phoenixcontact.com/caas/v1/stable/media/108504/full/b408?format=jpg")),
        "phoenix-ptv-4-pe" => Some(("1088730", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ptv-4-pe-1088730", "https://caas.phoenixcontact.com/caas/v1/stable/media/252896/full/b408?format=jpg")),
        "phoenix-st-25" => Some(("3031212", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-st-25-3031212", "https://caas.phoenixcontact.com/caas/v1/stable/media/246770/full/b408?format=jpg")),
        "phoenix-st-25-bu" => Some(("3031225", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-st-25-bu-3031225", "https://caas.phoenixcontact.com/caas/v1/stable/media/264912/full/b408?format=jpg")),
        "phoenix-st-25-pe" => Some(("3031238", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-st-25-pe-3031238", "https://caas.phoenixcontact.com/caas/v1/stable/media/112684/full/b408?format=jpg")),
        "phoenix-sttb-25" => Some(("3031270", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-sttb-25-3031270", "https://caas.phoenixcontact.com/caas/v1/stable/media/229202/full/b408?format=jpg")),
        "phoenix-sttb-25-pe" => Some(("3036026", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-sttb-25-pe-3036026", "https://caas.phoenixcontact.com/caas/v1/stable/media/125436/full/b408?format=jpg")),
        "phoenix-sttb-25-pel" => Some(("3036314", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-sttb-25-pel-3036314", "https://caas.phoenixcontact.com/caas/v1/stable/media/117532/full/b408?format=jpg")),
        "phoenix-ut-25" => Some(("3044076", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-25-3044076", "https://caas.phoenixcontact.com/caas/v1/stable/media/209542/full/b408?format=jpg")),
        "phoenix-ut-25-pe" => Some(("3044092", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ut-25-pe-3044092", "https://caas.phoenixcontact.com/caas/v1/stable/media/116466/full/b408?format=jpg")),
        "phoenix-ut-4" => Some(("3044102", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-4-3044102", "https://caas.phoenixcontact.com/caas/v1/stable/media/195454/full/b408?format=jpg")),
        "phoenix-ut-4-hesi" => Some(("3046032", "https://www.phoenixcontact.com/de-de/produkte/sicherungsklemme-ut-4-hesi-5x20-3046032", "https://caas.phoenixcontact.com/caas/v1/stable/media/108038/full/b408?format=jpg")),
        "phoenix-ut-6" => Some(("3044131", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-6-3044131", "https://caas.phoenixcontact.com/caas/v1/stable/media/195702/full/b408?format=jpg")),
        "phoenix-ut-10" => Some(("3044160", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-10-3044160", "https://caas.phoenixcontact.com/caas/v1/stable/media/119830/full/b408?format=jpg")),
        "phoenix-ut-16" => Some(("3044199", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-16-3044199", "https://caas.phoenixcontact.com/caas/v1/stable/media/121920/full/b408?format=jpg")),
        "phoenix-uttb-25" => Some(("3044636", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-uttb-25-3044636", "https://caas.phoenixcontact.com/caas/v1/stable/media/255718/full/b408?format=jpg")),
        "phoenix-uttb-25-pe" => Some(("3044665", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-uttb-25-pe-3044665", "https://caas.phoenixcontact.com/caas/v1/stable/media/199224/full/b408?format=jpg")),
        "phoenix-uttb-25-pel" => Some(("3044678", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-uttb-25-pel-3044678", "https://caas.phoenixcontact.com/caas/v1/stable/media/127298/full/b408?format=jpg")),
        "phoenix-uti-25-peln" => Some(("3076028", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-uti-25-pelnt-3076028", "https://caas.phoenixcontact.com/caas/v1/stable/media/236244/full/b408?format=jpg")),
        "phoenix-uk-5n" => Some(("3004362", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-uk-5-n-3004362", "https://caas.phoenixcontact.com/caas/v1/stable/media/268498/full/b408?format=jpg")),
        "phoenix-udmtk-5" => Some(("3048467", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-udmtk-5-3048467", "https://caas.phoenixcontact.com/caas/v1/stable/media/217516/full/b408?format=jpg")),
        "phoenix-udmtk-5-pp" => Some(("3101087", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-udmtk-5-pp-3101087", "https://caas.phoenixcontact.com/caas/v1/stable/media/235500/full/b408?format=jpg")),
        "phoenix-udmtk-5-pp-bu" => Some(("3101113", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-udmtk-5-pp-bu-3101113", "https://caas.phoenixcontact.com/caas/v1/stable/media/220494/full/b408?format=jpg")),
        "phoenix-udmtk-5-pl" => Some(("3048454", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-udmtk-5-pl-3048454", "https://caas.phoenixcontact.com/caas/v1/stable/media/221956/full/b408?format=jpg")),
        "phoenix-udmtk-5-twin-pp" => Some(("3101126", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-udmtk-5-twin-pp-3101126", "https://caas.phoenixcontact.com/caas/v1/stable/media/209994/full/b408?format=jpg")),
        "phoenix-udmtkb-5-pp" => Some(("3024478", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-udmtkb-5-pp-3024478", "https://caas.phoenixcontact.com/caas/v1/stable/media/116514/full/b408?format=jpg")),
        "phoenix-ukk-5-mtk-pp" => Some(("2800004", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-ukk-5-mtk-pp-2800004", "https://caas.phoenixcontact.com/caas/v1/stable/media/122734/full/b408?format=jpg")),
        "phoenix-uslkg-5" => Some(("0441504", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-uslkg-5-0441504", "https://caas.phoenixcontact.com/caas/v1/stable/media/253550/full/b408?format=jpg")),
        "phoenix-pt-25-quattro-tg" => Some(("3210208", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-25-quattro-tg-3210208", "https://caas.phoenixcontact.com/caas/v1/stable/media/31720/full/b408?format=jpg")),
        "phoenix-pt-6-tg" => Some(("3212163", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-6-tg-3212163", "https://caas.phoenixcontact.com/caas/v1/stable/media/198652/full/b408?format=jpg")),
        "phoenix-pti-25-pellt" => Some(("3213957", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-25-pelltb-3213957", "https://caas.phoenixcontact.com/caas/v1/stable/media/200814/full/b408?format=jpg")),
        "phoenix-pti-25-peltg" => Some(("3213960", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-25-peltg-3213960", "https://caas.phoenixcontact.com/caas/v1/stable/media/195334/full/b408?format=jpg")),
        "phoenix-pti-4-ll" => Some(("3214052", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-4-ll-3214052", "https://caas.phoenixcontact.com/caas/v1/stable/media/261974/full/b408?format=jpg")),
        "phoenix-pttb-25-bu" => Some(("3210570", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-25-bu-3210570", "https://caas.phoenixcontact.com/caas/v1/stable/media/233752/full/b408?format=jpg")),
        "phoenix-sti-25-pelnt" => Some(("3031827", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-sti-25-pelnt-3031827", "https://caas.phoenixcontact.com/caas/v1/stable/media/124280/full/b408?format=jpg")),
        "phoenix-ptv-4-twin-bu" => Some(("1088732", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ptv-4-twin-bu-1088732", "https://caas.phoenixcontact.com/caas/v1/stable/media/120990/full/b408?format=jpg")),
        "phoenix-ptv-4-twin-pe" => Some(("1088733", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ptv-4-twin-pe-1088733", "https://caas.phoenixcontact.com/caas/v1/stable/media/124398/full/b408?format=jpg")),
        "phoenix-ptv-4-quattro-bu" => Some(("1088735", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ptv-4-quattro-bu-1088735", "https://caas.phoenixcontact.com/caas/v1/stable/media/122088/full/b408?format=jpg")),
        "phoenix-ptv-4-quattro-pe" => Some(("1088736", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ptv-4-quattro-pe-1088736", "https://caas.phoenixcontact.com/caas/v1/stable/media/258876/full/b408?format=jpg")),
        "phoenix-ptvtb-25" => Some(("1079073", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttbv-25-1079073", "https://caas.phoenixcontact.com/caas/v1/stable/media/114148/full/b408?format=jpg")),
        "phoenix-ptvtb-25-pe" => Some(("1079076", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pttbv-25-pe-1079076", "https://caas.phoenixcontact.com/caas/v1/stable/media/232654/full/b408?format=jpg")),
        "phoenix-ptvtb-4" => Some(("1088737", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttbv-4-1088737", "https://caas.phoenixcontact.com/caas/v1/stable/media/116314/full/b408?format=jpg")),
        "phoenix-ptv-6" => Some(("1116734", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ptv-6-1116734", "https://caas.phoenixcontact.com/caas/v1/stable/media/246264/full/b408?format=jpg")),
        "phoenix-ptv-6-bu" => Some(("1116735", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ptv-6-bu-1116735", "https://caas.phoenixcontact.com/caas/v1/stable/media/246166/full/b408?format=jpg")),
        "phoenix-ptv-6-pe" => Some(("1116736", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ptv-6-pe-1116736", "https://caas.phoenixcontact.com/caas/v1/stable/media/227558/full/b408?format=jpg")),
        "phoenix-ptv-10" => Some(("1180897", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ptv-10-1180897", "https://caas.phoenixcontact.com/caas/v1/stable/media/246264/full/b408?format=jpg")),
        "phoenix-ptv-10-bu" => Some(("1180899", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ptv-10-bu-1180899", "https://caas.phoenixcontact.com/caas/v1/stable/media/246166/full/b408?format=jpg")),
        "phoenix-ptv-10-pe" => Some(("1180900", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ptv-10-pe-1180900", "https://caas.phoenixcontact.com/caas/v1/stable/media/227558/full/b408?format=jpg")),
        "phoenix-ptv-16" => Some(("1180906", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ptv-16-1180906", "https://caas.phoenixcontact.com/caas/v1/stable/media/246264/full/b408?format=jpg")),
        "phoenix-ptv-16-bu" => Some(("1180907", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ptv-16-bu-1180907", "https://caas.phoenixcontact.com/caas/v1/stable/media/246166/full/b408?format=jpg")),
        "phoenix-ptv-16-pe" => Some(("1180908", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ptv-16-pe-1180908", "https://caas.phoenixcontact.com/caas/v1/stable/media/227558/full/b408?format=jpg")),
        "phoenix-st-25-twin-bu" => Some(("3031254", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-st-25-twin-bu-3031254", "https://caas.phoenixcontact.com/caas/v1/stable/media/254356/full/b408?format=jpg")),
        "phoenix-st-25-twin-pe" => Some(("3031267", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-st-25-twin-pe-3031267", "https://caas.phoenixcontact.com/caas/v1/stable/media/208900/full/b408?format=jpg")),
        "phoenix-st-25-quattro-bu" => Some(("3031319", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-st-25-quattro-bu-3031319", "https://caas.phoenixcontact.com/caas/v1/stable/media/262934/full/b408?format=jpg")),
        "phoenix-st-25-quattro-pe" => Some(("3031322", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-st-25-quattro-pe-3031322", "https://caas.phoenixcontact.com/caas/v1/stable/media/119994/full/b408?format=jpg")),
        "phoenix-st-25-mt" => Some(("3036343", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-st-25-mt-3036343", "https://caas.phoenixcontact.com/caas/v1/stable/media/125796/full/b408?format=jpg")),
        "phoenix-sttb-25-pv" => Some(("3031539", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-sttb-25-pv-3031539", "https://caas.phoenixcontact.com/caas/v1/stable/media/113790/full/b408?format=jpg")),
        "phoenix-st-4-twin" => Some(("3031393", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-st-4-twin-3031393", "https://caas.phoenixcontact.com/caas/v1/stable/media/248382/full/b408?format=jpg")),
        "phoenix-st-4-quattro" => Some(("3031445", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-st-4-quattro-3031445", "https://caas.phoenixcontact.com/caas/v1/stable/media/224044/full/b408?format=jpg")),
        "phoenix-st-6" => Some(("3031487", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-st-6-3031487", "https://caas.phoenixcontact.com/caas/v1/stable/media/122908/full/b408?format=jpg")),
        "phoenix-st-6-bu" => Some(("3031490", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-st-6-bu-3031490", "https://caas.phoenixcontact.com/caas/v1/stable/media/239806/full/b408?format=jpg")),
        "phoenix-st-6-pe" => Some(("3031500", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-st-6-pe-3031500", "https://caas.phoenixcontact.com/caas/v1/stable/media/272326/full/b408?format=jpg")),
        "phoenix-st-10" => Some(("3036110", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-st-10-3036110", "https://caas.phoenixcontact.com/caas/v1/stable/media/272488/full/b408?format=jpg")),
        "phoenix-st-10-pe" => Some(("3036136", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-st-10-pe-3036136", "https://caas.phoenixcontact.com/caas/v1/stable/media/254760/full/b408?format=jpg")),
        "phoenix-st-16" => Some(("3036149", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-st-16-3036149", "https://caas.phoenixcontact.com/caas/v1/stable/media/228438/full/b408?format=jpg")),
        "phoenix-st-16-pe" => Some(("3036165", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-st-16-pe-3036165", "https://caas.phoenixcontact.com/caas/v1/stable/media/109630/full/b408?format=jpg")),
        "phoenix-ut-25-twin-bu" => Some(("3044526", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ut-25-twin-bu-3044526", "https://caas.phoenixcontact.com/caas/v1/stable/media/199266/full/b408?format=jpg")),
        "phoenix-ut-25-twin-pe" => Some(("3044539", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ut-25-twin-pe-3044539", "https://caas.phoenixcontact.com/caas/v1/stable/media/228218/full/b408?format=jpg")),
        "phoenix-ut-25-quattro-bu" => Some(("3044555", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ut-25-quattro-bu-3044555", "https://caas.phoenixcontact.com/caas/v1/stable/media/264826/full/b408?format=jpg")),
        "phoenix-ut-25-quattro-pe" => Some(("3044568", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ut-25-quattro-pe-3044568", "https://caas.phoenixcontact.com/caas/v1/stable/media/266566/full/b408?format=jpg")),
        "phoenix-uttb-25-pv" => Some(("3044652", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-uttb-25-pv-3044652", "https://caas.phoenixcontact.com/caas/v1/stable/media/269760/full/b408?format=jpg")),
        "phoenix-ut-4-twin" => Some(("3044364", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ut-4-twin-3044364", "https://caas.phoenixcontact.com/caas/v1/stable/media/108818/full/b408?format=jpg")),
        "phoenix-ut-4-twin-bu" => Some(("3044500", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ut-4-twin-bu-3044500", "https://caas.phoenixcontact.com/caas/v1/stable/media/248498/full/b408?format=jpg")),
        "phoenix-ut-4-twin-pe" => Some(("3044380", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ut-4-twin-pe-3044380", "https://caas.phoenixcontact.com/caas/v1/stable/media/114678/full/b408?format=jpg")),
        "phoenix-ut-4-quattro" => Some(("3044571", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ut-4-quattro-3044571", "https://caas.phoenixcontact.com/caas/v1/stable/media/107920/full/b408?format=jpg")),
        "phoenix-ut-4-quattro-pe" => Some(("3044597", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ut-4-quattro-pe-3044597", "https://caas.phoenixcontact.com/caas/v1/stable/media/247012/full/b408?format=jpg")),
        "phoenix-uttb-4" => Some(("3044814", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-uttb-4-3044814", "https://caas.phoenixcontact.com/caas/v1/stable/media/120002/full/b408?format=jpg")),
        "phoenix-uttb-4-pe" => Some(("3044759", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-uttb-4-pe-3044759", "https://caas.phoenixcontact.com/caas/v1/stable/media/108066/full/b408?format=jpg")),
        "phoenix-uttb-4-pv" => Some(("3044733", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-uttb-4-pv-3044733", "https://caas.phoenixcontact.com/caas/v1/stable/media/204540/full/b408?format=jpg")),
        "phoenix-ut-4-hesiled-24" => Some(("3046090", "https://www.phoenixcontact.com/de-de/produkte/sicherungsklemme-ut-4-hesiled-24-5x20-3046090", "https://caas.phoenixcontact.com/caas/v1/stable/media/234316/full/b408?format=jpg")),
        "phoenix-ut-10-bu" => Some(("3044188", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-10-bu-3044188", "https://caas.phoenixcontact.com/caas/v1/stable/media/253626/full/b408?format=jpg")),
        "phoenix-ut-16-bu" => Some(("3044209", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-16-bu-3044209", "https://caas.phoenixcontact.com/caas/v1/stable/media/243678/full/b408?format=jpg")),
        "phoenix-ut-16-pe" => Some(("3044212", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ut-16-pe-3044212", "https://caas.phoenixcontact.com/caas/v1/stable/media/261852/full/b408?format=jpg")),
        "phoenix-ut-35-bu" => Some(("3044238", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-35-bu-3044238", "https://caas.phoenixcontact.com/caas/v1/stable/media/201934/full/b408?format=jpg")),
        "phoenix-ut-35-pe" => Some(("3044241", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ut-35-pe-3044241", "https://caas.phoenixcontact.com/caas/v1/stable/media/213594/full/b408?format=jpg")),
        "phoenix-uk-5n-bu" => Some(("3004388", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-uk-5-n-bu-3004388", "https://caas.phoenixcontact.com/caas/v1/stable/media/228796/full/b408?format=jpg")),
        "phoenix-uk-6n" => Some(("3004524", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-uk-6-n-3004524", "https://caas.phoenixcontact.com/caas/v1/stable/media/123156/full/b408?format=jpg")),
        "phoenix-uk-10n" => Some(("3005073", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-uk-10-n-3005073", "https://caas.phoenixcontact.com/caas/v1/stable/media/125198/full/b408?format=jpg")),
        "phoenix-uk-16n" => Some(("3006043", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-uk-16-n-3006043", "https://caas.phoenixcontact.com/caas/v1/stable/media/196582/full/b408?format=jpg")),
        "phoenix-uslkg-6n" => Some(("0442079", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-uslkg-6-n-0442079", "https://caas.phoenixcontact.com/caas/v1/stable/media/263344/full/b408?format=jpg")),
        "phoenix-uslkg-10n" => Some(("3003923", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-uslkg-10-n-3003923", "https://caas.phoenixcontact.com/caas/v1/stable/media/120132/full/b408?format=jpg")),
        "phoenix-uslkg-16n" => Some(("0443023", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-uslkg-16-n-0443023", "https://caas.phoenixcontact.com/caas/v1/stable/media/194534/full/b408?format=jpg")),
        "phoenix-pt-25-dio-lr" => Some(("3210224", "https://www.phoenixcontact.com/de-de/produkte/bauelementeklemme-pt-25-diol-r-3210224", "https://caas.phoenixcontact.com/caas/v1/stable/media/246790/full/b408?format=jpg")),
        "phoenix-pt-25-quattro-dio-lr" => Some(("3210266", "https://www.phoenixcontact.com/de-de/produkte/bauelementeklemme-pt-25-quattro-diol-r-3210266", "https://caas.phoenixcontact.com/caas/v1/stable/media/248648/full/b408?format=jpg")),
        "phoenix-pttb-25-dio-ou" => Some(("3210923", "https://www.phoenixcontact.com/de-de/produkte/bauelementeklemme-pttb-25-dioo-u-3210923", "https://caas.phoenixcontact.com/caas/v1/stable/media/195678/full/b408?format=jpg")),
        "phoenix-pttb-25-la-24-rd" => Some(("3211456", "https://www.phoenixcontact.com/de-de/produkte/bauelementeklemme-pttb-25-la-24-rd-3211456", "https://caas.phoenixcontact.com/caas/v1/stable/media/116252/full/b408?format=jpg")),
        "phoenix-pt-6-hesi" => Some(("3211870", "https://www.phoenixcontact.com/de-de/produkte/sicherungsklemme-pt-6-hesi-63x32-3211870", "https://caas.phoenixcontact.com/caas/v1/stable/media/197342/full/b408?format=jpg")),
        "phoenix-pt-6-hesiled-24" => Some(("3211874", "https://www.phoenixcontact.com/de-de/produkte/sicherungsklemme-pt-6-hesiled-24-63x32-3211874", "https://caas.phoenixcontact.com/caas/v1/stable/media/197342/full/b408?format=jpg")),
        "phoenix-ptfix-6-18x25-gy" => Some(("3273374", "https://www.phoenixcontact.com/de-de/produkte/verteilerblock-ptfix-618x25-gy-3273374", "https://caas.phoenixcontact.com/caas/v1/stable/media/261878/full/b408?format=jpg")),
        "phoenix-ptfix-6-18x25-bu" => Some(("3273376", "https://www.phoenixcontact.com/de-de/produkte/verteilerblock-ptfix-618x25-bu-3273376", "https://caas.phoenixcontact.com/caas/v1/stable/media/213524/full/b408?format=jpg")),
        "phoenix-ptfix-6-18x25-pe" => Some(("1091670", "https://www.phoenixcontact.com/de-de/produkte/verteilerblock-ptfix-618x25-gnye-1091670", "https://caas.phoenixcontact.com/caas/v1/stable/media/31564/full/b408?format=jpg")),
        "phoenix-ptfix-6-12x25-gy" => Some(("3273352", "https://www.phoenixcontact.com/de-de/produkte/verteilerblock-ptfix-612x25-gy-3273352", "https://caas.phoenixcontact.com/caas/v1/stable/media/213370/full/b408?format=jpg")),
        "phoenix-ptfix-6-6x25-gy" => Some(("3273330", "https://www.phoenixcontact.com/de-de/produkte/verteilerblock-ptfix-66x25-gy-3273330", "https://caas.phoenixcontact.com/caas/v1/stable/media/259958/full/b408?format=jpg")),
        "phoenix-ptrv-4-whrd" => Some(("3270115", "https://www.phoenixcontact.com/de-de/produkte/rangierklemme-ptrv-4-wh-3270115", "https://caas.phoenixcontact.com/caas/v1/stable/media/228894/full/b408?format=jpg")),
        "phoenix-ptrv-8-whrd" => Some(("3270133", "https://www.phoenixcontact.com/de-de/produkte/rangierklemme-ptrv-8-whrd-3270133", "https://caas.phoenixcontact.com/caas/v1/stable/media/253528/full/b408?format=jpg")),
        "phoenix-pt-15s-twin-bu" => Some(("3208168", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-15s-twin-bu-3208168", "https://caas.phoenixcontact.com/caas/v1/stable/media/229710/full/b408?format=jpg")),
        "phoenix-pt-15s-quattro-bu" => Some(("3208208", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-15s-quattro-bu-3208208", "https://caas.phoenixcontact.com/caas/v1/stable/media/230962/full/b408?format=jpg")),
        "phoenix-pt-15s-quattro-pe" => Some(("3208333", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-15s-quattro-pe-3208333", "https://caas.phoenixcontact.com/caas/v1/stable/media/210624/full/b408?format=jpg")),
        "phoenix-pt-15s-tg" => Some(("3210306", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-15s-tg-3210306", "https://caas.phoenixcontact.com/caas/v1/stable/media/260742/full/b408?format=jpg")),
        "phoenix-pttb-15s-bu" => Some(("3208524", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-15s-bu-3208524", "https://caas.phoenixcontact.com/caas/v1/stable/media/269362/full/b408?format=jpg")),
        "phoenix-pttb-15s-pv" => Some(("3208540", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-15s-pv-3208540", "https://caas.phoenixcontact.com/caas/v1/stable/media/257880/full/b408?format=jpg")),
        "phoenix-pttb-15s-ln" => Some(("3208544", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-15s-ln-3208544", "https://caas.phoenixcontact.com/caas/v1/stable/media/215302/full/b408?format=jpg")),
        "phoenix-pttb-15s-pel" => Some(("3208553", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pttb-15s-pel-3208553", "https://caas.phoenixcontact.com/caas/v1/stable/media/256278/full/b408?format=jpg")),
        "phoenix-pt-15s-3pv" => Some(("3213742", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pt-15s-3pv-3213742", "https://caas.phoenixcontact.com/caas/v1/stable/media/261398/full/b408?format=jpg")),
        "phoenix-pt-15s-peln" => Some(("3213755", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-15s-peln-3213755", "https://caas.phoenixcontact.com/caas/v1/stable/media/230452/full/b408?format=jpg")),
        "phoenix-pt-15s-pell" => Some(("3213768", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-15s-pell-3213768", "https://caas.phoenixcontact.com/caas/v1/stable/media/209516/full/b408?format=jpg")),
        "phoenix-ptio-15s-4-pe" => Some(("3244465", "https://www.phoenixcontact.com/de-de/produkte/sensor-aktor-klemme-ptio-15s4-pe-3244465", "https://caas.phoenixcontact.com/caas/v1/stable/media/222960/full/b408?format=jpg")),
        "phoenix-ptio-in-25-4-pe" => Some(("3244481", "https://www.phoenixcontact.com/de-de/produkte/einspeiseklemme-ptio-in-254-pe-og-3244481", "https://caas.phoenixcontact.com/caas/v1/stable/media/212312/full/b408?format=jpg")),
        "phoenix-pttb-25-pv" => Some(("3210583", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-25-pv-3210583", "https://caas.phoenixcontact.com/caas/v1/stable/media/227492/full/b408?format=jpg")),
        "phoenix-pttb-25-ln-real" => Some(("3210994", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-25-ln-3210994", "https://caas.phoenixcontact.com/caas/v1/stable/media/299412/full/b408?format=jpg")),
        "phoenix-pt-25-3pv" => Some(("3210512", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pt-25-3pv-3210512", "https://caas.phoenixcontact.com/caas/v1/stable/media/241904/full/b408?format=jpg")),
        "phoenix-pttbs-25-bu" => Some(("3209617", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttbs-25-bu-3209617", "https://caas.phoenixcontact.com/caas/v1/stable/media/240900/full/b408?format=jpg")),
        "phoenix-pttbs-25-pe" => Some(("3209620", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pttbs-25-pe-3209620", "https://caas.phoenixcontact.com/caas/v1/stable/media/195934/full/b408?format=jpg")),
        "phoenix-pttbs-25-pv" => Some(("3210211", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttbs-25-pv-3210211", "https://caas.phoenixcontact.com/caas/v1/stable/media/249682/full/b408?format=jpg")),
        "phoenix-pt-25-twin-mt" => Some(("3210169", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-25-twin-mt-3210169", "https://caas.phoenixcontact.com/caas/v1/stable/media/11860/full/b408?format=jpg")),
        "phoenix-pt-25-quattro-mt" => Some(("3210172", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-25-quattro-mt-3210172", "https://caas.phoenixcontact.com/caas/v1/stable/media/11862/full/b408?format=jpg")),
        "phoenix-pt-25-twin-tg" => Some(("3210198", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-25-twin-tg-3210198", "https://caas.phoenixcontact.com/caas/v1/stable/media/31718/full/b408?format=jpg")),
        "phoenix-pt-4-twin-bu" => Some(("3211775", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-4-twin-bu-3211775", "https://caas.phoenixcontact.com/caas/v1/stable/media/247772/full/b408?format=jpg")),
        "phoenix-pt-4-quattro-bu" => Some(("3211802", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-4-quattro-bu-3211802", "https://caas.phoenixcontact.com/caas/v1/stable/media/218034/full/b408?format=jpg")),
        "phoenix-pt-4-quattro-pe" => Some(("3211809", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-4-quattro-pe-3211809", "https://caas.phoenixcontact.com/caas/v1/stable/media/199548/full/b408?format=jpg")),
        "phoenix-pttb-4-pv" => Some(("3211825", "https://www.phoenixcontact.com/de-de/produkte/mehrstockklemme-pttb-4-pv-3211825", "https://caas.phoenixcontact.com/caas/v1/stable/media/269016/full/b408?format=jpg")),
        "phoenix-pttb-4-pel" => Some(("3211857", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pttb-4-pel-3211857", "https://caas.phoenixcontact.com/caas/v1/stable/media/262544/full/b408?format=jpg")),
        "phoenix-pt-4-tg" => Some(("3211922", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-pt-4-tg-3211922", "https://caas.phoenixcontact.com/caas/v1/stable/media/28300/full/b408?format=jpg")),
        "phoenix-pt-4-hesiled-24" => Some(("3211903", "https://www.phoenixcontact.com/de-de/produkte/sicherungsklemme-pt-4-hesiled-24-5x20-3211903", "https://caas.phoenixcontact.com/caas/v1/stable/media/28936/full/b408?format=jpg")),
        "phoenix-pt-4-hesila-250" => Some(("3211907", "https://www.phoenixcontact.com/de-de/produkte/sicherungsklemme-pt-4-hesila-250-5x20-3211907", "https://caas.phoenixcontact.com/caas/v1/stable/media/251788/full/b408?format=jpg")),
        "phoenix-pt-6-twin" => Some(("3211929", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-6-twin-3211929", "https://caas.phoenixcontact.com/caas/v1/stable/media/236628/full/b408?format=jpg")),
        "phoenix-pt-6-twin-bu" => Some(("3211485", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-6-twin-bu-3211485", "https://caas.phoenixcontact.com/caas/v1/stable/media/218040/full/b408?format=jpg")),
        "phoenix-pt-6-twin-pe" => Some(("3211498", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-6-twin-pe-3211498", "https://caas.phoenixcontact.com/caas/v1/stable/media/110472/full/b408?format=jpg")),
        "phoenix-pt-6-quattro" => Some(("3212934", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-6-quattro-3212934", "https://caas.phoenixcontact.com/caas/v1/stable/media/3124/full/b408?format=jpg")),
        "phoenix-pt-6-quattro-bu" => Some(("3212947", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-6-quattro-bu-3212947", "https://caas.phoenixcontact.com/caas/v1/stable/media/217746/full/b408?format=jpg")),
        "phoenix-pt-6-quattro-pe" => Some(("3212950", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-6-quattro-pe-3212950", "https://caas.phoenixcontact.com/caas/v1/stable/media/122568/full/b408?format=jpg")),
        "phoenix-ptmed-6" => Some(("3212183", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ptmed-6-3212183", "https://caas.phoenixcontact.com/caas/v1/stable/media/259752/full/b408?format=jpg")),
        "phoenix-pt-10-bu" => Some(("3212123", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-10-bu-3212123", "https://caas.phoenixcontact.com/caas/v1/stable/media/267408/full/b408?format=jpg")),
        "phoenix-pt-10-pe" => Some(("3212131", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-10-pe-3212131", "https://caas.phoenixcontact.com/caas/v1/stable/media/252716/full/b408?format=jpg")),
        "phoenix-pt-10-twin" => Some(("3208746", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-10-twin-3208746", "https://caas.phoenixcontact.com/caas/v1/stable/media/233104/full/b408?format=jpg")),
        "phoenix-pt-10-twin-bu" => Some(("3208747", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-10-twin-bu-3208747", "https://caas.phoenixcontact.com/caas/v1/stable/media/228292/full/b408?format=jpg")),
        "phoenix-pt-10-twin-pe" => Some(("3208745", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-10-twin-pe-3208745", "https://caas.phoenixcontact.com/caas/v1/stable/media/195850/full/b408?format=jpg")),
        "phoenix-pt-16n-bu" => Some(("3212142", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-pt-16-n-bu-3212142", "https://caas.phoenixcontact.com/caas/v1/stable/media/232012/full/b408?format=jpg")),
        "phoenix-pt-16n-pe" => Some(("3212147", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-16-n-pe-3212147", "https://caas.phoenixcontact.com/caas/v1/stable/media/223254/full/b408?format=jpg")),
        "phoenix-pt-16-twin-n" => Some(("3208760", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-16-twin-n-3208760", "https://caas.phoenixcontact.com/caas/v1/stable/media/196826/full/b408?format=jpg")),
        "phoenix-pt-16-twin-n-bu" => Some(("3208773", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-pt-16-twin-n-bu-3208773", "https://caas.phoenixcontact.com/caas/v1/stable/media/200852/full/b408?format=jpg")),
        "phoenix-pt-16-twin-n-pe" => Some(("3208786", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pt-16-twin-n-pe-3208786", "https://caas.phoenixcontact.com/caas/v1/stable/media/199888/full/b408?format=jpg")),
        "phoenix-ptpower-35-bu" => Some(("3212065", "https://www.phoenixcontact.com/de-de/produkte/hochstromklemme-ptpower-35-bu-3212065", "https://caas.phoenixcontact.com/caas/v1/stable/media/206310/full/b408?format=jpg")),
        "phoenix-ptpower-35-pe" => Some(("3212066", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ptpower-35-pe-3212066", "https://caas.phoenixcontact.com/caas/v1/stable/media/272776/full/b408?format=jpg")),
        "phoenix-ptpower-50-bu" => Some(("3260051", "https://www.phoenixcontact.com/de-de/produkte/hochstromklemme-ptpower-50-bu-3260051", "https://caas.phoenixcontact.com/caas/v1/stable/media/268004/full/b408?format=jpg")),
        "phoenix-ptpower-50-pe" => Some(("3260052", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ptpower-50-pe-3260052", "https://caas.phoenixcontact.com/caas/v1/stable/media/256866/full/b408?format=jpg")),
        "phoenix-ptpower-95-bu" => Some(("3260103", "https://www.phoenixcontact.com/de-de/produkte/hochstromklemme-ptpower-95-bu-3260103", "https://caas.phoenixcontact.com/caas/v1/stable/media/268004/full/b408?format=jpg")),
        "phoenix-ptpower-95-pe" => Some(("3260106", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ptpower-95-pe-3260106", "https://caas.phoenixcontact.com/caas/v1/stable/media/256866/full/b408?format=jpg")),
        "phoenix-ptpower-185" => Some(("1054722", "https://www.phoenixcontact.com/de-de/produkte/hochstromklemme-ptpower-185-1054722", "https://caas.phoenixcontact.com/caas/v1/stable/media/223192/full/b408?format=jpg")),
        "phoenix-pti-25" => Some(("3213968", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-25-3213968", "https://caas.phoenixcontact.com/caas/v1/stable/media/227396/full/b408?format=jpg")),
        "phoenix-pti-25-bu" => Some(("3213969", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-25-bu-3213969", "https://caas.phoenixcontact.com/caas/v1/stable/media/234260/full/b408?format=jpg")),
        "phoenix-pti-25-pe" => Some(("3213962", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-25-pe-3213962", "https://caas.phoenixcontact.com/caas/v1/stable/media/228004/full/b408?format=jpg")),
        "phoenix-pti-4" => Some(("3213970", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-4-3213970", "https://caas.phoenixcontact.com/caas/v1/stable/media/230344/full/b408?format=jpg")),
        "phoenix-pti-4-bu" => Some(("3213971", "https://www.phoenixcontact.com/de-de/produkte/installationsklemme-pti-4-bu-3213971", "https://caas.phoenixcontact.com/caas/v1/stable/media/243144/full/b408?format=jpg")),
        "phoenix-pti-4-pe" => Some(("3213964", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-pti-4-pe-3213964", "https://caas.phoenixcontact.com/caas/v1/stable/media/198618/full/b408?format=jpg")),
        "phoenix-ptv-25-twin" => Some(("1078966", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ptv-25-twin-1078966", "https://caas.phoenixcontact.com/caas/v1/stable/media/109158/full/b408?format=jpg")),
        "phoenix-ptv-25-twin-bu" => Some(("1078971", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ptv-25-twin-bu-1078971", "https://caas.phoenixcontact.com/caas/v1/stable/media/109572/full/b408?format=jpg")),
        "phoenix-ptv-25-twin-pe" => Some(("1078991", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ptv-25-twin-pe-1078991", "https://caas.phoenixcontact.com/caas/v1/stable/media/217504/full/b408?format=jpg")),
        "phoenix-ptv-25-quattro" => Some(("1078999", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ptv-25-quattro-1078999", "https://caas.phoenixcontact.com/caas/v1/stable/media/117444/full/b408?format=jpg")),
        "phoenix-ptv-25-quattro-bu" => Some(("1079006", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ptv-25-quattro-bu-1079006", "https://caas.phoenixcontact.com/caas/v1/stable/media/109890/full/b408?format=jpg")),
        "phoenix-ptv-25-quattro-pe" => Some(("1079012", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ptv-25-quattro-pe-1079012", "https://caas.phoenixcontact.com/caas/v1/stable/media/118504/full/b408?format=jpg")),
        "phoenix-ptv-4-bu" => Some(("1088729", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ptv-4-bu-1088729", "https://caas.phoenixcontact.com/caas/v1/stable/media/124224/full/b408?format=jpg")),
        "phoenix-ptv-4-twin" => Some(("1088731", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ptv-4-twin-1088731", "https://caas.phoenixcontact.com/caas/v1/stable/media/269888/full/b408?format=jpg")),
        "phoenix-ptv-4-quattro" => Some(("1088734", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ptv-4-quattro-1088734", "https://caas.phoenixcontact.com/caas/v1/stable/media/213970/full/b408?format=jpg")),
        "phoenix-st-25-twin" => Some(("3031241", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-st-25-twin-3031241", "https://caas.phoenixcontact.com/caas/v1/stable/media/216566/full/b408?format=jpg")),
        "phoenix-st-25-quattro" => Some(("3031306", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-st-25-quattro-3031306", "https://caas.phoenixcontact.com/caas/v1/stable/media/112122/full/b408?format=jpg")),
        "phoenix-st-4" => Some(("3031364", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-st-4-3031364", "https://caas.phoenixcontact.com/caas/v1/stable/media/231736/full/b408?format=jpg")),
        "phoenix-st-4-bu" => Some(("3031377", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-st-4-bu-3031377", "https://caas.phoenixcontact.com/caas/v1/stable/media/268640/full/b408?format=jpg")),
        "phoenix-st-4-pe" => Some(("3031380", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-st-4-pe-3031380", "https://caas.phoenixcontact.com/caas/v1/stable/media/236602/full/b408?format=jpg")),
        "phoenix-ut-25-bu" => Some(("3044089", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-25-bu-3044089", "https://caas.phoenixcontact.com/caas/v1/stable/media/222646/full/b408?format=jpg")),
        "phoenix-ut-25-twin" => Some(("3044513", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ut-25-twin-3044513", "https://caas.phoenixcontact.com/caas/v1/stable/media/234988/full/b408?format=jpg")),
        "phoenix-ut-25-quattro" => Some(("3044542", "https://www.phoenixcontact.com/de-de/produkte/mehrleiterklemme-ut-25-quattro-3044542", "https://caas.phoenixcontact.com/caas/v1/stable/media/269886/full/b408?format=jpg")),
        "phoenix-ut-25-mt" => Some(("3046362", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-ut-25-mt-3046362", "https://caas.phoenixcontact.com/caas/v1/stable/media/252840/full/b408?format=jpg")),
        "phoenix-ut-4-bu" => Some(("3044115", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-4-bu-3044115", "https://caas.phoenixcontact.com/caas/v1/stable/media/254566/full/b408?format=jpg")),
        "phoenix-ut-4-pe" => Some(("3044128", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ut-4-pe-3044128", "https://caas.phoenixcontact.com/caas/v1/stable/media/253902/full/b408?format=jpg")),
        "phoenix-ut-4-mt" => Some(("3046139", "https://www.phoenixcontact.com/de-de/produkte/trennklemme-ut-4-mt-3046139", "https://caas.phoenixcontact.com/caas/v1/stable/media/222934/full/b408?format=jpg")),
        "phoenix-ut-6-bu" => Some(("3044144", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-6-bu-3044144", "https://caas.phoenixcontact.com/caas/v1/stable/media/194306/full/b408?format=jpg")),
        "phoenix-ut-6-pe" => Some(("3044157", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ut-6-pe-3044157", "https://caas.phoenixcontact.com/caas/v1/stable/media/212788/full/b408?format=jpg")),
        "phoenix-ut-10-pe" => Some(("3044173", "https://www.phoenixcontact.com/de-de/produkte/schutzleiterklemme-ut-10-pe-3044173", "https://caas.phoenixcontact.com/caas/v1/stable/media/256890/full/b408?format=jpg")),
        "phoenix-ut-35" => Some(("3044225", "https://www.phoenixcontact.com/de-de/produkte/durchgangsklemme-ut-35-3044225", "https://caas.phoenixcontact.com/caas/v1/stable/media/220104/full/b408?format=jpg")),
        "phoenix-ab-pti-3" => Some(("3213974", "https://www.phoenixcontact.com/de-de/produkte/auflagebock-ab-pti3-3213974", "https://caas.phoenixcontact.com/caas/v1/stable/media/266030/full/b408?format=jpg")),
        "phoenix-ab-pti-4-3" => Some(("3214053", "https://www.phoenixcontact.com/de-de/produkte/auflagebock-ab-pti-43-3214053", "https://caas.phoenixcontact.com/caas/v1/stable/media/266030/full/b408?format=jpg")),
        "phoenix-ab-pti" => Some(("3214006", "https://www.phoenixcontact.com/de-de/produkte/auflagebock-ab-pti-3214006", "https://caas.phoenixcontact.com/caas/v1/stable/media/215234/full/b408?format=jpg")),
        "phoenix-ab-pti-6" => Some(("3214008", "https://www.phoenixcontact.com/de-de/produkte/auflagebock-ab-pti-6-3214008", "https://caas.phoenixcontact.com/caas/v1/stable/media/246774/full/b408?format=jpg")),
        "phoenix-ab-pti-16-nls-bu" => Some(("1030138", "https://www.phoenixcontact.com/de-de/produkte/auflagebock-ab-pti-16-nls-bu-1030138", "https://caas.phoenixcontact.com/caas/v1/stable/media/221880/full/b408?format=jpg")),
        "phoenix-ab-uti-3" => Some(("3076038", "https://www.phoenixcontact.com/de-de/produkte/auflagebock-ab-uti3-3076038", "https://caas.phoenixcontact.com/caas/v1/stable/media/270338/full/b408?format=jpg")),
        "phoenix-ab-uti-6-3" => Some(("1037092", "https://www.phoenixcontact.com/de-de/produkte/auflagebock-ab-uti-63-1037092", "https://caas.phoenixcontact.com/caas/v1/stable/media/202414/full/b408?format=jpg")),
        "phoenix-ab-sti-3" => Some(("3030831", "https://www.phoenixcontact.com/de-de/produkte/auflagebock-ab-sti3-3030831", "https://caas.phoenixcontact.com/caas/v1/stable/media/266030/full/b408?format=jpg")),
        "phoenix-ab-ss" => Some(("0404428", "https://www.phoenixcontact.com/de-de/produkte/auflagebock-abss-0404428", "https://caas.phoenixcontact.com/caas/v1/stable/media/244668/full/b408?format=jpg")),

        // --- WEIDMÜLLER ---
        "weidmuller-a2c-15" => Some(("1552790000", "https://eshop.weidmueller.com/de/a2c-1-5/p/1552790000", "https://assets.dam.weidmueller.com/assets/api/e97bfd80-8178-485e-b52e-869943c7b95c/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-15-bl" => Some(("1552820000", "https://eshop.weidmueller.com/de/p/1552820000", "https://assets.dam.weidmueller.com/assets/api/345b4a4b-ab00-4108-9cdf-2550036e02b0/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-15-pe" => Some(("1552680000", "https://eshop.weidmueller.com/de/a2c-1-5-pe/p/1552680000", "https://assets.dam.weidmueller.com/assets/api/24634b68-2c52-4973-a8b2-39f4369157f1/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3c-15" => Some(("1552740000", "https://eshop.weidmueller.com/de/p/1552740000", "https://assets.dam.weidmueller.com/assets/api/4f668d3d-a39d-4dc0-a02a-c00d0e905c96/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a4c-15" => Some(("1552690000", "https://eshop.weidmueller.com/de/p/1552690000", "https://assets.dam.weidmueller.com/assets/api/6c67882e-6bbd-4df2-9424-36e53b2b4dc1/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-25" => Some(("1521850000", "https://eshop.weidmueller.com/de/a2c-2-5/p/1521850000", "https://assets.dam.weidmueller.com/assets/api/2d76620f-5915-4c72-afd5-0fffe2de88ab/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-25-bl" => Some(("1521880000", "https://eshop.weidmueller.com/de/a2c-2-5-bl/p/1521880000", "https://assets.dam.weidmueller.com/assets/api/80a54677-0c4a-41a5-865f-fbd26c3ae934/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-25-pe" => Some(("1521680000", "https://eshop.weidmueller.com/de/a2c-2-5-pe/p/1521680000", "https://assets.dam.weidmueller.com/assets/api/e4f70efa-ed27-42a1-98c6-372ac81cbe30/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3c-25" => Some(("1521740000", "https://eshop.weidmueller.com/de/a3c-2-5/p/1521740000", "https://assets.dam.weidmueller.com/assets/api/ad53394d-84ad-4f47-91bd-49fcf385a87c/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3c-25-bl" => Some(("1521780000", "https://eshop.weidmueller.com/de/p/1521780000", "https://assets.dam.weidmueller.com/assets/api/5309dea0-fee1-4b9e-a05d-819ebcb01d25/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3c-25-pe" => Some(("1521670000", "https://eshop.weidmueller.com/de/p/1521670000", "https://assets.dam.weidmueller.com/assets/api/5a311589-83bc-4691-8d98-de63b58f3e0a/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a4c-25" => Some(("1521690000", "https://eshop.weidmueller.com/de/a4c-2-5/p/1521690000", "https://assets.dam.weidmueller.com/assets/api/88ff371f-0680-453c-b9b7-a634a3667b96/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a4c-25-bl" => Some(("1521700000", "https://eshop.weidmueller.com/de/p/1521700000", "https://assets.dam.weidmueller.com/assets/api/1a80dde1-cd0e-47cb-a597-96a3c85c1261/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a4c-25-pe" => Some(("1521540000", "https://eshop.weidmueller.com/de/p/1521540000", "https://assets.dam.weidmueller.com/assets/api/6eccc40a-30a3-43b3-b225-e878dcd0c511/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2t-25" => Some(("1547610000", "https://eshop.weidmueller.com/de/a2t-2-5/p/1547610000", "https://assets.dam.weidmueller.com/assets/api/d2054db7-d5a2-42b1-80ce-c5e41a701c86/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2t-25-vl" => Some(("1547650000", "https://eshop.weidmueller.com/de/a2t-2-5-vl/p/1547650000", "https://assets.dam.weidmueller.com/assets/api/bb0f8286-f1e2-422e-b330-66438736117e/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2t-25-pe" => Some(("1547680000", "https://eshop.weidmueller.com/de/p/1547680000", "https://assets.dam.weidmueller.com/assets/api/b23fd235-4033-47c3-9397-303be60d6571/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2t-25-ft-pe" => Some(("1547640000", "https://eshop.weidmueller.com/de/p/1547640000", "https://assets.dam.weidmueller.com/assets/api/b9b3e387-1119-4af8-bdcd-3b35656cc46e/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3t-25" => Some(("2428510000", "https://eshop.weidmueller.com/de/a3t-2-5/p/2428510000", "https://assets.dam.weidmueller.com/assets/api/b22971ca-7cfb-40f7-b90e-89872195a387/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3t-25-pe" => Some(("2428550000", "https://eshop.weidmueller.com/de/p/2428550000", "https://assets.dam.weidmueller.com/assets/api/6684e190-ab38-4f22-aa3e-4c0279b01f81/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3t-25-peln" => Some(("2428840000", "https://eshop.weidmueller.com/de/p/2428840000", "https://assets.dam.weidmueller.com/assets/api/76a22161-d61e-4065-acec-a7d2dbe800a6/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-4" => Some(("2051180000", "https://eshop.weidmueller.com/de/a2c-4/p/2051180000", "https://assets.dam.weidmueller.com/assets/api/00481506-1083-4277-a279-c18ba32d70ec/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-4-bl" => Some(("2051210000", "https://eshop.weidmueller.com/de/a2c-4-bl/p/2051210000", "https://assets.dam.weidmueller.com/assets/api/f9392296-b10a-4620-a07b-c5d4c5d53eb0/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-4-pe" => Some(("2051360000", "https://eshop.weidmueller.com/de/a2c-4-pe/p/2051360000", "https://assets.dam.weidmueller.com/assets/api/28026b79-d38b-406a-9bd6-b853f9a1fdf8/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3c-4" => Some(("2051240000", "https://eshop.weidmueller.com/de/p/2051240000", "https://assets.dam.weidmueller.com/assets/api/0c59bc12-c4ec-45c2-851f-358555188e29/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a3c-4-pe" => Some(("2051410000", "https://eshop.weidmueller.com/de/p/2051410000", "https://assets.dam.weidmueller.com/assets/api/23a791c3-bc66-4560-a61b-65c2f53656f1/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2t-4" => Some(("2539970000", "https://eshop.weidmueller.com/de/a2t-4/p/2539970000", "https://assets.dam.weidmueller.com/assets/api/5cda7b26-655c-46d3-bb03-5e43816e57ac/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-6" => Some(("1992110000", "https://eshop.weidmueller.com/de/a2c-6/p/1992110000", "https://assets.dam.weidmueller.com/assets/api/73f75663-597b-42df-8b20-cfeb0d8e26c6/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-6-bl" => Some(("1991790000", "https://eshop.weidmueller.com/de/p/1991790000", "https://assets.dam.weidmueller.com/assets/api/70f73856-eadb-4870-b05f-fea5ba6b712b/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-6-pe" => Some(("1991810000", "https://eshop.weidmueller.com/de/a2c-6-pe/p/1991810000", "https://assets.dam.weidmueller.com/assets/api/27c0e626-d361-4552-98e5-3a883e588c43/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-10" => Some(("2490360000", "https://eshop.weidmueller.com/de/a2c-10/p/2490360000", "https://assets.dam.weidmueller.com/assets/api/0c4ad510-744d-4b30-be81-d4c6af5fbf5f/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-10-bl" => Some(("2490370000", "https://eshop.weidmueller.com/de/p/2490370000", "https://assets.dam.weidmueller.com/assets/api/0619e804-ba26-41b6-883d-8b4b9d171449/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-10-pe" => Some(("2490440000", "https://eshop.weidmueller.com/de/p/2490440000", "https://assets.dam.weidmueller.com/assets/api/4fcef12e-a9e0-43d8-a3a7-61f6984df258/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-16" => Some(("2494000000", "https://eshop.weidmueller.com/de/a2c-16/p/2494000000", "https://assets.dam.weidmueller.com/assets/api/b095a3d6-2f03-431c-b533-183241e1913c/DAP_Master_PP,w_800,q_80")),
        "weidmuller-a2c-35" => Some(("2551510000", "https://eshop.weidmueller.com/de/a2c-35/p/2551510000", "https://assets.dam.weidmueller.com/assets/api/0821071c-e508-4d31-bd3a-acc38d69e453/DAP_Master_PP,w_800,q_80")),
        "weidmuller-amc-25" => Some(("2434340000", "https://eshop.weidmueller.com/de/p/2434340000", "https://assets.dam.weidmueller.com/assets/api/617cbd15-c168-4f24-b8f7-657d3c3c19b5/DAP_Master_PP,w_800,q_80")),
        "weidmuller-adt-25" => Some(("1989800000", "https://eshop.weidmueller.com/de/p/1989800000", "https://assets.dam.weidmueller.com/assets/api/2e5c1fb3-49d7-40a1-9ac1-516548bcdcdb/DAP_Master_PP,w_800,q_80")),
        "weidmuller-afs-4" => Some(("2429860000", "https://eshop.weidmueller.com/de/p/2429860000", "https://assets.dam.weidmueller.com/assets/api/7c4b5800-6c04-4b32-a267-63f4c09c89c6/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aio21" => Some(("1992260000", "https://eshop.weidmueller.com/de/p/1992260000", "https://assets.dam.weidmueller.com/assets/api/07381d74-59de-4526-8590-ef8cfab48422/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-16-ndt-bl" => Some(("2668770000", "https://eshop.weidmueller.com/de/p/2668770000", "https://assets.dam.weidmueller.com/assets/api/49458f66-30a0-4a55-ae92-01e97380e2bc/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-16-nlo-bl" => Some(("2669140000", "https://eshop.weidmueller.com/de/p/2669140000", "https://assets.dam.weidmueller.com/assets/api/d577c648-f962-4ac6-b5ca-3bba4407791a/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-25-ntlp" => Some(("2668950000", "https://eshop.weidmueller.com/de/p/2668950000", "https://assets.dam.weidmueller.com/assets/api/3d42f558-d204-4df1-a36e-6e08cb5ee8c4/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-25-nlp" => Some(("2668940000", "https://eshop.weidmueller.com/de/aitb-2-5-bb-n-l-pe/p/2668940000", "https://assets.dam.weidmueller.com/assets/api/e281c782-7fa1-4d4f-b663-47c8fcd43a7a/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-25-llp" => Some(("2668930000", "https://eshop.weidmueller.com/de/p/2668930000", "https://assets.dam.weidmueller.com/assets/api/2ad6a413-ed9b-4f79-a324-39d52a0fadac/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-25-ll" => Some(("2668900000", "https://eshop.weidmueller.com/de/aitb-2-5-bb-l-l/p/2668900000", "https://assets.dam.weidmueller.com/assets/api/86f00180-e755-46fb-85bb-e06ab6d46105/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-25-nl" => Some(("2668910000", "https://eshop.weidmueller.com/de/aitb-2-5-bb-n-l/p/2668910000", "https://assets.dam.weidmueller.com/assets/api/753f2a48-b0d4-45c4-9ac2-5504a73d46e4/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-4-ntlp" => Some(("2669100000", "https://eshop.weidmueller.com/de/p/2669100000", "https://assets.dam.weidmueller.com/assets/api/621fda14-4edb-4320-8150-13e7e29aefb2/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-4-nlp" => Some(("2669090000", "https://eshop.weidmueller.com/de/aitb-4-bb-n-l-pe/p/2669090000", "https://assets.dam.weidmueller.com/assets/api/d763a83c-329c-4358-bb01-a2dffa7b157b/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-4-llp" => Some(("2669080000", "https://eshop.weidmueller.com/de/aitb-4-bb-l-l-pe/p/2669080000", "https://assets.dam.weidmueller.com/assets/api/ad0808fe-accb-4a1b-ae29-c579cd18f80b/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aitb-4-ll" => Some(("2669070000", "https://eshop.weidmueller.com/de/aitb-4-bb-l-l/p/2669070000", "https://assets.dam.weidmueller.com/assets/api/be6ef554-5ced-4dc5-b3d7-63466fe9e200/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aeb-itb-25-4-bl" => Some(("2690440000", "https://eshop.weidmueller.com/de/aeb-itb-2-5-4-bb-bl/p/2690440000", "https://assets.dam.weidmueller.com/assets/api/dba90e6f-61e6-4823-86bc-39b5adada8eb/DAP_Master_PP,w_800,q_80")),
        "weidmuller-aeb-itb-16-bl" => Some(("2669130000", "https://eshop.weidmueller.com/de/aeb-itb-16-bb-bl/p/2669130000", "https://assets.dam.weidmueller.com/assets/api/242abe79-710f-4754-a294-2bdfa167741f/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-25" => Some(("1020000000", "https://eshop.weidmueller.com/de/wdu-2-5/p/1020000000", "https://assets.dam.weidmueller.com/assets/api/39e06f09-d8ee-4912-beb4-700715a2acf1/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-25-bl" => Some(("1020080000", "https://eshop.weidmueller.com/de/wdu-2-5-bl/p/1020080000", "https://assets.dam.weidmueller.com/assets/api/953044f8-26dc-4809-87f3-3d76eee0107b/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wpe-25" => Some(("1010000000", "https://eshop.weidmueller.com/de/wpe-2-5/p/1010000000", "https://assets.dam.weidmueller.com/assets/api/8fdcddfc-2a0e-47db-8a7b-025cd551ecce/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-4" => Some(("1020100000", "https://eshop.weidmueller.com/de/wdu-4/p/1020100000", "https://assets.dam.weidmueller.com/assets/api/14db5102-fbfb-4b13-bcd3-afb8cdf5e16d/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-4-bl" => Some(("1020180000", "https://eshop.weidmueller.com/de/wdu-4-bl/p/1020180000", "https://assets.dam.weidmueller.com/assets/api/243b3997-cbb2-4996-8161-77f5ab935a69/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wpe-4" => Some(("1010100000", "https://eshop.weidmueller.com/de/wpe-4/p/1010100000", "https://assets.dam.weidmueller.com/assets/api/06ebc68a-59a5-4aef-ad33-d664be533d82/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-6" => Some(("1020200000", "https://eshop.weidmueller.com/de/wdu-6/p/1020200000", "https://assets.dam.weidmueller.com/assets/api/748df442-2268-4cc8-a823-c247cb16b75c/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-6-bl" => Some(("1020280000", "https://eshop.weidmueller.com/de/p/1020280000", "https://assets.dam.weidmueller.com/assets/api/e72e19b8-7e0e-4772-b47d-88004db03a7f/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wpe-6" => Some(("1010200000", "https://eshop.weidmueller.com/de/wpe-6/p/1010200000", "https://assets.dam.weidmueller.com/assets/api/4b44dec8-d063-4a5d-8f7f-c182fcf8c356/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-10" => Some(("1020300000", "https://eshop.weidmueller.com/de/wdu-10/p/1020300000", "https://assets.dam.weidmueller.com/assets/api/9819ba81-56c6-48bf-9104-ecf827e950a8/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-16" => Some(("1020400000", "https://eshop.weidmueller.com/de/wdu-16/p/1020400000", "https://assets.dam.weidmueller.com/assets/api/5c783010-f1ec-44e6-b6a8-bd24e8df60bc/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdu-35" => Some(("1020500000", "https://eshop.weidmueller.com/de/p/1020500000", "https://assets.dam.weidmueller.com/assets/api/04aa8d0b-1ae7-43b1-a6c3-c7ecd78e3136/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdk-25" => Some(("1021500000", "https://eshop.weidmueller.com/de/wdk-2-5/p/1021500000", "https://assets.dam.weidmueller.com/assets/api/e78a7db7-cca5-4f7e-b43e-93c9d20a79ab/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdk-25-n" => Some(("1021580000", "https://eshop.weidmueller.com/de/p/1021580000", "https://assets.dam.weidmueller.com/assets/api/481bad65-c0c9-481d-869a-06f5852ad03e/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wdk-25-pe" => Some(("1036300000", "https://eshop.weidmueller.com/de/p/1036300000", "https://assets.dam.weidmueller.com/assets/api/cd0d7674-5d31-49bd-9ba8-32a120b009c2/DAP_Master_PP,w_800,q_80")),
        "weidmuller-dli-25" => Some(("1783820000", "https://eshop.weidmueller.com/de/dli-2-5/p/1783820000", "https://assets.dam.weidmueller.com/assets/api/986a5bb7-ca57-4e14-9c59-9fc5492655c6/DAP_Master_PP,w_800,q_80")),
        "weidmuller-dld-25-pe" => Some(("1783800000", "https://eshop.weidmueller.com/de/dld-2-5-pe/p/1783800000", "https://assets.dam.weidmueller.com/assets/api/5ac514f3-c02e-49ff-bbfc-238ce1567814/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wtr-25" => Some(("1855610000", "https://eshop.weidmueller.com/de/p/1855610000", "https://assets.dam.weidmueller.com/assets/api/908ccf95-2dca-4a86-91d7-b68d52aa0fde/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wtr-4" => Some(("7910180000", "https://eshop.weidmueller.com/de/p/7910180000", "https://assets.dam.weidmueller.com/assets/api/8406441f-705c-40d9-9e4f-69a134714f07/DAP_Master_PP,w_800,q_80")),
        "weidmuller-wsi-6" => Some(("1011000000", "https://eshop.weidmueller.com/de/p/1011000000", "https://assets.dam.weidmueller.com/assets/api/b53a71f7-6f24-4252-89ce-b718e0d8cd6b/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zdu-25" => Some(("1608510000", "https://eshop.weidmueller.com/de/zdu-2-5/p/1608510000", "https://assets.dam.weidmueller.com/assets/api/bb83d5f1-0304-4ee6-baf9-aefd6df87aec/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zpe-25" => Some(("1608640000", "https://eshop.weidmueller.com/de/zpe-2-5/p/1608640000", "https://assets.dam.weidmueller.com/assets/api/20dbdc64-2c36-4806-89fe-28cd650bc6b8/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zdk-25" => Some(("1674300000", "https://eshop.weidmueller.com/de/zdk-2-5/p/1674300000", "https://assets.dam.weidmueller.com/assets/api/46401129-1a26-4206-825c-25fc0676e8bd/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zdk-25-v" => Some(("1689990000", "https://eshop.weidmueller.com/de/zdk-2-5-v/p/1689990000", "https://assets.dam.weidmueller.com/assets/api/50fdf618-f1d2-4ebb-8315-c3fa735b56cc/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zdk-25-pe" => Some(("1690000000", "https://eshop.weidmueller.com/de/p/1690000000", "https://assets.dam.weidmueller.com/assets/api/7bb6d52a-94c3-4a8b-bf85-8d3d572187ab/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zdu-4" => Some(("1632050000", "https://eshop.weidmueller.com/de/zdu-4/p/1632050000", "https://assets.dam.weidmueller.com/assets/api/9e643106-5f17-4374-b941-e9f49e6fa908/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zpe-4" => Some(("1632080000", "https://eshop.weidmueller.com/de/p/1632080000", "https://assets.dam.weidmueller.com/assets/api/cb13ee2b-3380-43a3-bbc3-0251aecb4568/DAP_Master_PP,w_800,q_80")),
        "weidmuller-zdl-25" => Some(("1720810000", "https://eshop.weidmueller.com/de/p/1720810000", "https://assets.dam.weidmueller.com/assets/api/82bcf06d-5440-4af9-ac17-b26bbcf1a965/DAP_Master_PP,w_800,q_80")),
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

/// Default Raw GitHub URL used to check and download terminal database updates without rebuilding the binary.
pub const GITHUB_TERMINAL_DB_URL: &str =
    "https://raw.githubusercontent.com/PatrickBarbian/TermiGrid/main/assets/terminal_specs.json";

/// Returns the active Raw GitHub URL for terminal database updates (can be overridden via `TERMIGRID_DB_URL`).
pub fn configured_terminal_db_url() -> String {
    if let Ok(custom) = std::env::var("TERMIGRID_DB_URL")
        && !custom.trim().is_empty()
    {
        return custom.trim().to_string();
    }
    GITHUB_TERMINAL_DB_URL.to_string()
}

/// Computes a deterministic 64-bit FNV-1a hex fingerprint across a list of `TerminalSpec`s.
pub fn canonical_db_hash(specs: &[TerminalSpec]) -> String {
    let mut sorted: Vec<&TerminalSpec> = specs.iter().collect();
    sorted.sort_by(|a, b| a.id.cmp(&b.id));
    let mut hash: u64 = 0xcbf29ce484222325;
    for sp in sorted {
        if let Ok(s) = serde_json::to_string(sp) {
            for b in s.as_bytes() {
                hash ^= u64::from(*b);
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
    }
    format!("{hash:016x}")
}

/// Parses and validates a raw terminal database JSON payload (either a top-level array `[ ... ]`
/// or a versioned object `{ "version": "...", "terminals": [ ... ] }`).
/// Rejects empty, malformed, or structurally invalid specs so a corrupt download can never break the app.
pub fn parse_and_validate_db_json(
    raw_json: &str,
) -> Result<(Vec<TerminalSpec>, Option<String>), String> {
    let trimmed = raw_json.trim().trim_start_matches('\u{feff}').trim();
    if trimmed.is_empty() {
        return Err("Leere JSON-Datenbank".to_string());
    }

    let val: serde_json::Value =
        serde_json::from_str(trimmed).map_err(|e| format!("Ungültiges JSON: {e}"))?;

    let (arr_val, version) = match val {
        serde_json::Value::Array(arr) => (arr, None),
        serde_json::Value::Object(mut map) => {
            let ver = map
                .get("version")
                .and_then(|v| v.as_str())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());
            let list = map
                .remove("terminals")
                .or_else(|| map.remove("specs"))
                .ok_or_else(|| {
                    "JSON-Objekt enthält kein 'terminals'- oder 'specs'-Array".to_string()
                })?;
            match list {
                serde_json::Value::Array(arr) => (arr, ver),
                _ => return Err("Feld 'terminals' ist kein JSON-Array".to_string()),
            }
        }
        _ => return Err("Erwartet wurde ein JSON-Array oder Datenbank-Objekt".to_string()),
    };

    if arr_val.is_empty() {
        return Err("Klemmen-Datenbank darf nicht leer sein".to_string());
    }

    let mut specs = Vec::with_capacity(arr_val.len());
    let mut seen_ids = std::collections::HashSet::with_capacity(arr_val.len());
    for (idx, item) in arr_val.into_iter().enumerate() {
        let spec: TerminalSpec = serde_json::from_value(item)
            .map_err(|e| format!("Fehler in Klemmen-Eintrag #{}: {e}", idx + 1))?;
        if spec.id.trim().is_empty()
            || spec.name.trim().is_empty()
            || spec.manufacturer.trim().is_empty()
            || spec.levels == 0
            || spec.width_tenths_mm == 0
            || spec.tiers.is_empty()
        {
            return Err(format!(
                "Ungültige Pflichtfelder in Klemmen-Eintrag #{} ({})",
                idx + 1,
                spec.id
            ));
        }
        if seen_ids.insert(spec.id.clone()) {
            specs.push(spec);
        }
    }

    Ok((specs, version))
}

/// In-memory catalog loaded from the local downloaded or embedded JSON terminal database.
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

    /// Loads the active terminal database with priority for a locally downloaded GitHub database
    /// (`default_downloaded_db_path()`), falling back to the embedded database when no downloaded
    /// file exists or if the local file is invalid.
    pub fn load_active() -> Self {
        let path = crate::model::library::default_downloaded_db_path();
        Self::load_from_db_path_or_embedded(&path)
    }

    /// Loads the terminal database from `db_path` if it exists and is valid (merging any embedded
    /// specs whose IDs are not present in `db_path` while giving `db_path` priority), or falls back
    /// to `Self::load_embedded()`.
    pub fn load_from_db_path_or_embedded(db_path: &std::path::Path) -> Self {
        if db_path.exists()
            && let Ok(raw) = std::fs::read_to_string(db_path)
            && let Ok((downloaded_specs, _)) = parse_and_validate_db_json(&raw)
        {
            let mut merged = downloaded_specs;
            let present_ids: std::collections::HashSet<String> =
                merged.iter().map(|s| s.id.clone()).collect();
            let embedded: Vec<TerminalSpec> = serde_json::from_str(EMBEDDED_SPECS_JSON)
                .expect("Embedded terminal_specs.json must be valid JSON");
            for emb in embedded {
                if !present_ids.contains(&emb.id) {
                    merged.push(emb);
                }
            }
            let mut catalog = Self { specs: merged };
            catalog.sort_by_manufacturer_and_type();
            return catalog;
        }
        Self::load_embedded()
    }

    /// Validates `raw_json` and atomically writes it to `db_path`. Returns the validated spec count.
    pub fn save_and_validate_downloaded_db(
        db_path: &std::path::Path,
        raw_json: &str,
    ) -> Result<usize, String> {
        let (specs, _) = parse_and_validate_db_json(raw_json)?;
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let tmp_path = db_path.with_extension("json.tmp");
        std::fs::write(&tmp_path, raw_json).map_err(|e| e.to_string())?;
        std::fs::rename(&tmp_path, db_path)
            .or_else(|_| std::fs::write(db_path, raw_json))
            .map_err(|e| e.to_string())?;
        Ok(specs.len())
    }

    /// Returns `(canonical_hash, spec_count, optional_version)` for the currently active base database
    /// (locally downloaded file if present and valid, otherwise the embedded database).
    pub fn active_base_db_fingerprint() -> (String, usize, Option<String>) {
        let path = crate::model::library::default_downloaded_db_path();
        Self::base_db_fingerprint_for_path(&path)
    }

    /// Returns `(canonical_hash, spec_count, optional_version)` for `db_path` if present and valid,
    /// otherwise for `EMBEDDED_SPECS_JSON`.
    pub fn base_db_fingerprint_for_path(
        db_path: &std::path::Path,
    ) -> (String, usize, Option<String>) {
        if db_path.exists()
            && let Ok(raw) = std::fs::read_to_string(db_path)
            && let Ok((specs, ver)) = parse_and_validate_db_json(&raw)
        {
            let h = canonical_db_hash(&specs);
            return (h, specs.len(), ver);
        }
        let specs: Vec<TerminalSpec> = serde_json::from_str(EMBEDDED_SPECS_JSON)
            .expect("Embedded terminal_specs.json must be valid JSON");
        let h = canonical_db_hash(&specs);
        (h, specs.len(), None)
    }

    /// Looks up the built-in factory default specification for `id`, if `id` is an embedded catalog entry.
    pub fn embedded_default_by_id(id: &str) -> Option<TerminalSpec> {
        let specs: Vec<TerminalSpec> = serde_json::from_str(EMBEDDED_SPECS_JSON)
            .expect("Embedded terminal_specs.json must be valid JSON");
        specs.into_iter().find(|s| s.id == id)
    }

    /// Rebuilds the catalog from embedded specs plus project-specific custom specs.
    pub fn reload_with_custom(&mut self, custom_specs: &[TerminalSpec]) {
        *self = Self::load_embedded();
        for spec in custom_specs {
            self.add_custom_spec(spec.clone());
        }
        self.sort_by_manufacturer_and_type();
    }

    /// Rebuilds the catalog from the active base database (downloaded GitHub DB if present, else embedded)
    /// plus user/project custom specs.
    pub fn reload_active_with_custom(&mut self, custom_specs: &[TerminalSpec]) {
        *self = Self::load_active();
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

    /// Finds a specification matching a normalized manufacturer hint, article number, or type designation.
    pub fn find_by_article_or_type(
        &self,
        manufacturer_hint: Option<&str>,
        article: &str,
        type_number: &str,
    ) -> Option<&TerminalSpec> {
        fn norm(s: &str) -> String {
            s.chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .flat_map(|c| c.to_lowercase())
                .collect()
        }
        fn strip_prefix(s: &str) -> &str {
            let trimmed = s.trim();
            for sep in ['.', ':'] {
                if let Some((head, tail)) = trimmed.split_once(sep) {
                    let h = head.trim();
                    let t = tail.trim();
                    if !t.is_empty()
                        && (((2..=5).contains(&h.len())
                            && h.chars().all(|c| c.is_ascii_alphabetic()))
                            || matches!(
                                h.to_ascii_uppercase().as_str(),
                                "PHOENIX"
                                    | "PHOENIXCONTACT"
                                    | "WEIDMULLER"
                                    | "WEIDMÜLLER"
                                    | "WEIDMUELLER"
                            ))
                    {
                        return t;
                    }
                }
            }
            trimmed
        }

        let clean_art = strip_prefix(article);
        let clean_type = strip_prefix(type_number);
        let art_norm = norm(clean_art);
        let type_norm = norm(clean_type);
        let alt_type_norm = norm(&clean_type.replace(',', "."));

        let mfg_matches = |spec: &TerminalSpec| -> bool {
            manufacturer_hint
                .is_none_or(|m| spec.manufacturer.eq_ignore_ascii_case(m))
        };

        // Pass 1: exact article number match
        if !art_norm.is_empty()
            && let Some(spec) = self.specs.iter().find(|spec| {
                mfg_matches(spec)
                    && spec
                        .effective_article_number()
                        .is_some_and(|a| norm(a) == art_norm)
            })
        {
            return Some(spec);
        }

        // Pass 2: match type number against effective article number (for WAGO e.g. 2204-1201)
        if !type_norm.is_empty()
            && let Some(spec) = self.specs.iter().find(|spec| {
                mfg_matches(spec)
                    && spec
                        .effective_article_number()
                        .is_some_and(|a| norm(a) == type_norm)
            })
        {
            return Some(spec);
        }

        // Pass 3: match type number against short spec name (prefix before " (")
        if !type_norm.is_empty()
            && let Some(spec) = self.specs.iter().find(|spec| {
                if !mfg_matches(spec) {
                    return false;
                }
                let short = spec.name.split(" (").next().unwrap_or(&spec.name).trim();
                let sn = norm(short);
                sn == type_norm || sn == alt_type_norm
            })
        {
            return Some(spec);
        }

        // Pass 4: article equals short spec name (e.g. "PTI 2,5-PE/L/NT")
        if !art_norm.is_empty()
            && let Some(spec) = self.specs.iter().find(|spec| {
                if !mfg_matches(spec) {
                    return false;
                }
                let short = spec.name.split(" (").next().unwrap_or(&spec.name).trim();
                norm(short) == art_norm
            })
        {
            return Some(spec);
        }

        None
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

    /// Filters catalog specifications:
    /// - When `search_query.trim()` is non-empty, performs a **global relevance-ranked search**
    ///   across ALL manufacturers (ignoring `manufacturer_filter`), supporting plain-text terms
    ///   such as `"3 Stock klemme"`, `"NT klemme"`, cross-sections (`"2,5mm²"`), and article numbers.
    /// - When `search_query.trim()` is empty, respects `manufacturer_filter` in catalog sort order.
    pub fn filter_specs(
        &self,
        manufacturer_filter: Option<&str>,
        search_query: &str,
    ) -> Vec<&TerminalSpec> {
        self.filter_specs_for_reference(manufacturer_filter, search_query, None)
    }

    /// Filters catalog specifications, optionally prioritizing compatible terminal types relative to `reference`.
    pub fn filter_specs_for_reference(
        &self,
        manufacturer_filter: Option<&str>,
        search_query: &str,
        reference: Option<&TerminalSpec>,
    ) -> Vec<&TerminalSpec> {
        let parsed = parse_search_query(search_query);
        if parsed.tokens.is_empty() {
            let mut list: Vec<&TerminalSpec> = self
                .specs
                .iter()
                .filter(|s| manufacturer_filter.is_none_or(|m| s.manufacturer == m))
                .collect();
            if let Some(ref_spec) = reference {
                self.sort_specs_for_reference(&mut list, ref_spec);
            }
            return list;
        }

        let mut scored: Vec<(u32, (u8, u8), &TerminalSpec)> = self
            .specs
            .iter()
            .filter_map(|s| {
                s.score_search_query(&parsed).map(|score| {
                    let compat = reference.map_or((0, 0), |r| s.compatibility_rank(r));
                    (score, compat, s)
                })
            })
            .collect();
        scored.sort_by(|(sa, ra, a), (sb, rb, b)| {
            sb.cmp(sa)
                .then_with(|| ra.cmp(rb))
                .then_with(|| a.manufacturer.to_lowercase().cmp(&b.manufacturer.to_lowercase()))
                .then_with(|| natural_cmp(&a.name, &b.name))
        });
        scored.into_iter().map(|(_, _, s)| s).collect()
    }

    /// Sorts `specs` by compatibility rank relative to `reference`, keeping manufacturer and natural order inside each rank.
    pub fn sort_specs_for_reference(&self, specs: &mut [&TerminalSpec], reference: &TerminalSpec) {
        specs.sort_by(|a, b| {
            a.compatibility_rank(reference)
                .cmp(&b.compatibility_rank(reference))
                .then_with(|| a.manufacturer.to_lowercase().cmp(&b.manufacturer.to_lowercase()))
                .then_with(|| natural_cmp(&a.name, &b.name))
                .then_with(|| a.id.cmp(&b.id))
        });
    }

    /// Computes a `1-Polig` (1× L + N + PE) or `3-Polig` (3× L + N + PE) feeder block sequence matching `reference`.
    pub fn feeder_block_plan(&self, reference: &TerminalSpec, poles: u8) -> Option<FeederBlockPlan<'_>> {
        if (poles != 1 && poles != 3)
            || reference.is_busbar_carrier()
            || reference.is_n_supply_terminal()
        {
            return None;
        }

        let id = reference.id.as_str();
        let is_nt = reference.is_nt_disconnect_terminal();

        let seq_ids: Vec<&str> = match reference.manufacturer.as_str() {
            "WAGO" => {
                if id.starts_with("wago-2003-") {
                    let target = if id.starts_with("wago-2003-66") {
                        if is_nt { "wago-2003-6641" } else { "wago-2003-6646" }
                    } else if is_nt {
                        "wago-2003-7641"
                    } else {
                        "wago-2003-7646"
                    };
                    if poles == 1 {
                        vec![target]
                    } else {
                        vec!["wago-2003-7642", target]
                    }
                } else if id.starts_with("wago-2203-") {
                    let target = if id.starts_with("wago-2203-66") {
                        if is_nt { "wago-2203-6641" } else { "wago-2203-6646" }
                    } else if is_nt {
                        "wago-2203-7641"
                    } else {
                        "wago-2203-7646"
                    };
                    if poles == 1 {
                        vec![target]
                    } else {
                        vec!["wago-2203-7642", target]
                    }
                } else if id.starts_with("wago-2005-") {
                    let target = if is_nt { "wago-2005-7641" } else { "wago-2005-7646" };
                    if poles == 1 {
                        vec![target]
                    } else {
                        vec!["wago-2005-7642", target]
                    }
                } else if id.starts_with("wago-2205-") {
                    let target = if is_nt { "wago-2205-7641" } else { "wago-2205-7646" };
                    if poles == 1 {
                        vec![target]
                    } else {
                        vec!["wago-2205-7642", target]
                    }
                } else if id.starts_with("wago-2002-32") {
                    if poles == 1 {
                        vec!["wago-2002-3209"]
                    } else {
                        vec!["wago-2002-3208", "wago-2002-3209"]
                    }
                } else if reference.levels == 2 && id.starts_with("wago-2000-22") {
                    if poles == 1 {
                        vec!["wago-2000-2204", "wago-2000-2207"]
                    } else {
                        vec!["wago-2000-2201", "wago-2000-2204", "wago-2000-2207"]
                    }
                } else if reference.levels == 2 && (id.starts_with("wago-2002-2") || id.starts_with("wago-2002-29")) {
                    if poles == 1 {
                        vec!["wago-2002-2204", "wago-2002-2207"]
                    } else {
                        vec!["wago-2002-2201", "wago-2002-2204", "wago-2002-2207"]
                    }
                } else if reference.levels == 2 && id.starts_with("wago-2202-22") {
                    if poles == 1 {
                        vec!["wago-2202-2204", "wago-2202-2207"]
                    } else {
                        vec!["wago-2202-2201", "wago-2202-2204", "wago-2202-2207"]
                    }
                } else if reference.levels == 1 {
                    let (l_id, n_id, pe_id) = match id {
                        s if s.starts_with("wago-2000-12") => ("wago-2000-1201", "wago-2000-1204", "wago-2000-1207"),
                        s if s.starts_with("wago-2000-13") => ("wago-2000-1301", "wago-2000-1304", "wago-2000-1307"),
                        s if s.starts_with("wago-2000-14") => ("wago-2000-1401", "wago-2000-1404", "wago-2000-1407"),
                        s if s.starts_with("wago-2001-12") => ("wago-2001-1201", "wago-2001-1204", "wago-2001-1207"),
                        s if s.starts_with("wago-2001-13") => ("wago-2001-1301", "wago-2001-1304", "wago-2001-1307"),
                        s if s.starts_with("wago-2001-14") => ("wago-2001-1401", "wago-2001-1404", "wago-2001-1407"),
                        "wago-2001-1661" => ("wago-2001-1661", "wago-2001-1204", "wago-2001-1207"),
                        s if s.starts_with("wago-2002-13") => ("wago-2002-1301", "wago-2002-1304", "wago-2002-1307"),
                        s if s.starts_with("wago-2002-14") => ("wago-2002-1401", "wago-2002-1404", "wago-2002-1407"),
                        "wago-2002-1711" | "wago-2002-1771" => (id, "wago-2002-1304", "wago-2002-1307"),
                        "wago-2002-1811" | "wago-2002-1861" | "wago-2002-1871" => {
                            (id, "wago-2002-1404", "wago-2002-1407")
                        }
                        s if s.starts_with("wago-2002-1") || s == "wago-2022-1201" => {
                            let l = if s.starts_with("wago-2002-16") || s.starts_with("wago-2002-19") {
                                id
                            } else {
                                "wago-2002-1201"
                            };
                            (l, "wago-2002-1204", "wago-2002-1207")
                        }
                        s if s.starts_with("wago-2004-12") => ("wago-2004-1201", "wago-2004-1204", "wago-2004-1207"),
                        s if s.starts_with("wago-2004-13") => ("wago-2004-1301", "wago-2004-1304", "wago-2004-1307"),
                        s if s.starts_with("wago-2004-14") => ("wago-2004-1401", "wago-2004-1404", "wago-2004-1407"),
                        "wago-2004-1661" => ("wago-2004-1661", "wago-2004-1204", "wago-2004-1207"),
                        s if s.starts_with("wago-2006-13") => ("wago-2006-1301", "wago-2006-1304", "wago-2006-1307"),
                        s if s.starts_with("wago-2006-") => {
                            let l = if s.starts_with("wago-2006-16") { id } else { "wago-2006-1201" };
                            (l, "wago-2006-1204", "wago-2006-1207")
                        }
                        s if s.starts_with("wago-2010-13") => ("wago-2010-1301", "wago-2010-1304", "wago-2010-1307"),
                        s if s.starts_with("wago-2010-") => ("wago-2010-1201", "wago-2010-1204", "wago-2010-1207"),
                        s if s.starts_with("wago-2016-13") => ("wago-2016-1301", "wago-2016-1304", "wago-2016-1307"),
                        s if s.starts_with("wago-2016-") => ("wago-2016-1201", "wago-2016-1204", "wago-2016-1207"),
                        s if s.starts_with("wago-2035-") => ("wago-2035-1201", "wago-2035-1204", "wago-2035-1207"),
                        s if s.starts_with("wago-2102-13") => ("wago-2102-1301", "wago-2102-1304", "wago-2102-1307"),
                        s if s.starts_with("wago-2102-14") => ("wago-2102-1401", "wago-2102-1404", "wago-2102-1407"),
                        s if s.starts_with("wago-2102-") => ("wago-2102-1201", "wago-2102-1204", "wago-2102-1207"),
                        s if s.starts_with("wago-2104-13") => ("wago-2104-1301", "wago-2104-1304", "wago-2104-1307"),
                        s if s.starts_with("wago-2104-") => ("wago-2104-1201", "wago-2104-1204", "wago-2104-1207"),
                        s if s.starts_with("wago-2106-13") => ("wago-2106-1301", "wago-2106-1304", "wago-2106-1307"),
                        s if s.starts_with("wago-2106-") => ("wago-2106-1201", "wago-2106-1204", "wago-2106-1207"),
                        s if s.starts_with("wago-2110-13") => ("wago-2110-1301", "wago-2110-1304", "wago-2110-1307"),
                        s if s.starts_with("wago-2110-") => ("wago-2110-1201", "wago-2110-1204", "wago-2110-1207"),
                        s if s.starts_with("wago-2116-13") => ("wago-2116-1301", "wago-2116-1304", "wago-2116-1307"),
                        s if s.starts_with("wago-2116-") => ("wago-2116-1201", "wago-2116-1204", "wago-2116-1207"),
                        s if s.starts_with("wago-2200-12") => ("wago-2200-1201", "wago-2200-1204", "wago-2200-1207"),
                        s if s.starts_with("wago-2200-13") => ("wago-2200-1301", "wago-2200-1304", "wago-2200-1307"),
                        s if s.starts_with("wago-2200-14") => ("wago-2200-1401", "wago-2200-1404", "wago-2200-1407"),
                        s if s.starts_with("wago-2201-12") => ("wago-2201-1201", "wago-2201-1204", "wago-2201-1207"),
                        s if s.starts_with("wago-2201-13") => ("wago-2201-1301", "wago-2201-1304", "wago-2201-1307"),
                        s if s.starts_with("wago-2201-14") => ("wago-2201-1401", "wago-2201-1404", "wago-2201-1407"),
                        s if s.starts_with("wago-2202-13") => ("wago-2202-1301", "wago-2202-1304", "wago-2202-1307"),
                        s if s.starts_with("wago-2202-14") => ("wago-2202-1401", "wago-2202-1404", "wago-2202-1407"),
                        "wago-2202-1711" => ("wago-2202-1711", "wago-2202-1304", "wago-2202-1307"),
                        s if s.starts_with("wago-2202-1") => {
                            let l = if s == "wago-2202-1611" || s == "wago-2202-1661" { id } else { "wago-2202-1201" };
                            (l, "wago-2202-1204", "wago-2202-1207")
                        }
                        s if s.starts_with("wago-2204-12") => ("wago-2204-1201", "wago-2204-1204", "wago-2204-1207"),
                        s if s.starts_with("wago-2204-13") => ("wago-2204-1301", "wago-2204-1304", "wago-2204-1307"),
                        s if s.starts_with("wago-2204-14") => ("wago-2204-1401", "wago-2204-1404", "wago-2204-1407"),
                        s if s.starts_with("wago-2206-13") => ("wago-2206-1301", "wago-2206-1304", "wago-2206-1307"),
                        s if s.starts_with("wago-2206-") => ("wago-2206-1201", "wago-2206-1204", "wago-2206-1207"),
                        s if s.starts_with("wago-2210-") => ("wago-2210-1201", "wago-2210-1204", "wago-2210-1207"),
                        s if s.starts_with("wago-2216-") => ("wago-2216-1201", "wago-2216-1204", "wago-2216-1207"),
                        s if s.starts_with("wago-280-68") => ("wago-280-681", "wago-280-684", "wago-280-687"),
                        s if s.starts_with("wago-280-83") => ("wago-280-833", "wago-280-834", "wago-280-837"),
                        s if s.starts_with("wago-280-90") => ("wago-280-901", "wago-280-904", "wago-280-907"),
                        s if s.starts_with("wago-281-90") => ("wago-281-901", "wago-281-904", "wago-281-907"),
                        s if s.starts_with("wago-285-13") => ("wago-285-135", "wago-285-134", "wago-285-137"),
                        s if s.starts_with("wago-285-15") => ("wago-285-150", "wago-285-154", "wago-285-157"),
                        s if s.starts_with("wago-285-19") => ("wago-285-195", "wago-285-194", "wago-285-197"),
                        _ => return None,
                    };
                    if poles == 1 {
                        vec![l_id, n_id, pe_id]
                    } else {
                        vec![l_id, l_id, l_id, n_id, pe_id]
                    }
                } else {
                    return None;
                }
            }
            "Phoenix Contact" => {
                if id.starts_with("phoenix-pti-25") && reference.levels >= 2 {
                    let target = if is_nt { "phoenix-pti-25-pelnt" } else { "phoenix-pti-25-peln" };
                    if poles == 1 {
                        vec![target]
                    } else {
                        vec!["phoenix-pti-25-ll", target]
                    }
                } else if id.starts_with("phoenix-pti-4") && reference.levels >= 2 {
                    let target = if is_nt { "phoenix-pti-4-pelnt" } else { "phoenix-pti-4-peln" };
                    if poles == 1 {
                        vec![target]
                    } else {
                        vec!["phoenix-pti-4-ll", target]
                    }
                } else if id == "phoenix-sti-25-pelnt" && poles == 1 {
                    vec!["phoenix-sti-25-pelnt"]
                } else if id == "phoenix-uti-25-peln" && poles == 1 {
                    vec!["phoenix-uti-25-peln"]
                } else if id.starts_with("phoenix-pt-15s-") && reference.levels == 3 {
                    if poles == 1 {
                        vec!["phoenix-pt-15s-peln"]
                    } else {
                        vec!["phoenix-pt-15s-pell", "phoenix-pt-15s-peln"]
                    }
                } else if id.starts_with("phoenix-pt-25-") && reference.levels == 3 {
                    if poles == 1 {
                        vec!["phoenix-pt-25-peln"]
                    } else {
                        vec!["phoenix-pt-25-pell", "phoenix-pt-25-peln"]
                    }
                } else if id.starts_with("phoenix-pttb-15s") {
                    if poles == 1 {
                        vec!["phoenix-pttb-15s-ln", "phoenix-pttb-15s-pe"]
                    } else {
                        vec!["phoenix-pttb-15s", "phoenix-pttb-15s-ln", "phoenix-pttb-15s-pe"]
                    }
                } else if id.starts_with("phoenix-pttb-25") {
                    if poles == 1 {
                        vec!["phoenix-pttb-25-ln-real", "phoenix-pttb-25-pe"]
                    } else {
                        vec!["phoenix-pttb-25", "phoenix-pttb-25-ln-real", "phoenix-pttb-25-pe"]
                    }
                } else if reference.levels == 1 {
                    let (l_id, n_id, pe_id) = match id {
                        "phoenix-pti-25"
                        | "phoenix-pti-25-bu"
                        | "phoenix-pti-25-pe"
                        | "phoenix-ptn-25" => (
                            "phoenix-pti-25",
                            if is_nt { "phoenix-ptn-25" } else { "phoenix-pti-25-bu" },
                            "phoenix-pti-25-pe",
                        ),
                        "phoenix-pti-4"
                        | "phoenix-pti-4-bu"
                        | "phoenix-pti-4-pe"
                        | "phoenix-ptn-4" => (
                            "phoenix-pti-4",
                            if is_nt { "phoenix-ptn-4" } else { "phoenix-pti-4-bu" },
                            "phoenix-pti-4-pe",
                        ),
                        "phoenix-pti-6"
                        | "phoenix-pti-6-bu"
                        | "phoenix-pti-6-pe"
                        | "phoenix-ptn-6" => (
                            "phoenix-pti-6",
                            if is_nt { "phoenix-ptn-6" } else { "phoenix-pti-6-bu" },
                            "phoenix-pti-6-pe",
                        ),
                        "phoenix-pti-16s"
                        | "phoenix-pti-16s-bu"
                        | "phoenix-pti-16s-pe"
                        | "phoenix-ptn-16s" => (
                            "phoenix-pti-16s",
                            if is_nt { "phoenix-ptn-16s" } else { "phoenix-pti-16s-bu" },
                            "phoenix-pti-16s-pe",
                        ),
                        s if s.starts_with("phoenix-pt-15s-quattro") => (
                            "phoenix-pt-15s-quattro",
                            "phoenix-pt-15s-quattro-bu",
                            "phoenix-pt-15s-quattro-pe",
                        ),
                        s if s.starts_with("phoenix-pt-15s-twin") => (
                            "phoenix-pt-15s-twin",
                            "phoenix-pt-15s-twin-bu",
                            "phoenix-pt-15s-twin-pe",
                        ),
                        s if s.starts_with("phoenix-pt-15s") => {
                            let l = if s == "phoenix-pt-15s-mt" || s == "phoenix-pt-15s-tg" {
                                id
                            } else {
                                "phoenix-pt-15s"
                            };
                            (l, "phoenix-pt-15s-bu", "phoenix-pt-15s-pe")
                        }
                        s if s.starts_with("phoenix-pt-25-quattro") => (
                            if s.ends_with("-mt") || s.ends_with("-tg") || s.ends_with("-dio-lr") {
                                id
                            } else {
                                "phoenix-pt-25-quattro"
                            },
                            "phoenix-pt-25-quattro-bu",
                            "phoenix-pt-25-quattro-pe",
                        ),
                        s if s.starts_with("phoenix-pt-25-twin") => (
                            if s.ends_with("-mt") || s.ends_with("-tg") { id } else { "phoenix-pt-25-twin" },
                            "phoenix-pt-25-twin-bu",
                            "phoenix-pt-25-twin-pe",
                        ),
                        s if s.starts_with("phoenix-pt-25") => {
                            let l = if s == "phoenix-pt-25-mt" || s == "phoenix-pt-25-tg" || s == "phoenix-pt-25-dio-lr" {
                                id
                            } else {
                                "phoenix-pt-25"
                            };
                            (l, "phoenix-pt-25-bu", "phoenix-pt-25-pe")
                        }
                        s if s.starts_with("phoenix-pt-4-quattro") => (
                            "phoenix-pt-4-quattro",
                            "phoenix-pt-4-quattro-bu",
                            "phoenix-pt-4-quattro-pe",
                        ),
                        s if s.starts_with("phoenix-pt-4-twin") => (
                            "phoenix-pt-4-twin",
                            "phoenix-pt-4-twin-bu",
                            "phoenix-pt-4-twin-pe",
                        ),
                        s if s.starts_with("phoenix-pt-4") => {
                            let l = if s.contains("-hesi") || s == "phoenix-pt-4-mt" || s == "phoenix-pt-4-tg" {
                                id
                            } else {
                                "phoenix-pt-4"
                            };
                            (l, "phoenix-pt-4-bu", "phoenix-pt-4-pe")
                        }
                        s if s.starts_with("phoenix-pt-6-quattro") => (
                            "phoenix-pt-6-quattro",
                            "phoenix-pt-6-quattro-bu",
                            "phoenix-pt-6-quattro-pe",
                        ),
                        s if s.starts_with("phoenix-pt-6-twin") => (
                            "phoenix-pt-6-twin",
                            "phoenix-pt-6-twin-bu",
                            "phoenix-pt-6-twin-pe",
                        ),
                        s if s.starts_with("phoenix-pt-6") => {
                            let l = if s.contains("-hesi") || s == "phoenix-pt-6-t" || s == "phoenix-pt-6-tg" {
                                id
                            } else {
                                "phoenix-pt-6"
                            };
                            (l, "phoenix-pt-6-bu", "phoenix-pt-6-pe")
                        }
                        s if s.starts_with("phoenix-pt-10-twin") => (
                            "phoenix-pt-10-twin",
                            "phoenix-pt-10-twin-bu",
                            "phoenix-pt-10-twin-pe",
                        ),
                        s if s.starts_with("phoenix-pt-10") => {
                            ("phoenix-pt-10", "phoenix-pt-10-bu", "phoenix-pt-10-pe")
                        }
                        s if s.starts_with("phoenix-pt-16-twin") => (
                            "phoenix-pt-16-twin-n",
                            "phoenix-pt-16-twin-n-bu",
                            "phoenix-pt-16-twin-n-pe",
                        ),
                        s if s.starts_with("phoenix-pt-16") => {
                            ("phoenix-pt-16n", "phoenix-pt-16n-bu", "phoenix-pt-16n-pe")
                        }
                        s if s.starts_with("phoenix-ptv-25-quattro") => (
                            "phoenix-ptv-25-quattro",
                            "phoenix-ptv-25-quattro-bu",
                            "phoenix-ptv-25-quattro-pe",
                        ),
                        s if s.starts_with("phoenix-ptv-25-twin") => (
                            "phoenix-ptv-25-twin",
                            "phoenix-ptv-25-twin-bu",
                            "phoenix-ptv-25-twin-pe",
                        ),
                        s if s.starts_with("phoenix-ptv-25") => {
                            ("phoenix-ptv-25", "phoenix-ptv-25-bu", "phoenix-ptv-25-pe")
                        }
                        s if s.starts_with("phoenix-ptv-4-quattro") => (
                            "phoenix-ptv-4-quattro",
                            "phoenix-ptv-4-quattro-bu",
                            "phoenix-ptv-4-quattro-pe",
                        ),
                        s if s.starts_with("phoenix-ptv-4-twin") => (
                            "phoenix-ptv-4-twin",
                            "phoenix-ptv-4-twin-bu",
                            "phoenix-ptv-4-twin-pe",
                        ),
                        s if s.starts_with("phoenix-ptv-4") => {
                            ("phoenix-ptv-4", "phoenix-ptv-4-bu", "phoenix-ptv-4-pe")
                        }
                        s if s.starts_with("phoenix-ptv-6") => {
                            ("phoenix-ptv-6", "phoenix-ptv-6-bu", "phoenix-ptv-6-pe")
                        }
                        s if s.starts_with("phoenix-ptv-10") => {
                            ("phoenix-ptv-10", "phoenix-ptv-10-bu", "phoenix-ptv-10-pe")
                        }
                        s if s.starts_with("phoenix-ptv-16") => {
                            ("phoenix-ptv-16", "phoenix-ptv-16-bu", "phoenix-ptv-16-pe")
                        }
                        s if s.starts_with("phoenix-st-25-quattro") => (
                            "phoenix-st-25-quattro",
                            "phoenix-st-25-quattro-bu",
                            "phoenix-st-25-quattro-pe",
                        ),
                        s if s.starts_with("phoenix-st-25-twin") => (
                            "phoenix-st-25-twin",
                            "phoenix-st-25-twin-bu",
                            "phoenix-st-25-twin-pe",
                        ),
                        s if s.starts_with("phoenix-st-25") || s == "phoenix-stn-25" => {
                            let l = if s == "phoenix-st-25-mt" { id } else { "phoenix-st-25" };
                            (
                                l,
                                if is_nt { "phoenix-stn-25" } else { "phoenix-st-25-bu" },
                                "phoenix-st-25-pe",
                            )
                        }
                        "phoenix-st-4-quattro" => {
                            ("phoenix-st-4-quattro", "phoenix-st-4-bu", "phoenix-st-4-pe")
                        }
                        "phoenix-st-4-twin" => ("phoenix-st-4-twin", "phoenix-st-4-bu", "phoenix-st-4-pe"),
                        s if s.starts_with("phoenix-st-4") || s == "phoenix-stn-4" => (
                            "phoenix-st-4",
                            if is_nt { "phoenix-stn-4" } else { "phoenix-st-4-bu" },
                            "phoenix-st-4-pe",
                        ),
                        s if s.starts_with("phoenix-st-6") => {
                            ("phoenix-st-6", "phoenix-st-6-bu", "phoenix-st-6-pe")
                        }
                        "phoenix-stn-10" => ("phoenix-st-10", "phoenix-stn-10", "phoenix-st-10-pe"),
                        "phoenix-stn-16" => ("phoenix-st-16", "phoenix-stn-16", "phoenix-st-16-pe"),
                        s if s.starts_with("phoenix-ut-25-quattro") => (
                            "phoenix-ut-25-quattro",
                            "phoenix-ut-25-quattro-bu",
                            "phoenix-ut-25-quattro-pe",
                        ),
                        s if s.starts_with("phoenix-ut-25-twin") => (
                            "phoenix-ut-25-twin",
                            "phoenix-ut-25-twin-bu",
                            "phoenix-ut-25-twin-pe",
                        ),
                        s if s.starts_with("phoenix-ut-25") || s == "phoenix-utn-25" => {
                            let l = if s == "phoenix-ut-25-mt" { id } else { "phoenix-ut-25" };
                            (
                                l,
                                if is_nt { "phoenix-utn-25" } else { "phoenix-ut-25-bu" },
                                "phoenix-ut-25-pe",
                            )
                        }
                        "phoenix-ut-4-quattro" | "phoenix-ut-4-quattro-pe" => (
                            "phoenix-ut-4-quattro",
                            "phoenix-ut-4-bu",
                            "phoenix-ut-4-quattro-pe",
                        ),
                        s if s.starts_with("phoenix-ut-4-twin") => (
                            "phoenix-ut-4-twin",
                            "phoenix-ut-4-twin-bu",
                            "phoenix-ut-4-twin-pe",
                        ),
                        s if s.starts_with("phoenix-ut-4") || s == "phoenix-utn-4" => {
                            let l = if s.contains("-hesi") || s == "phoenix-ut-4-mt" {
                                id
                            } else {
                                "phoenix-ut-4"
                            };
                            (
                                l,
                                if is_nt { "phoenix-utn-4" } else { "phoenix-ut-4-bu" },
                                "phoenix-ut-4-pe",
                            )
                        }
                        s if s.starts_with("phoenix-ut-6") || s == "phoenix-utn-6" => (
                            "phoenix-ut-6",
                            if is_nt { "phoenix-utn-6" } else { "phoenix-ut-6-bu" },
                            "phoenix-ut-6-pe",
                        ),
                        s if s.starts_with("phoenix-ut-10") || s == "phoenix-utn-10" => (
                            "phoenix-ut-10",
                            if is_nt { "phoenix-utn-10" } else { "phoenix-ut-10-bu" },
                            "phoenix-ut-10-pe",
                        ),
                        s if s.starts_with("phoenix-ut-16") || s == "phoenix-utn-16" => (
                            "phoenix-ut-16",
                            if is_nt { "phoenix-utn-16" } else { "phoenix-ut-16-bu" },
                            "phoenix-ut-16-pe",
                        ),
                        s if s.starts_with("phoenix-ut-35") || s == "phoenix-utn-35" => (
                            "phoenix-ut-35",
                            if is_nt { "phoenix-utn-35" } else { "phoenix-ut-35-bu" },
                            "phoenix-ut-35-pe",
                        ),
                        "phoenix-uk-5n" | "phoenix-uk-5n-bu" | "phoenix-uslkg-5" => {
                            ("phoenix-uk-5n", "phoenix-uk-5n-bu", "phoenix-uslkg-5")
                        }
                        s if s.starts_with("phoenix-ptpower-35") => (
                            "phoenix-ptpower-35",
                            "phoenix-ptpower-35-bu",
                            "phoenix-ptpower-35-pe",
                        ),
                        s if s.starts_with("phoenix-ptpower-50") => (
                            "phoenix-ptpower-50",
                            "phoenix-ptpower-50-bu",
                            "phoenix-ptpower-50-pe",
                        ),
                        s if s.starts_with("phoenix-ptpower-95") => (
                            "phoenix-ptpower-95",
                            "phoenix-ptpower-95-bu",
                            "phoenix-ptpower-95-pe",
                        ),
                        s if s.starts_with("phoenix-ptfix-6-18x25") => (
                            "phoenix-ptfix-6-18x25-gy",
                            "phoenix-ptfix-6-18x25-bu",
                            "phoenix-ptfix-6-18x25-pe",
                        ),
                        _ => return None,
                    };
                    if poles == 1 {
                        vec![l_id, n_id, pe_id]
                    } else {
                        vec![l_id, l_id, l_id, n_id, pe_id]
                    }
                } else {
                    return None;
                }
            }
            "Weidmüller" => {
                if id.starts_with("weidmuller-aitb-25-") && reference.levels >= 2 {
                    let target = if is_nt {
                        "weidmuller-aitb-25-ntlp"
                    } else {
                        "weidmuller-aitb-25-nlp"
                    };
                    if poles == 1 {
                        vec![target]
                    } else {
                        vec!["weidmuller-aitb-25-ll", target]
                    }
                } else if id.starts_with("weidmuller-aitb-4-") && reference.levels >= 2 {
                    let target = if is_nt {
                        "weidmuller-aitb-4-ntlp"
                    } else {
                        "weidmuller-aitb-4-nlp"
                    };
                    if poles == 1 {
                        vec![target]
                    } else {
                        vec!["weidmuller-aitb-4-ll", target]
                    }
                } else if id.starts_with("weidmuller-a3t-25") {
                    if poles == 1 {
                        vec!["weidmuller-a3t-25-peln"]
                    } else {
                        vec!["weidmuller-a3t-25-pell", "weidmuller-a3t-25-peln"]
                    }
                } else if id.starts_with("weidmuller-dli-25") || id.starts_with("weidmuller-dld-25") {
                    if poles == 1 {
                        vec!["weidmuller-dli-25"]
                    } else {
                        vec!["weidmuller-dld-25-pe", "weidmuller-dli-25"]
                    }
                } else if id.starts_with("weidmuller-zdl-25") {
                    if poles == 1 {
                        vec!["weidmuller-zdl-25"]
                    } else {
                        vec!["weidmuller-zdk-25", "weidmuller-zdl-25"]
                    }
                } else if id.starts_with("weidmuller-wdk-25") {
                    if poles == 1 {
                        vec!["weidmuller-wdk-25-n", "weidmuller-wdk-25-pe"]
                    } else {
                        vec!["weidmuller-wdk-25", "weidmuller-wdk-25-n", "weidmuller-wdk-25-pe"]
                    }
                } else if id.starts_with("weidmuller-a2t-15") {
                    if poles == 1 {
                        vec!["weidmuller-a2t-15-ft-pe", "weidmuller-a2c-15-bl"]
                    } else {
                        vec!["weidmuller-a2t-15", "weidmuller-a2t-15-ft-pe", "weidmuller-a2c-15-bl"]
                    }
                } else if id.starts_with("weidmuller-a2t-25") {
                    if poles == 1 {
                        vec!["weidmuller-a2t-25-ft-pe", "weidmuller-a2c-25-bl"]
                    } else {
                        vec!["weidmuller-a2t-25", "weidmuller-a2t-25-ft-pe", "weidmuller-a2c-25-bl"]
                    }
                } else if id.starts_with("weidmuller-a2t-4") {
                    if poles == 1 {
                        vec!["weidmuller-a2t-4-ft-pe", "weidmuller-a2c-4-bl"]
                    } else {
                        vec!["weidmuller-a2t-4", "weidmuller-a2t-4-ft-pe", "weidmuller-a2c-4-bl"]
                    }
                } else if reference.levels == 1 {
                    let (l_id, n_id, pe_id) = match id {
                        s if s.starts_with("weidmuller-aitb-25-bb") => (
                            "weidmuller-aitb-25-bb",
                            "weidmuller-aitb-25-bb-bl",
                            "weidmuller-aitb-25-bb-pe",
                        ),
                        s if s.starts_with("weidmuller-aitb-4-bb") => (
                            "weidmuller-aitb-4-bb",
                            "weidmuller-aitb-4-bb-bl",
                            "weidmuller-aitb-4-bb-pe",
                        ),
                        s if s.starts_with("weidmuller-aitb-6-bb") => (
                            "weidmuller-aitb-6-bb",
                            "weidmuller-aitb-6-bb-bl",
                            "weidmuller-aitb-6-bb-pe",
                        ),
                        s if s.starts_with("weidmuller-aitb-10-bb") => (
                            "weidmuller-aitb-10-bb",
                            "weidmuller-aitb-10-bb-bl",
                            "weidmuller-aitb-10-bb-pe",
                        ),
                        s if s.starts_with("weidmuller-aitb-16-bb") => (
                            "weidmuller-aitb-16-bb",
                            "weidmuller-aitb-16-bb-bl",
                            "weidmuller-aitb-16-bb-pe",
                        ),
                        s if s.starts_with("weidmuller-a2c-15") => (
                            "weidmuller-a2c-15",
                            "weidmuller-a2c-15-bl",
                            "weidmuller-a2c-15-pe",
                        ),
                        s if s.starts_with("weidmuller-a2c-25")
                            || s == "weidmuller-amc-25"
                            || s == "weidmuller-adt-25" =>
                        {
                            let l = if s == "weidmuller-amc-25" || s == "weidmuller-adt-25" {
                                id
                            } else {
                                "weidmuller-a2c-25"
                            };
                            (l, "weidmuller-a2c-25-bl", "weidmuller-a2c-25-pe")
                        }
                        s if s.starts_with("weidmuller-a2c-4")
                            || s == "weidmuller-amc-4"
                            || s == "weidmuller-adt-4-2c"
                            || s.starts_with("weidmuller-afs-4") =>
                        {
                            let l = if s.starts_with("weidmuller-a2c-4") {
                                "weidmuller-a2c-4"
                            } else {
                                id
                            };
                            (l, "weidmuller-a2c-4-bl", "weidmuller-a2c-4-pe")
                        }
                        s if s.starts_with("weidmuller-a2c-6") => (
                            "weidmuller-a2c-6",
                            "weidmuller-a2c-6-bl",
                            "weidmuller-a2c-6-pe",
                        ),
                        s if s.starts_with("weidmuller-a2c-10") => (
                            "weidmuller-a2c-10",
                            "weidmuller-a2c-10-bl",
                            "weidmuller-a2c-10-pe",
                        ),
                        s if s.starts_with("weidmuller-a2c-16") => (
                            "weidmuller-a2c-16",
                            "weidmuller-a2c-16-bl",
                            "weidmuller-a2c-16-pe",
                        ),
                        s if s.starts_with("weidmuller-a2c-35") => (
                            "weidmuller-a2c-35",
                            "weidmuller-a2c-35-bl",
                            "weidmuller-a2c-35-pe",
                        ),
                        s if s.starts_with("weidmuller-a2c-50-70") => (
                            "weidmuller-a2c-50-70",
                            "weidmuller-a2c-50-70-bl",
                            "weidmuller-a2c-50-70-pe",
                        ),
                        s if s.starts_with("weidmuller-a2c-95-120") => (
                            "weidmuller-a2c-95-120",
                            "weidmuller-a2c-95-120-bl",
                            "weidmuller-a2c-95-120-pe",
                        ),
                        s if s.starts_with("weidmuller-a3c-15") => (
                            "weidmuller-a3c-15",
                            "weidmuller-a3c-15-bl",
                            "weidmuller-a3c-15-pe",
                        ),
                        s if s.starts_with("weidmuller-a3c-25") || s == "weidmuller-adt-25-3c" => {
                            let l = if s == "weidmuller-adt-25-3c" { id } else { "weidmuller-a3c-25" };
                            (l, "weidmuller-a3c-25-bl", "weidmuller-a3c-25-pe")
                        }
                        s if s.starts_with("weidmuller-a3c-4") => (
                            "weidmuller-a3c-4",
                            "weidmuller-a3c-4-bl",
                            "weidmuller-a3c-4-pe",
                        ),
                        s if s.starts_with("weidmuller-a3c-6") => (
                            "weidmuller-a3c-6",
                            "weidmuller-a3c-6-bl",
                            "weidmuller-a3c-6-pe",
                        ),
                        s if s.starts_with("weidmuller-a3c-10") => (
                            "weidmuller-a3c-10",
                            "weidmuller-a3c-10-bl",
                            "weidmuller-a3c-10-pe",
                        ),
                        s if s.starts_with("weidmuller-a3c-16") => (
                            "weidmuller-a3c-16",
                            "weidmuller-a3c-16-bl",
                            "weidmuller-a3c-16-pe",
                        ),
                        s if s.starts_with("weidmuller-a4c-15") => (
                            "weidmuller-a4c-15",
                            "weidmuller-a4c-15-bl",
                            "weidmuller-a4c-15-pe",
                        ),
                        s if s.starts_with("weidmuller-a4c-25") || s == "weidmuller-adt-25-4c" => {
                            let l = if s == "weidmuller-adt-25-4c" { id } else { "weidmuller-a4c-25" };
                            (l, "weidmuller-a4c-25-bl", "weidmuller-a4c-25-pe")
                        }
                        s if s.starts_with("weidmuller-a4c-4") => (
                            "weidmuller-a4c-4",
                            "weidmuller-a4c-4-bl",
                            "weidmuller-a4c-4-pe",
                        ),
                        "weidmuller-wdu-15-zz" | "weidmuller-wdu-15-zz-bl" | "weidmuller-wpe-15-zz" => (
                            "weidmuller-wdu-15-zz",
                            "weidmuller-wdu-15-zz-bl",
                            "weidmuller-wpe-15-zz",
                        ),
                        "weidmuller-wdu-25-15-zr"
                        | "weidmuller-wdu-25-15-zr-bl"
                        | "weidmuller-wpe-25-15-zr" => (
                            "weidmuller-wdu-25-15-zr",
                            "weidmuller-wdu-25-15-zr-bl",
                            "weidmuller-wpe-25-15-zr",
                        ),
                        "weidmuller-wdu-25" | "weidmuller-wdu-25-bl" | "weidmuller-wpe-25" | "weidmuller-wtr-25" => (
                            if id == "weidmuller-wtr-25" { id } else { "weidmuller-wdu-25" },
                            "weidmuller-wdu-25-bl",
                            "weidmuller-wpe-25",
                        ),
                        "weidmuller-wdu-4" | "weidmuller-wdu-4-bl" | "weidmuller-wpe-4" | "weidmuller-wtr-4" => (
                            if id == "weidmuller-wtr-4" { id } else { "weidmuller-wdu-4" },
                            "weidmuller-wdu-4-bl",
                            "weidmuller-wpe-4",
                        ),
                        s if s.starts_with("weidmuller-wdu-6") || s == "weidmuller-wpe-6" || s.starts_with("weidmuller-wsi-6") => (
                            if s.starts_with("weidmuller-wsi-6") { id } else { "weidmuller-wdu-6" },
                            "weidmuller-wdu-6-bl",
                            "weidmuller-wpe-6",
                        ),
                        "weidmuller-wdu-10" | "weidmuller-wdu-10-bl" | "weidmuller-wpe-10" | "weidmuller-wnt-10" => (
                            "weidmuller-wdu-10",
                            if is_nt { "weidmuller-wnt-10" } else { "weidmuller-wdu-10-bl" },
                            "weidmuller-wpe-10",
                        ),
                        "weidmuller-wdu-16" | "weidmuller-wdu-16-bl" | "weidmuller-wpe-16" | "weidmuller-wnt-16n" => (
                            "weidmuller-wdu-16",
                            if is_nt { "weidmuller-wnt-16n" } else { "weidmuller-wdu-16-bl" },
                            "weidmuller-wpe-16",
                        ),
                        "weidmuller-wdu-35" | "weidmuller-wdu-35-bl" | "weidmuller-wpe-35" => (
                            "weidmuller-wdu-35",
                            "weidmuller-wdu-35-bl",
                            "weidmuller-wpe-35",
                        ),
                        "weidmuller-zdu-15" | "weidmuller-zdu-15-bl" | "weidmuller-zpe-15" => (
                            "weidmuller-zdu-15",
                            "weidmuller-zdu-15-bl",
                            "weidmuller-zpe-15",
                        ),
                        "weidmuller-zdu-25-3an" | "weidmuller-zdu-25-3an-bl" | "weidmuller-zpe-25-3an" => (
                            "weidmuller-zdu-25-3an",
                            "weidmuller-zdu-25-3an-bl",
                            "weidmuller-zpe-25-3an",
                        ),
                        "weidmuller-zdu-25-4an" | "weidmuller-zdu-25-4an-bl" | "weidmuller-zpe-25-4an" => (
                            "weidmuller-zdu-25-4an",
                            "weidmuller-zdu-25-4an-bl",
                            "weidmuller-zpe-25-4an",
                        ),
                        "weidmuller-zdu-25" | "weidmuller-zdu-25-bl" | "weidmuller-zpe-25" | "weidmuller-ztr-25" => (
                            if id == "weidmuller-ztr-25" { id } else { "weidmuller-zdu-25" },
                            "weidmuller-zdu-25-bl",
                            "weidmuller-zpe-25",
                        ),
                        "weidmuller-zdu-4" | "weidmuller-zdu-4-bl" | "weidmuller-zpe-4" => (
                            "weidmuller-zdu-4",
                            "weidmuller-zdu-4-bl",
                            "weidmuller-zpe-4",
                        ),
                        "weidmuller-zdu-6" | "weidmuller-zdu-6-bl" | "weidmuller-zpe-6" => (
                            "weidmuller-zdu-6",
                            "weidmuller-zdu-6-bl",
                            "weidmuller-zpe-6",
                        ),
                        _ => return None,
                    };
                    if poles == 1 {
                        vec![l_id, n_id, pe_id]
                    } else {
                        vec![l_id, l_id, l_id, n_id, pe_id]
                    }
                } else {
                    return None;
                }
            }
            _ => return None,
        };

        let mut specs = Vec::with_capacity(seq_ids.len());
        for sid in seq_ids {
            specs.push(self.find_by_id(sid)?);
        }
        let summary = format_spec_sequence_summary(&specs);
        Some(FeederBlockPlan { specs, summary })
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

/// Precomputed sequence of terminals for inserting a `1-Polig` (`1× L + N + PE`) or `3-Polig` (`3× L + N + PE`) circuit.
#[derive(Debug, Clone)]
pub struct FeederBlockPlan<'a> {
    pub specs: Vec<&'a TerminalSpec>,
    pub summary: String,
}

fn short_spec_code(name: &str) -> &str {
    name.split(" (").next().unwrap_or(name).trim()
}

fn format_spec_sequence_summary(specs: &[&TerminalSpec]) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut i = 0;
    while i < specs.len() {
        let mut count = 1usize;
        while i + count < specs.len() && specs[i + count].id == specs[i].id {
            count += 1;
        }
        parts.push(format!("{count}× {}", short_spec_code(&specs[i].name)));
        i += count;
    }
    parts.join(" + ")
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
            let min_h = if spec.id.contains("ptfix") { 280 } else { 420 };
            assert!(
                (min_h..=2300).contains(&h),
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
            catalog.find_by_id("phoenix-ptfix-6-18x25-gy").unwrap().effective_height_tenths_mm(),
            286
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

    #[test]
    fn udmtk_and_multi_level_pe_catalog_specs_are_complete_and_accurate() {
        use crate::catalog::accessories::{end_plate_width_tenths_mm, resolve_accessories};

        let catalog = TerminalCatalog::load_embedded();

        // Phoenix UDMTK 5 & UKK 5-MTK knife-disconnect family
        for id in [
            "phoenix-udmtk-5",
            "phoenix-udmtk-5-pp",
            "phoenix-udmtk-5-pp-bu",
            "phoenix-udmtk-5-pl",
            "phoenix-udmtk-5-twin-pp",
            "phoenix-udmtkb-5-pp",
            "phoenix-ukk-5-mtk-pp",
        ] {
            let spec = catalog.find_by_id(id).expect(id);
            assert_eq!(spec.manufacturer, "Phoenix Contact");
            assert_eq!(end_plate_width_tenths_mm(spec), 0, "{id} has closed side wall");
            assert_eq!(spec.effective_rail_offset_tenths_mm(), 0);
            let _acc = resolve_accessories(spec);
        }
        assert_eq!(
            catalog.find_by_id("phoenix-udmtk-5").unwrap().effective_height_tenths_mm(),
            930
        );
        assert_eq!(
            catalog.find_by_id("phoenix-udmtk-5").unwrap().width_tenths_mm,
            62
        );
        assert_eq!(
            catalog.find_by_id("phoenix-udmtk-5").unwrap().levels,
            2
        );
        assert_eq!(
            catalog.find_by_id("phoenix-udmtk-5").unwrap().tiers.len(),
            2
        );
        assert_eq!(
            catalog.find_by_id("phoenix-ukk-5-mtk-pp").unwrap().levels,
            2
        );

        // Pure 2-level & 3-level PE terminals must have all tiers as ProtectiveEarth (no L/L!)
        for id in [
            "phoenix-pttb-15s-pe",
            "phoenix-pttb-25-pe",
            "phoenix-pttb-4-pe",
            "phoenix-ptvtb-25-pe",
            "phoenix-sttb-25-pe",
            "phoenix-uttb-25-pe",
            "phoenix-uttb-4-pe",
            "phoenix-pt-15s-3pe",
            "phoenix-pt-25-3pe",
            "wago-2000-2207",
            "wago-2002-2207",
            "wago-2202-2207",
            "wago-2002-3207",
            "wago-2002-3237",
            "wago-2202-3207",
            "weidmuller-a2t-25-pe",
            "weidmuller-a3t-25-pe",
            "weidmuller-wdk-25-pe",
            "weidmuller-zdk-25-pe",
        ] {
            let spec = catalog.find_by_id(id).expect(id);
            assert!(spec.levels >= 2, "{id} must be multi-level");
            assert_eq!(spec.tiers.len(), spec.levels);
            assert!(
                spec.tiers.iter().all(|t| t.behavior.is_pe()),
                "{id} must be pure PE across all {} tiers",
                spec.levels
            );
            assert_eq!(spec.sequential_tier_count(), 0);
        }

        // Hybrid PE/L terminals remain 1x PE + 1x Sequential
        for id in [
            "phoenix-pttb-25-pel",
            "phoenix-sttb-25-pel",
            "phoenix-uttb-25-pel",
            "weidmuller-a2t-25-ft-pe",
        ] {
            let spec = catalog.find_by_id(id).expect(id);
            assert_eq!(spec.levels, 2);
            assert!(spec.tiers[0].behavior.is_pe());
            assert_eq!(spec.tiers[1].behavior, TierBehavior::Sequential);
        }

        // Single-tier PTI 6 terminals stay symmetric at 66.0 mm height
        for id in ["phoenix-pti-6", "phoenix-pti-6-bu", "phoenix-pti-6-pe"] {
            let spec = catalog.find_by_id(id).expect(id);
            assert_eq!(spec.levels, 1);
            assert_eq!(spec.effective_height_tenths_mm(), 660);
            assert_eq!(spec.effective_rail_offset_tenths_mm(), 0);
        }

        // Phoenix Contact 280-spec terminal block catalog verifications (incl. PTN, PTI 16/S, UTN, STN, URTK, QTC)
        assert_eq!(
            catalog
                .all()
                .iter()
                .filter(|s| s.manufacturer == "Phoenix Contact" && !s.is_busbar_carrier())
                .count(),
            280
        );
        for ptn_id in [
            "phoenix-ptn-25",
            "phoenix-ptn-4",
            "phoenix-ptn-6",
            "phoenix-ptn-16s",
            "phoenix-utn-25",
            "phoenix-utn-4",
            "phoenix-utn-6",
            "phoenix-utn-10",
            "phoenix-utn-16",
            "phoenix-utn-35",
            "phoenix-stn-25",
            "phoenix-stn-4",
            "phoenix-stn-10",
            "phoenix-stn-16",
        ] {
            let ptn = catalog.find_by_id(ptn_id).expect(ptn_id);
            assert_eq!(ptn.levels, 1);
            assert!(ptn.is_nt_disconnect_terminal(), "{ptn_id} must be recognized as NT");
            assert!(ptn.is_installation_or_nt_terminal());
            assert_eq!(ptn.effective_jumper_slots(), 0);
            assert_eq!(ptn.effective_max_jumper_poles(), 0);
        }
        assert_eq!(
            catalog.find_by_id("phoenix-uslkg-5").unwrap().effective_jumper_slots(),
            0
        );
        assert_eq!(
            catalog.find_by_id("phoenix-ptrv-8-whrd").unwrap().levels,
            8
        );
        assert_eq!(
            catalog.find_by_id("phoenix-ptrv-8-whrd").unwrap().effective_jumper_slots(),
            0
        );
        let ut16 = catalog.find_by_id("phoenix-ut-16").unwrap();
        assert_eq!(end_plate_width_tenths_mm(ut16), 22);
        assert_eq!(resolve_accessories(ut16).end_plate, "D-UT 16 (3047206)");
        assert_eq!(
            resolve_accessories(catalog.find_by_id("phoenix-pt-25-quattro-tg").unwrap()).end_plate,
            "D-PT 2,5-QUATTRO-MT (3211320)"
        );
        assert_eq!(
            resolve_accessories(catalog.find_by_id("phoenix-pti-25-lnt").unwrap()).end_plate,
            "D-PTI/3 (3213975)"
        );
        let ptio4 = catalog.find_by_id("phoenix-ptio-15s-4-pe").unwrap();
        assert_eq!(ptio4.levels, 4);
        assert_eq!(ptio4.effective_rail_offset_tenths_mm(), 80);
        assert_eq!(resolve_accessories(ptio4).end_plate, "D-PTIO 1,5/S/4 (3244588)");
        let ptio_in = catalog.find_by_id("phoenix-ptio-in-25-4-pe").unwrap();
        assert_eq!(end_plate_width_tenths_mm(ptio_in), 0);
        let pt185 = catalog.find_by_id("phoenix-ptpower-185").unwrap();
        assert_eq!(pt185.effective_height_tenths_mm(), 1500);
        assert_eq!(resolve_accessories(pt185).jumpers, "EB 2-31/PT (3215057)");
        assert_eq!(
            resolve_accessories(catalog.find_by_id("phoenix-pti-25").unwrap()).end_plate,
            "D-PTI 2,5 (3213974)"
        );
        assert_eq!(
            resolve_accessories(catalog.find_by_id("phoenix-pti-4").unwrap()).end_plate,
            "D-PTI 4 (3213979)"
        );
        let ut4mt = catalog.find_by_id("phoenix-ut-4-mt").unwrap();
        assert_eq!(end_plate_width_tenths_mm(ut4mt), 0);
        let pt15tg = catalog.find_by_id("phoenix-pt-15s-tg").unwrap();
        assert_eq!(end_plate_width_tenths_mm(pt15tg), 8);

        // WAGO 263-spec terminal block catalog verifications (incl. 2002-3217, 2002-3227, 2202-2701)
        assert_eq!(
            catalog
                .all()
                .iter()
                .filter(|s| s.manufacturer == "WAGO" && !s.is_busbar_carrier())
                .count(),
            263
        );
        let wago_2202_2701 = catalog.find_by_id("wago-2202-2701").unwrap();
        assert_eq!(wago_2202_2701.levels, 2);
        assert_eq!(wago_2202_2701.pattern_summary(), "L / L");
        assert_eq!(
            resolve_accessories(wago_2202_2701).end_plate,
            "2002-2791 (grau) / 2002-2792 (orange)"
        );
        let wago_2202_3227 = catalog.find_by_id("wago-2202-3227").unwrap();
        assert_eq!(wago_2202_3227.levels, 3);
        assert_eq!(wago_2202_3227.pattern_summary(), "PE / L / L");
        assert_eq!(end_plate_width_tenths_mm(wago_2202_3227), 8);
        assert_eq!(resolve_accessories(wago_2202_3227).end_plate, "2002-3291 (3-Etagen)");

        let wago_2035 = catalog.find_by_id("wago-2035-1201").unwrap();
        assert_eq!(end_plate_width_tenths_mm(wago_2035), 0);
        assert_eq!(wago_2035.effective_max_jumper_poles(), 2);
        assert_eq!(wago_2035.effective_jumper_slots(), 2);

        let wago_285_1185 = catalog.find_by_id("wago-285-1185").unwrap();
        assert_eq!(wago_285_1185.effective_height_tenths_mm(), 1300);

        let wago_2002_2961 = catalog.find_by_id("wago-2002-2961").unwrap();
        assert_eq!(wago_2002_2961.effective_max_jumper_poles(), 10);
        assert_eq!(end_plate_width_tenths_mm(wago_2002_2961), 10);

        for fuse_id in ["wago-2002-1611", "wago-2202-1611"] {
            let spec = catalog.find_by_id(fuse_id).unwrap();
            assert_eq!(spec.effective_rail_offset_tenths_mm(), 45);
            assert_eq!(spec.effective_jumper_slots(), 2);
            assert_eq!(end_plate_width_tenths_mm(spec), 10);
        }

        // Weidmüller 226-spec terminal block & 10-end-bracket verifications + collision checks
        let wm_specs: Vec<_> = catalog
            .all()
            .iter()
            .filter(|s| s.manufacturer == "Weidmüller")
            .collect();
        assert_eq!(
            wm_specs.iter().filter(|s| !s.is_busbar_carrier()).count(),
            226
        );
        assert_eq!(
            crate::catalog::accessories::all_end_brackets()
                .iter()
                .filter(|eb| eb.manufacturer == "Weidmüller")
                .count(),
            10
        );
        let mut wm_ids = std::collections::HashSet::new();
        let mut wm_articles = std::collections::HashSet::new();
        for s in &wm_specs {
            assert!(wm_ids.insert(s.id.as_str()), "Duplicate WM id: {}", s.id);
            let art = s
                .effective_article_number()
                .unwrap_or_else(|| panic!("Missing WM article number: {}", s.id));
            assert!(!art.is_empty(), "Empty WM article number: {}", s.id);
            assert!(
                wm_articles.insert(art.to_string()),
                "Duplicate WM article {}: {}",
                art,
                s.id
            );
        }
        let wm_a2t_vl = catalog.find_by_id("weidmuller-a2t-25-vl").unwrap();
        assert_eq!(wm_a2t_vl.effective_article_number(), Some("1547650000"));
        assert_eq!(wm_a2t_vl.pattern_summary(), "L / L");
        let wm_dld_pe = catalog.find_by_id("weidmuller-dld-25-pe").unwrap();
        assert_eq!(wm_dld_pe.effective_article_number(), Some("1783800000"));
        assert_eq!(wm_dld_pe.pattern_summary(), "PE / L / L");
        assert_eq!(wm_dld_pe.effective_rail_offset_tenths_mm(), 60);
        let wm_wnt_10 = catalog.find_by_id("weidmuller-wnt-10").unwrap();
        assert_eq!(wm_wnt_10.effective_jumper_slots(), 0);
        assert_eq!(wm_wnt_10.effective_max_jumper_poles(), 0);
        let wm_zdu_16 = catalog.find_by_id("weidmuller-zdu-16").unwrap();
        assert_eq!(end_plate_width_tenths_mm(wm_zdu_16), 27);
        let wm_a2c_95 = catalog.find_by_id("weidmuller-a2c-95-120").unwrap();
        assert_eq!(end_plate_width_tenths_mm(wm_a2c_95), 0);
        assert_eq!(
            catalog
                .find_by_id("weidmuller-aitb-25-nl")
                .unwrap()
                .matching_nt_spec_id(),
            Some("weidmuller-aitb-25-ndt")
        );
        assert_eq!(
            catalog
                .find_by_id("weidmuller-aitb-25-nlp")
                .unwrap()
                .matching_nt_spec_id(),
            Some("weidmuller-aitb-25-ntlp")
        );
    }

    #[test]
    fn busbar_carriers_and_auflagebock_specs_are_complete_and_accurate() {
        use crate::catalog::accessories::{end_plate_width_tenths_mm, resolve_accessories};

        let catalog = TerminalCatalog::load_embedded();
        let carriers: Vec<_> = catalog
            .all()
            .iter()
            .filter(|s| s.is_busbar_carrier())
            .collect();
        assert_eq!(carriers.len(), 13);

        for carrier in &carriers {
            assert_eq!(carrier.effective_jumper_slots(), 0);
            assert_eq!(carrier.effective_max_jumper_poles(), 0);
            assert_eq!(end_plate_width_tenths_mm(carrier), 0);
            assert!(!carrier.is_installation_or_nt_terminal());
            assert!(!carrier.is_n_supply_terminal());
            let _ = resolve_accessories(carrier);
        }

        let c_305 = catalog.find_by_id("wago-2009-305").unwrap();
        assert_eq!(c_305.width_tenths_mm, 75);
        assert_eq!(c_305.effective_article_number(), Some("2009-305"));
        assert!(c_305.effective_image_url().is_some());

        let c_304 = catalog.find_by_id("wago-2009-304").unwrap();
        assert_eq!(c_304.width_tenths_mm, 15);
        assert_eq!(c_304.effective_article_number(), Some("2009-304"));

        let wago_2003 = catalog.find_by_id("wago-2003-7641").unwrap();
        assert_eq!(
            wago_2003.recommended_outer_busbar_carrier_spec_id(),
            Some("wago-2009-305")
        );
        assert_eq!(
            wago_2003.recommended_intermediate_busbar_support_spec_id(),
            Some("wago-2009-304")
        );

        let phx_pti = catalog.find_by_id("phoenix-pti-25-pelnt").unwrap();
        assert_eq!(
            phx_pti.recommended_outer_busbar_carrier_spec_id(),
            Some("phoenix-ab-pti-3")
        );
        assert_eq!(
            phx_pti.recommended_intermediate_busbar_support_spec_id(),
            Some("phoenix-ab-pti-3")
        );
    }

    #[test]
    fn global_search_ignores_manufacturer_filter_when_query_present() {
        let catalog = TerminalCatalog::load_embedded();

        // With empty query, manufacturer filter applies strictly
        let wago_only = catalog.filter_specs(Some("WAGO"), "   ");
        assert!(!wago_only.is_empty());
        assert!(wago_only.iter().all(|s| s.manufacturer == "WAGO"));

        // Searching a Phoenix Contact article number while WAGO is preselected still finds Phoenix
        let hits = catalog.filter_specs(Some("WAGO"), "3209510");
        assert!(hits.iter().any(|s| s.id == "phoenix-pt-25"));

        // Searching a Weidmüller article number while Phoenix is preselected still finds Weidmüller
        let wm_hits = catalog.filter_specs(Some("Phoenix Contact"), "1547650000");
        assert!(wm_hits.iter().any(|s| s.id == "weidmuller-a2t-25-vl"));
    }

    #[test]
    fn core_wago_specs_including_2002_2201_have_direct_image_urls() {
        let catalog = TerminalCatalog::load_embedded();
        for id in [
            "wago-2002-1201",
            "wago-2002-1204",
            "wago-2002-1207",
            "wago-2002-1401",
            "wago-2002-2201",
            "wago-2002-2204",
            "wago-2002-3201",
            "wago-2003-7641",
            "wago-2003-7646",
            "wago-2202-2201",
            "wago-2203-7641",
            "wago-2203-7646",
        ] {
            let spec = catalog.find_by_id(id).unwrap_or_else(|| panic!("Missing {id}"));
            let img = spec
                .effective_image_url()
                .unwrap_or_else(|| panic!("Missing image_url for {id}"));
            assert!(
                img.starts_with("https://www.wago.com/medias/"),
                "Unexpected image URL for {id}: {img}"
            );
        }
    }

    #[test]
    fn search_on_steroids_handles_plain_text_phrases_synonyms_and_ranking() {
        let catalog = TerminalCatalog::load_embedded();

        // 1. "3 Stock klemme" / "Dreistockklemme" / "3-Etagen" must find 3-level blocks and EXCLUDE 1-level blocks
        for q in ["3 Stock klemme", "3-Stock-Klemme", "Dreistockklemme", "3 Etagen"] {
            let hits = catalog.filter_specs(Some("WAGO"), q);
            assert!(!hits.is_empty(), "No hits for '{q}'");
            assert!(
                hits.iter().all(|s| s.levels == 3),
                "Query '{q}' matched non-3-level terminal: {:?}",
                hits.iter()
                    .find(|s| s.levels != 3)
                    .map(|s| (&s.id, s.levels))
            );
            assert!(hits.iter().any(|s| s.id == "wago-2002-3201"), "Missing 2002-3201 for '{q}'");
            assert!(hits.iter().any(|s| s.id == "phoenix-pt-25-3l"), "Missing PT 2,5-3L for '{q}'");
            assert!(hits.iter().any(|s| s.id == "wago-2003-7641"), "Missing 2003-7641 for '{q}'");
        }

        // 2. "NT klemme" must find actual NT terminals without matching generic "Phoenix Contact" blocks
        let nt_hits = catalog.filter_specs(None, "NT klemme");
        assert!(nt_hits.iter().any(|s| s.id == "wago-2003-7641"));
        assert!(nt_hits.iter().any(|s| s.id == "phoenix-pti-25-pelnt"));
        assert!(nt_hits.iter().any(|s| s.id == "weidmuller-aitb-25-ndt"));
        assert!(
            !nt_hits.iter().any(|s| s.id == "phoenix-pt-25"),
            "'NT klemme' must not accidentally match 'Phoenix Contact' PT 2,5"
        );

        // 3. Combined "3 Stock NT klemme" narrows down to 3-level NT terminals
        let nt_3_hits = catalog.filter_specs(None, "3 Stock NT klemme");
        assert!(!nt_3_hits.is_empty());
        assert!(nt_3_hits.iter().all(|s| s.levels == 3));
        assert!(nt_3_hits.iter().any(|s| s.id == "wago-2003-7641"));
        assert!(nt_3_hits.iter().any(|s| s.id == "phoenix-pti-25-pelnt"));

        // 4. Plain-text synonyms: Doppelstock PE, Sicherung 5x20, 2,5mm² blau, Auflagebock
        let dp_pe = catalog.filter_specs(None, "Doppelstock PE");
        assert!(dp_pe.iter().all(|s| s.levels == 2));
        assert!(dp_pe.iter().any(|s| s.id == "phoenix-pttb-25-pe"));
        assert!(dp_pe.iter().any(|s| s.id == "wago-2002-2207"));

        let fuse_hits = catalog.filter_specs(None, "Sicherung 5x20");
        assert!(fuse_hits.iter().any(|s| s.id == "phoenix-pt-4-hesi"));

        let blue_25 = catalog.filter_specs(None, "2.5mm² blau");
        assert!(blue_25.iter().any(|s| s.id == "wago-2002-1204"));
        assert!(blue_25.iter().any(|s| s.id == "phoenix-pt-25-bu"));

        let ab_hits = catalog.filter_specs(None, "Auflagebock PTI");
        assert!(ab_hits.iter().any(|s| s.id == "phoenix-ab-pti-3"));

        // 5. Exact article number ranks #1
        let art_hits = catalog.filter_specs(Some("WAGO"), "3209510");
        assert_eq!(art_hits.first().map(|s| s.id.as_str()), Some("phoenix-pt-25"));
    }

    #[test]
    fn all_catalog_specs_and_end_brackets_have_valid_image_urls_and_unique_skus() {
        let catalog = TerminalCatalog::load_embedded();
        assert_eq!(catalog.all().len(), 782);

        let mut seen_skus = std::collections::HashSet::new();
        for spec in catalog.all() {
            let art = spec
                .effective_article_number()
                .unwrap_or_else(|| panic!("Missing article number for {}", spec.id));
            assert!(
                seen_skus.insert((spec.manufacturer.as_str(), art)),
                "Duplicate SKU ({}, {}) on {}",
                spec.manufacturer,
                art,
                spec.id
            );

            let purl = spec.effective_product_url();
            assert!(
                purl.starts_with("https://"),
                "Invalid product URL for {}: {}",
                spec.id,
                purl
            );

            let img = spec
                .effective_image_url()
                .unwrap_or_else(|| panic!("Missing image URL for {}", spec.id));
            match spec.manufacturer.as_str() {
                "WAGO" => assert!(
                    img.starts_with("https://www.wago.com/medias/"),
                    "Unexpected WAGO image URL for {}: {}",
                    spec.id,
                    img
                ),
                "Phoenix Contact" => assert!(
                    img.starts_with("https://caas.phoenixcontact.com/caas/v1/stable/media/"),
                    "Unexpected Phoenix image URL for {}: {}",
                    spec.id,
                    img
                ),
                "Weidmüller" => assert!(
                    img.starts_with("https://assets.dam.weidmueller.com/assets/api/")
                        && img.ends_with("/DAP_Master_PP,w_800,q_80"),
                    "Unexpected Weidmüller image URL for {}: {}",
                    spec.id,
                    img
                ),
                other => panic!("Unexpected manufacturer {other}"),
            }
        }

        // Spot-check key verified fixes across all three manufacturers
        assert_eq!(
            catalog
                .find_by_id("phoenix-ptpower-95-pe")
                .unwrap()
                .effective_image_url()
                .as_deref(),
            Some("https://caas.phoenixcontact.com/caas/v1/stable/media/256866/full/b408?format=jpg")
        );
        assert_eq!(
            catalog
                .find_by_id("phoenix-pti-25-lnt")
                .unwrap()
                .effective_article_number(),
            Some("3213947")
        );
        assert_eq!(
            catalog
                .find_by_id("phoenix-pti-25-pellt")
                .unwrap()
                .effective_article_number(),
            Some("3213957")
        );
        assert_eq!(
            catalog
                .find_by_id("weidmuller-zdk-25-v")
                .unwrap()
                .effective_article_number(),
            Some("1689990000")
        );
        assert_eq!(
            crate::catalog::accessories::end_bracket_by_id("wago-209-101")
                .and_then(|eb| eb.image_url),
            Some("https://it.farnell.com/productimages/large/en_US/28K7529-40.jpg")
        );

        let w2002_3217 = catalog.find_by_id("wago-2002-3217").expect("wago-2002-3217 must exist");
        assert_eq!(w2002_3217.effective_article_number(), Some("2002-3217"));
        assert_eq!(w2002_3217.levels, 3);
        assert_eq!(w2002_3217.sequential_tier_count(), 1);
        assert_eq!(w2002_3217.pattern_summary(), "PE / N / L");
        assert_eq!(w2002_3217.width_tenths_mm, 52);
        assert_eq!(w2002_3217.effective_height_tenths_mm(), 933);
        let acc_3217 = catalog.accessories_for(w2002_3217);
        assert!(acc_3217.end_plate.contains("2002-3291"));
        assert!(acc_3217.markers.contains("2009-115"));
        assert!(acc_3217.jumpers.contains("2002-402"));

        let w2002_3227 = catalog.find_by_id("wago-2002-3227").expect("wago-2002-3227 must exist");
        assert_eq!(w2002_3227.effective_article_number(), Some("2002-3227"));
        assert_eq!(w2002_3227.levels, 3);
        assert_eq!(w2002_3227.sequential_tier_count(), 2);
        assert_eq!(w2002_3227.pattern_summary(), "PE / L / L");
        assert_eq!(w2002_3227.width_tenths_mm, 52);
        assert_eq!(w2002_3227.effective_height_tenths_mm(), 933);
        let acc_3227 = catalog.accessories_for(w2002_3227);
        assert!(acc_3227.end_plate.contains("2002-3291"));

        let w2002_3231 = catalog.find_by_id("wago-2002-3231").expect("wago-2002-3231 must exist");
        assert_eq!(w2002_3231.effective_article_number(), Some("2002-3231"));
        assert_eq!(w2002_3231.levels, 3);
        assert_eq!(w2002_3231.sequential_tier_count(), 3);
        assert_eq!(w2002_3231.pattern_summary(), "L / L / L");
        assert_eq!(w2002_3231.width_tenths_mm, 52);
        assert_eq!(w2002_3231.effective_height_tenths_mm(), 933);
        let acc_3231 = catalog.accessories_for(w2002_3231);
        assert!(acc_3231.end_plate.contains("2002-3291"));

        for (id, art, w) in [
            ("phoenix-urtk-sp", "0311126", 82u16),
            ("phoenix-urtk-6", "3026272", 82u16),
            ("phoenix-qtc-15-tg", "3205145", 52u16),
        ] {
            let sp = catalog.find_by_id(id).expect(id);
            assert_eq!(sp.effective_article_number(), Some(art));
            assert_eq!(sp.width_tenths_mm, w);
        }
    }

    #[test]
    fn compatibility_rank_and_feeder_plans_work_across_families() {
        let catalog = TerminalCatalog::load_embedded();

        // 1. Compatibility sorting places exact match & same family/levels first
        let ref_2002 = catalog.find_by_id("wago-2002-1201").unwrap();
        let ranked = catalog.filter_specs_for_reference(Some("WAGO"), "", Some(ref_2002));
        assert_eq!(ranked.first().map(|s| s.id.as_str()), Some("wago-2002-1201"));
        let top_family_ids: Vec<&str> = ranked.iter().take(6).map(|s| s.id.as_str()).collect();
        assert!(top_family_ids.contains(&"wago-2002-1204"));
        assert!(top_family_ids.contains(&"wago-2002-1207"));

        let ref_pti = catalog.find_by_id("phoenix-pti-25-peln").unwrap();
        let ranked_pti =
            catalog.filter_specs_for_reference(Some("Phoenix Contact"), "", Some(ref_pti));
        let top_pti: Vec<&str> = ranked_pti.iter().take(8).map(|s| s.id.as_str()).collect();
        assert!(top_pti.contains(&"phoenix-pti-25-pelnt"));
        assert!(top_pti.contains(&"phoenix-pti-25-pell"));

        // 2. N <-> NT equivalence never touches L-only or PE-only blocks and resolves valid IDs
        for spec in catalog.all() {
            if let Some(nt_id) = spec.matching_nt_equivalent_id() {
                assert!(spec.has_neutral_tier(), "{} should have neutral tier", spec.id);
                let nt_spec = catalog.find_by_id(nt_id).unwrap_or_else(|| panic!("Missing {nt_id}"));
                assert!(
                    nt_spec.is_nt_disconnect_terminal(),
                    "{} -> {nt_id} is not NT",
                    spec.id
                );
            }
            if let Some(n_id) = spec.matching_n_from_nt_id() {
                assert!(spec.is_nt_disconnect_terminal());
                let n_spec = catalog.find_by_id(n_id).unwrap_or_else(|| panic!("Missing {n_id}"));
                assert!(n_spec.has_neutral_tier(), "{n_id} has no N tier");
                assert!(!n_spec.is_nt_disconnect_terminal());
                assert_eq!(n_spec.levels, spec.levels);
            }
        }
        assert_eq!(
            catalog.find_by_id("wago-2002-1201").unwrap().matching_nt_equivalent_id(),
            None
        );
        assert_eq!(
            catalog.find_by_id("wago-2003-7642").unwrap().matching_nt_equivalent_id(),
            None
        );
        assert_eq!(
            catalog.find_by_id("wago-2003-7646").unwrap().matching_nt_equivalent_id(),
            Some("wago-2003-7641")
        );
        assert_eq!(
            catalog.find_by_id("wago-2003-7641").unwrap().matching_n_from_nt_id(),
            Some("wago-2003-7646")
        );

        // 3. Feeder block plans (1-Polig and 3-Polig) resolve across major families
        let w1 = catalog.feeder_block_plan(ref_2002, 1).unwrap();
        assert_eq!(
            w1.specs.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(),
            vec!["wago-2002-1201", "wago-2002-1204", "wago-2002-1207"]
        );
        let w3 = catalog.feeder_block_plan(ref_2002, 3).unwrap();
        assert_eq!(
            w3.specs.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(),
            vec![
                "wago-2002-1201",
                "wago-2002-1201",
                "wago-2002-1201",
                "wago-2002-1204",
                "wago-2002-1207"
            ]
        );

        let pti_peln = catalog.find_by_id("phoenix-pti-25-peln").unwrap();
        assert_eq!(
            catalog
                .feeder_block_plan(pti_peln, 1)
                .unwrap()
                .specs
                .iter()
                .map(|s| s.id.as_str())
                .collect::<Vec<_>>(),
            vec!["phoenix-pti-25-peln"]
        );
        assert_eq!(
            catalog
                .feeder_block_plan(pti_peln, 3)
                .unwrap()
                .specs
                .iter()
                .map(|s| s.id.as_str())
                .collect::<Vec<_>>(),
            vec!["phoenix-pti-25-ll", "phoenix-pti-25-peln"]
        );

        let pti_pelnt = catalog.find_by_id("phoenix-pti-25-pelnt").unwrap();
        assert_eq!(
            catalog
                .feeder_block_plan(pti_pelnt, 3)
                .unwrap()
                .specs
                .iter()
                .map(|s| s.id.as_str())
                .collect::<Vec<_>>(),
            vec!["phoenix-pti-25-ll", "phoenix-pti-25-pelnt"]
        );

        let pttb = catalog.find_by_id("phoenix-pttb-25").unwrap();
        assert_eq!(
            catalog
                .feeder_block_plan(pttb, 3)
                .unwrap()
                .specs
                .iter()
                .map(|s| s.id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "phoenix-pttb-25",
                "phoenix-pttb-25-ln-real",
                "phoenix-pttb-25-pe"
            ]
        );

        let wm_aitb = catalog.find_by_id("weidmuller-aitb-25-ntlp").unwrap();
        assert_eq!(
            catalog
                .feeder_block_plan(wm_aitb, 3)
                .unwrap()
                .specs
                .iter()
                .map(|s| s.id.as_str())
                .collect::<Vec<_>>(),
            vec!["weidmuller-aitb-25-ll", "weidmuller-aitb-25-ntlp"]
        );
    }

    #[test]
    fn downloaded_db_priority_validation_and_hot_reload() {
        let tmp = std::env::temp_dir().join(format!(
            "termigrid_db_test_{}.json",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&tmp);

        // 1. Missing downloaded DB falls back to embedded
        let cat_fallback = TerminalCatalog::load_from_db_path_or_embedded(&tmp);
        assert_eq!(cat_fallback.all().len(), 782);

        // 2. Invalid JSON is rejected by save_and_validate_downloaded_db
        assert!(TerminalCatalog::save_and_validate_downloaded_db(&tmp, "{ invalid json").is_err());
        assert!(TerminalCatalog::save_and_validate_downloaded_db(&tmp, "[]").is_err());

        // 3. Valid downloaded DB overrides matching embedded IDs and adds new specs while keeping embedded base
        let mut custom_specs: Vec<TerminalSpec> = serde_json::from_str(EMBEDDED_SPECS_JSON).unwrap();
        let mut extra = custom_specs[0].clone();
        extra.id = "github-hot-added-spec".to_string();
        extra.name = "GH-HOT 2,5 (GitHub Live Update Test)".to_string();
        extra.article_number = Some("9999999".to_string());
        custom_specs.push(extra);

        let raw = serde_json::to_string_pretty(&custom_specs).unwrap();
        let count = TerminalCatalog::save_and_validate_downloaded_db(&tmp, &raw).unwrap();
        assert_eq!(count, 783);

        let cat_loaded = TerminalCatalog::load_from_db_path_or_embedded(&tmp);
        assert_eq!(cat_loaded.all().len(), 783);
        assert!(cat_loaded.find_by_id("github-hot-added-spec").is_some());

        // 4. Corrupting the file on disk still safely falls back to embedded on startup
        std::fs::write(&tmp, "not valid json").unwrap();
        let cat_corrupt = TerminalCatalog::load_from_db_path_or_embedded(&tmp);
        assert_eq!(cat_corrupt.all().len(), 782);

        let _ = std::fs::remove_file(&tmp);
    }
}
