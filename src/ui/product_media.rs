use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

use egui::{
    Align2, Color32, ColorImage, CornerRadius, FontId, Pos2, Rect, Stroke, StrokeKind,
    TextureHandle, TextureOptions, Ui, Vec2,
};

use crate::catalog::accessories::EndBracketSpec;
use crate::catalog::TerminalSpec;
use crate::i18n::Language;
use crate::model::TierColor;
use crate::ui::logos::{show_logo_badge, BrandLogos};
use crate::ui::net::{http_get_bytes, http_get_text};
use crate::ui::theme::secondary_text_color;
use crate::ui::updater::open_external_url;

enum PhotoEntry {
    Loading,
    Ready(TextureHandle),
    Unavailable,
}

#[derive(Default)]
struct SharedQueue {
    completed: BTreeMap<String, Option<ColorImage>>,
}

/// Async cache for fetching & displaying manufacturer product photos when reachable,
/// with automatic fallback to a detailed technical profile preview.
#[derive(Clone, Default)]
pub struct ProductMediaCache {
    entries: Arc<Mutex<BTreeMap<String, PhotoEntry>>>,
    queue: Arc<Mutex<SharedQueue>>,
    in_flight: Arc<Mutex<BTreeSet<String>>>,
}

impl ProductMediaCache {
    /// Polls any background-decoded images into GPU textures.
    pub fn poll_uploads(&self, ctx: &egui::Context) {
        let ready: Vec<(String, Option<ColorImage>)> = {
            let Ok(mut q) = self.queue.lock() else {
                return;
            };
            if q.completed.is_empty() {
                return;
            }
            std::mem::take(&mut q.completed).into_iter().collect()
        };

        let Ok(mut entries) = self.entries.lock() else {
            return;
        };
        for (spec_id, maybe_img) in ready {
            if let Some(img) = maybe_img {
                let tex = ctx.load_texture(
                    format!("prod_photo_{spec_id}"),
                    img,
                    TextureOptions::LINEAR,
                );
                entries.insert(spec_id, PhotoEntry::Ready(tex));
            } else {
                entries.insert(spec_id, PhotoEntry::Unavailable);
            }
        }
    }

    /// Returns an already-loaded texture, triggers an async fetch on first view for supported open URLs,
    /// or returns `None` to show the technical terminal profile preview.
    pub fn texture_for(&self, ctx: &egui::Context, spec: &TerminalSpec) -> Option<TextureHandle> {
        let direct_image_url = spec.effective_image_url();
        let can_fetch = direct_image_url.is_some()
            || spec.manufacturer == "WAGO"
            || spec.product_url.is_some();
        self.texture_for_key(
            ctx,
            &spec.id,
            direct_image_url,
            spec.effective_product_url(),
            can_fetch,
        )
    }

    /// Returns an already-loaded texture for a TS 35 end bracket, or fetches its studio image asynchronously.
    pub fn texture_for_end_bracket(
        &self,
        ctx: &egui::Context,
        eb: &EndBracketSpec,
    ) -> Option<TextureHandle> {
        let key = format!("eb_{}", eb.id);
        let direct_image_url = eb.image_url.map(str::to_string);
        let can_fetch = direct_image_url.is_some() || eb.manufacturer == "WAGO";
        self.texture_for_key(
            ctx,
            &key,
            direct_image_url,
            eb.product_url.to_string(),
            can_fetch,
        )
    }

    fn texture_for_key(
        &self,
        ctx: &egui::Context,
        cache_key: &str,
        direct_image_url: Option<String>,
        page_url: String,
        can_fetch: bool,
    ) -> Option<TextureHandle> {
        self.poll_uploads(ctx);

        if let Ok(entries) = self.entries.lock()
            && let Some(entry) = entries.get(cache_key)
        {
            return match entry {
                PhotoEntry::Ready(tex) => Some(tex.clone()),
                PhotoEntry::Loading | PhotoEntry::Unavailable => None,
            };
        }

        if !can_fetch {
            if let Ok(mut entries) = self.entries.lock() {
                entries.insert(cache_key.to_string(), PhotoEntry::Unavailable);
            }
            return None;
        }

        let already_running = {
            let Ok(mut flight) = self.in_flight.lock() else {
                return None;
            };
            !flight.insert(cache_key.to_string())
        };
        if already_running {
            return None;
        }

        if let Ok(mut entries) = self.entries.lock() {
            entries.insert(cache_key.to_string(), PhotoEntry::Loading);
        }

        let key = cache_key.to_string();
        let queue = Arc::clone(&self.queue);
        let in_flight = Arc::clone(&self.in_flight);
        let ctx_clone = ctx.clone();

        std::thread::spawn(move || {
            let image_opt = load_cached_color_image(&key).or_else(|| {
                let bytes = direct_image_url
                    .as_deref()
                    .and_then(|u| http_get_bytes(u).ok())
                    .or_else(|| fetch_product_image_bytes(&page_url))?;
                let img = decode_color_image(&bytes)?;
                save_cached_image_bytes(&key, &bytes);
                Some(img)
            });
            if let Ok(mut q) = queue.lock() {
                q.completed.insert(key.clone(), image_opt);
            }
            if let Ok(mut flight) = in_flight.lock() {
                flight.remove(&key);
            }
            ctx_clone.request_repaint();
        });

        None
    }
}

fn photo_cache_dir() -> std::path::PathBuf {
    #[cfg(target_os = "windows")]
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        return std::path::PathBuf::from(local)
            .join("TermiGrid")
            .join("photo_cache");
    }
    #[cfg(target_os = "macos")]
    if let Ok(home) = std::env::var("HOME") {
        return std::path::PathBuf::from(home)
            .join("Library")
            .join("Caches")
            .join("TermiGrid")
            .join("photo_cache");
    }
    #[cfg(target_os = "android")]
    {
        let pkg_cache = std::path::PathBuf::from("/data/user/0/de.patrickbarbian.termigrid/cache/photo_cache");
        if pkg_cache.exists() || std::fs::create_dir_all(&pkg_cache).is_ok() {
            return pkg_cache;
        }
    }
    std::env::temp_dir().join("termigrid_photo_cache")
}

fn cache_file_path(spec_id: &str) -> std::path::PathBuf {
    let clean: String = spec_id
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    photo_cache_dir().join(format!("{clean}.img"))
}

fn load_cached_color_image(spec_id: &str) -> Option<ColorImage> {
    let path = cache_file_path(spec_id);
    let bytes = std::fs::read(path).ok()?;
    decode_color_image(&bytes)
}

fn save_cached_image_bytes(spec_id: &str, bytes: &[u8]) {
    let path = cache_file_path(spec_id);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, bytes);
}

fn decode_color_image(bytes: &[u8]) -> Option<ColorImage> {
    let dyn_img = image::load_from_memory(bytes).ok()?;
    let rgba = dyn_img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    Some(ColorImage::from_rgba_unmultiplied(size, rgba.as_raw()))
}

fn fetch_product_image_bytes(page_url: &str) -> Option<Vec<u8>> {
    let html = http_get_text(page_url).ok()?;
    let img_url = extract_product_image_url(&html, page_url)?;
    http_get_bytes(&img_url).ok()
}

fn extract_product_image_url(html: &str, base_url: &str) -> Option<String> {
    // 1. WAGO /medias/, Phoenix Contact CaaS, Weidmüller DAM, or JSON-LD/OpenGraph product image
    for marker in [
        "https://www.wago.com/medias/",
        "https://caas.phoenixcontact.com/caas/v1/stable/media/",
        "https://assets.dam.weidmueller.com/assets/api/",
        "/medias/480-",
        "/medias/768-",
        "/medias/200-",
        "\"contentUrl\":\"https://",
        "property=\"og:image\" content=\"",
        "content=\"https://",
    ] {
        if let Some(pos) = html.find(marker) {
            let slice = &html[pos..];
            let start = if marker.starts_with("\"contentUrl\":\"") {
                "\"contentUrl\":\"".len()
            } else if marker.starts_with("property=\"og:image\"") {
                "property=\"og:image\" content=\"".len()
            } else if marker.starts_with("content=\"") {
                "content=\"".len()
            } else {
                0
            };
            let rest = &slice[start..];
            let end = rest
                .find(['"', '\'', ' ', '<'])
                .unwrap_or(rest.len().min(500));
            let candidate = &rest[..end];
            if candidate.contains(".jpg")
                || candidate.contains(".jpeg")
                || candidate.contains(".png")
                || candidate.contains("/medias/")
                || candidate.contains("caas.phoenixcontact.com")
                || candidate.contains("assets.dam.weidmueller.com")
            {
                if candidate.starts_with("http") {
                    return Some(candidate.to_string());
                }
                if candidate.starts_with('/')
                    && let Some(scheme_end) = base_url.find("://")
                {
                    let tail = &base_url[scheme_end + 3..];
                    let host = tail.split('/').next().unwrap_or("");
                    return Some(format!("{}{host}{candidate}", &base_url[..scheme_end + 3]));
                }
            }
        }
    }
    None
}

/// Renders the product preview card (live photo if available + technical profile illustration,
/// width, TS 35 Bauhöhe, jumper chamber count, and clickable manufacturer product link).
#[allow(clippy::too_many_arguments)]
pub fn show_product_preview_card(
    ui: &mut Ui,
    spec: &TerminalSpec,
    logos: &BrandLogos,
    media_cache: &ProductMediaCache,
    open_product_label: &str,
    chambers_label: &str,
    height_label: &str,
) {
    let dark = ui.visuals().dark_mode;
    let sec = secondary_text_color(ui);

    ui.group(|ui| {
        ui.horizontal_wrapped(|ui| {
            show_logo_badge(ui, logos, &spec.manufacturer, 18.0);
            ui.add(egui::Label::new(egui::RichText::new(&spec.name).strong()).wrap());
        });
        ui.add_space(4.0);

        let compact = ui.available_width() < 290.0;
        let preview_size = if compact {
            Vec2::new(82.0, 62.0)
        } else {
            Vec2::new(104.0, 78.0)
        };
        let tex_opt = media_cache.texture_for(ui.ctx(), spec);

        ui.horizontal(|ui| {
            if let Some(tex) = tex_opt {
                // Real manufacturer product photo on a bright studio frame
                let (rect, _) = ui.allocate_exact_size(preview_size, egui::Sense::hover());
                ui.painter()
                    .rect_filled(rect, CornerRadius::same(4), Color32::WHITE);
                ui.painter().rect_stroke(
                    rect,
                    CornerRadius::same(4),
                    Stroke::new(1.0, Color32::from_rgb(203, 213, 225)),
                    StrokeKind::Inside,
                );
                let img = egui::Image::from_texture(&tex)
                    .max_size(preview_size - Vec2::splat(6.0))
                    .corner_radius(3);
                ui.put(rect.shrink(3.0), img);
            } else {
                draw_technical_profile(ui, spec, preview_size, dark);
            }

            ui.vertical(|ui| {
                let slots = spec.effective_jumper_slots();
                let max_poles = spec.effective_max_jumper_poles();
                let height_mm = f32::from(spec.effective_height_tenths_mm()) / 10.0;
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!(
                            "{:.1} mm | {} Etagen",
                            f32::from(spec.width_tenths_mm) / 10.0,
                            spec.levels
                        ))
                        .small(),
                    )
                    .wrap(),
                );
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!("{height_label}: {height_mm:.1} mm"))
                            .small()
                            .strong(),
                    )
                    .wrap(),
                );
                if let Some(art) = spec.effective_article_number() {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(format!("Art.-Nr.: {art}"))
                                .color(sec)
                                .small(),
                        )
                        .wrap(),
                    );
                }
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!("{slots}× {chambers_label} (max. {max_poles}p)"))
                            .color(sec)
                            .small(),
                    )
                    .wrap(),
                );
                ui.add_space(3.0);
                if ui.button(open_product_label).clicked() {
                    open_external_url(ui.ctx(), &spec.effective_product_url());
                }
            });
        });
    });
}

/// Renders the preview card for a TS 35 DIN-rail end bracket (`Endhalter`) with studio photo
/// or side-profile schematic, physical width, Bauhöhe, article number, and manufacturer link.
#[allow(clippy::too_many_arguments)]
pub fn show_end_bracket_preview_card(
    ui: &mut Ui,
    eb: &EndBracketSpec,
    logos: &BrandLogos,
    media_cache: &ProductMediaCache,
    open_product_label: &str,
    height_label: &str,
    lang: Language,
) {
    let dark = ui.visuals().dark_mode;
    let sec = secondary_text_color(ui);

    ui.group(|ui| {
        ui.horizontal_wrapped(|ui| {
            show_logo_badge(ui, logos, eb.manufacturer, 18.0);
            ui.add(egui::Label::new(egui::RichText::new(eb.name).strong()).wrap());
        });
        ui.add_space(4.0);

        let compact = ui.available_width() < 290.0;
        let preview_size = if compact {
            Vec2::new(82.0, 62.0)
        } else {
            Vec2::new(104.0, 78.0)
        };
        let tex_opt = media_cache.texture_for_end_bracket(ui.ctx(), eb);

        ui.horizontal(|ui| {
            if let Some(tex) = tex_opt {
                let (rect, _) = ui.allocate_exact_size(preview_size, egui::Sense::hover());
                ui.painter()
                    .rect_filled(rect, CornerRadius::same(4), Color32::WHITE);
                ui.painter().rect_stroke(
                    rect,
                    CornerRadius::same(4),
                    Stroke::new(1.0, Color32::from_rgb(203, 213, 225)),
                    StrokeKind::Inside,
                );
                let img = egui::Image::from_texture(&tex)
                    .max_size(preview_size - Vec2::splat(6.0))
                    .corner_radius(3);
                ui.put(rect.shrink(3.0), img);
            } else {
                draw_end_bracket_profile(ui, eb, preview_size, dark);
            }

            ui.vertical(|ui| {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!(
                            "{:.1} mm | {height_label}: {:.1} mm",
                            f32::from(eb.width_tenths_mm) / 10.0,
                            f32::from(eb.height_tenths_mm) / 10.0
                        ))
                        .small()
                        .strong(),
                    )
                    .wrap(),
                );
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!("Art.-Nr.: {}", eb.article_number))
                            .color(sec)
                            .small(),
                    )
                    .wrap(),
                );
                let mounting = match lang {
                    Language::De => eb.mounting_de,
                    _ => eb.mounting_en,
                };
                ui.add(
                    egui::Label::new(egui::RichText::new(mounting).color(sec).small()).wrap(),
                );
                ui.add_space(3.0);
                if ui.button(open_product_label).clicked() {
                    open_external_url(ui.ctx(), eb.product_url);
                }
            });
        });
    });
}

/// Draws a side-profile schematic of a TS 35 DIN-rail end bracket (`Endhalter`) when no studio photo is cached.
fn draw_end_bracket_profile(ui: &mut Ui, eb: &EndBracketSpec, size: Vec2, dark: bool) {
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter_at(rect);

    let bg = if dark {
        Color32::from_rgb(15, 23, 42)
    } else {
        Color32::from_rgb(248, 250, 252)
    };
    let border = if dark {
        Color32::from_rgb(71, 85, 105)
    } else {
        Color32::from_rgb(148, 163, 184)
    };
    painter.rect_filled(rect, CornerRadius::same(4), bg);
    painter.rect_stroke(rect, CornerRadius::same(4), Stroke::new(1.0, border), StrokeKind::Inside);

    let body_fill = if dark {
        Color32::from_rgb(71, 85, 105)
    } else {
        Color32::from_rgb(148, 163, 184)
    };
    let body_stroke = Stroke::new(
        1.2,
        if dark {
            Color32::from_rgb(203, 213, 225)
        } else {
            Color32::from_rgb(30, 41, 59)
        },
    );

    // Clamp housing on TS 35 rail
    let body = Rect::from_min_max(
        Pos2::new(rect.min.x + 18.0, rect.min.y + 22.0),
        Pos2::new(rect.max.x - 18.0, rect.max.y - 16.0),
    );
    painter.rect_filled(body, CornerRadius::same(4), body_fill);
    painter.rect_stroke(body, CornerRadius::same(4), body_stroke, StrokeKind::Inside);

    // Screw head / snap latch top detail + marker slot
    let latch = Rect::from_center_size(
        Pos2::new(rect.center().x, body.min.y + 8.0),
        Vec2::new(22.0, 7.0),
    );
    painter.rect_filled(latch, CornerRadius::same(2), Color32::from_rgb(226, 232, 240));

    // TS 35 metallic DIN-rail bar at the bottom
    let rail = Rect::from_min_max(
        Pos2::new(rect.min.x + 8.0, rect.max.y - 15.0),
        Pos2::new(rect.max.x - 8.0, rect.max.y - 9.0),
    );
    painter.rect_filled(rail, CornerRadius::same(1), Color32::from_rgb(203, 213, 225));
    painter.rect_stroke(rail, CornerRadius::same(1), Stroke::new(1.0, Color32::from_rgb(71, 85, 105)), StrokeKind::Inside);

    painter.text(
        Pos2::new(rect.center().x, rect.min.y + 4.0),
        Align2::CENTER_TOP,
        eb.name,
        FontId::proportional(9.5),
        if dark {
            Color32::from_rgb(226, 232, 240)
        } else {
            Color32::from_rgb(15, 23, 42)
        },
    );
}

/// Draws a crisp side-profile technical schematic of the selected terminal block
/// highlighting its tiers, wire entries, TS35 rail foot, and 1 vs 2 jumper slots.
fn draw_technical_profile(ui: &mut Ui, spec: &TerminalSpec, size: Vec2, dark: bool) {
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter_at(rect);

    let bg = if dark {
        Color32::from_rgb(15, 23, 42)
    } else {
        Color32::from_rgb(248, 250, 252)
    };
    let border = if dark {
        Color32::from_rgb(71, 85, 105)
    } else {
        Color32::from_rgb(148, 163, 184)
    };
    painter.rect_filled(rect, CornerRadius::same(4), bg);
    painter.rect_stroke(rect, CornerRadius::same(4), Stroke::new(1.0, border), StrokeKind::Inside);

    let has_pe = spec
        .tiers
        .iter()
        .any(|t| matches!(t.behavior, crate::model::TierBehavior::ProtectiveEarth));
    let has_blue = spec
        .tiers
        .iter()
        .any(|t| t.color == TierColor::BlueNeutral);

    let body_color = if spec.tiers.len() == 1 && has_pe {
        Color32::from_rgb(34, 197, 94)
    } else if spec.tiers.len() == 1 && has_blue {
        Color32::from_rgb(37, 99, 235)
    } else if dark {
        Color32::from_rgb(71, 85, 105)
    } else {
        Color32::from_rgb(203, 213, 225)
    };
    let body_stroke = Stroke::new(
        1.2,
        if dark {
            Color32::from_rgb(148, 163, 184)
        } else {
            Color32::from_rgb(51, 65, 85)
        },
    );

    // Draw stepped housing profile matching number of tiers
    let levels = spec.levels.clamp(1, 3);
    let foot_y = rect.max.y - 10.0;
    let body_w = rect.width() - 20.0;
    let base_x = rect.min.x + 10.0;

    for l in 0..levels {
        let inset = (l as f32) * 10.0;
        let step_h = match levels {
            1 => 34.0,
            2 => 22.0,
            _ => 17.0,
        };
        let top = foot_y - 6.0 - ((l + 1) as f32) * (step_h - 3.0);
        let step_rect = Rect::from_min_max(
            Pos2::new(base_x + inset, top.max(rect.min.y + 8.0)),
            Pos2::new(rect.max.x - 10.0 - inset, foot_y - 4.0),
        );
        painter.rect_filled(step_rect, CornerRadius::same(3), body_color);
        painter.rect_stroke(step_rect, CornerRadius::same(3), body_stroke, StrokeKind::Inside);

        // Conductor entry pockets left & right + orange push-button / actuator hint
        let port_y = step_rect.min.y + 6.0;
        painter.rect_filled(
            Rect::from_min_size(Pos2::new(step_rect.min.x + 4.0, port_y), Vec2::new(7.0, 5.0)),
            CornerRadius::same(1),
            Color32::from_rgb(15, 23, 42),
        );
        painter.rect_filled(
            Rect::from_min_size(Pos2::new(step_rect.max.x - 11.0, port_y), Vec2::new(7.0, 5.0)),
            CornerRadius::same(1),
            Color32::from_rgb(15, 23, 42),
        );
        if spec.id.contains("220") || spec.manufacturer == "Phoenix Contact" || spec.id.contains("a2c") {
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(step_rect.min.x + 13.0, port_y), Vec2::new(5.0, 4.0)),
                CornerRadius::same(1),
                Color32::from_rgb(249, 115, 22),
            );
        }
    }

    // Center jumper pockets (1 vs 2 slots clearly illustrated!)
    let slots = spec.effective_jumper_slots();
    let cx = rect.center().x;
    let slot_y = rect.min.y + 14.0;
    if slots >= 2 {
        painter.circle_filled(Pos2::new(cx - 5.0, slot_y), 3.0, Color32::from_rgb(245, 158, 11));
        painter.circle_filled(Pos2::new(cx + 5.0, slot_y), 3.0, Color32::from_rgb(245, 158, 11));
    } else {
        painter.circle_filled(Pos2::new(cx, slot_y), 3.0, Color32::from_rgb(245, 158, 11));
    }

    // DIN rail snap-on foot silhouette
    let rail_clip = Rect::from_min_max(
        Pos2::new(cx - body_w * 0.22, foot_y - 4.0),
        Pos2::new(cx + body_w * 0.22, foot_y + 3.0),
    );
    painter.rect_filled(
        rail_clip,
        CornerRadius::same(1),
        if has_pe {
            Color32::from_rgb(234, 179, 8)
        } else {
            body_color
        },
    );

    let short = spec.name.split(" (").next().unwrap_or(&spec.name).trim();
    painter.text(
        Pos2::new(rect.center().x, rect.min.y + 4.0),
        Align2::CENTER_TOP,
        short,
        FontId::proportional(9.5),
        if dark {
            Color32::from_rgb(226, 232, 240)
        } else {
            Color32::from_rgb(15, 23, 42)
        },
    );
}
