use egui::{Color32, ColorImage, CornerRadius, Stroke, TextureHandle, TextureOptions, Ui, Vec2};

const WAGO_PNG: &[u8] = include_bytes!("../../assets/logos/wago.png");
const PHOENIX_PNG: &[u8] = include_bytes!("../../assets/logos/phoenix.png");
const WEIDMULLER_PNG: &[u8] = include_bytes!("../../assets/logos/weidmuller.png");

/// Cached GPU textures for the embedded manufacturer logos.
pub struct BrandLogos {
    pub wago: TextureHandle,
    pub phoenix: TextureHandle,
    pub weidmuller: TextureHandle,
}

impl BrandLogos {
    /// Decodes and uploads the embedded PNG manufacturer logos into `ctx`.
    pub fn load(ctx: &egui::Context) -> Self {
        Self {
            wago: load_png_texture(ctx, "logo_wago", WAGO_PNG),
            phoenix: load_png_texture(ctx, "logo_phoenix", PHOENIX_PNG),
            weidmuller: load_png_texture(ctx, "logo_weidmuller", WEIDMULLER_PNG),
        }
    }

    /// Returns the matching texture handle for a manufacturer name.
    pub fn for_manufacturer(&self, manufacturer: &str) -> Option<&TextureHandle> {
        match manufacturer {
            "WAGO" => Some(&self.wago),
            "Phoenix Contact" => Some(&self.phoenix),
            "Weidmüller" => Some(&self.weidmuller),
            _ => None,
        }
    }
}

fn load_png_texture(ctx: &egui::Context, name: &str, bytes: &[u8]) -> TextureHandle {
    let img = image::load_from_memory(bytes)
        .expect("Embedded manufacturer PNG logo must decode")
        .to_rgba8();
    let size = [img.width() as usize, img.height() as usize];
    let color_image = ColorImage::from_rgba_unmultiplied(size, img.as_raw());
    ctx.load_texture(name, color_image, TextureOptions::LINEAR)
}

/// Renders a manufacturer logo inside a crisp light pill badge (readable in both Dark & Light mode).
pub fn show_logo_badge(ui: &mut Ui, logos: &BrandLogos, manufacturer: &str, height: f32) {
    let Some(tex) = logos.for_manufacturer(manufacturer) else {
        ui.label(egui::RichText::new(manufacturer).strong());
        return;
    };

    let tex_size = tex.size_vec2();
    let aspect = if tex_size.y > 0.0 {
        tex_size.x / tex_size.y
    } else {
        3.0
    };
    let img_size = Vec2::new((height * aspect).min(170.0), height);

    egui::Frame::NONE
        .fill(Color32::from_rgb(255, 255, 255))
        .stroke(Stroke::new(1.0, Color32::from_rgb(148, 163, 184)))
        .corner_radius(CornerRadius::same(4))
        .inner_margin(egui::Margin::symmetric(7, 4))
        .show(ui, |ui| {
            ui.add(egui::Image::new((tex.id(), img_size)));
        });
}

/// Renders a clickable manufacturer filter pill with logo + active highlight.
pub fn logo_filter_button(
    ui: &mut Ui,
    logos: &BrandLogos,
    manufacturer: &str,
    selected: bool,
    height: f32,
) -> egui::Response {
    let Some(tex) = logos.for_manufacturer(manufacturer) else {
        return ui.selectable_label(selected, manufacturer);
    };
    let tex_size = tex.size_vec2();
    let aspect = if tex_size.y > 0.0 {
        tex_size.x / tex_size.y
    } else {
        3.0
    };
    let img_size = Vec2::new((height * aspect).min(145.0), height);

    let frame = egui::Frame::NONE
        .fill(Color32::from_rgb(255, 255, 255))
        .stroke(if selected {
            Stroke::new(2.5, Color32::from_rgb(2, 132, 199))
        } else {
            Stroke::new(1.2, Color32::from_rgb(148, 163, 184))
        })
        .corner_radius(CornerRadius::same(5))
        .inner_margin(egui::Margin::symmetric(8, 4));

    let resp = frame
        .show(ui, |ui| {
            ui.add(
                egui::Image::new((tex.id(), img_size))
                    .sense(egui::Sense::click()),
            )
        })
        .inner;

    resp.on_hover_text(manufacturer)
}
