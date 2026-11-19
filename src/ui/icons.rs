use egui::{
    Align2, Color32, Context, CornerRadius, FontId, Frame, Margin, Painter, Pos2, Rect, Response,
    Sense, Shape, Stroke, StrokeKind, Ui, Vec2,
};

use crate::ui::theme::tokens;

/// Consistent vector icon stroke width across all UI pictograms.
const ICON_STROKE_W: f32 = 1.5;
/// Standard inline icon size in logical pixels (`16x16`).
pub const ICON_SIZE_SM: f32 = 16.0;
/// Consistent horizontal spacing between an icon and its button label.
pub const ICON_TEXT_GAP: f32 = 6.0;

/// Unified modern vector pictograms used across toolbars, sidebars, tabs, and modals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiIcon {
    NewFile,
    FolderOpen,
    Save,
    Undo,
    Redo,
    ExportXlsx,
    ExportCsv,
    ExportDxf,
    ExternalLink,
    DownloadUpdate,
    RefreshCheck,
    Menu,
    Info,
    Language,
    ThemeSystem,
    ThemeLight,
    ThemeDark,
    SidebarLeft(bool),
    SidebarRight(bool),
    Close,
    Search,
    Clear,
    CadEditor,
    Database,
    Guide,
    ModePan,
    ModeSelect,
    ModeBridge,
    ModeEplan,
    ZoomIn,
    ZoomOut,
    ZoomReset,
    SelectAll,
    SelectNone,
    Edit,
    Add,
    Delete,
    Duplicate,
    MoveLeft,
    MoveRight,
    Apply,
    InsertBefore,
    InsertAfter,
    FlipOrder,
    Potential,
    BridgeBar,
    EndPlate,
    EndBracket,
    Settings,
    GitHub,
    Coffee,
    Bolt,
    Check,
    Library,
    Bookmark,
    ConvertNt,
    PoleBlock1,
    PoleBlock3,
    ImportPdf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonTone {
    Default,
    Ghost,
    Primary,
    Success,
    SubtleSuccess,
    Danger,
    Selected,
}

/// Draws a crisp `16x16`-normalized geometric vector pictogram inside `rect`.
pub fn draw_icon(painter: &Painter, rect: Rect, icon: UiIcon, color: Color32) {
    let side = rect.width().min(rect.height()).clamp(10.0, 24.0);
    let c = rect.center();
    let r = side * 0.5;
    let p = |nx: f32, ny: f32| Pos2::new(c.x + nx * r, c.y + ny * r);
    let st = Stroke::new(ICON_STROKE_W, color);
    let thin = Stroke::new(1.25, color);

    match icon {
        UiIcon::NewFile => {
            let pts = vec![
                p(-0.55, -0.75),
                p(0.18, -0.75),
                p(0.55, -0.38),
                p(0.55, 0.75),
                p(-0.55, 0.75),
                p(-0.55, -0.75),
            ];
            painter.add(Shape::line(pts, st));
            painter.line_segment([p(0.18, -0.75), p(0.18, -0.38)], thin);
            painter.line_segment([p(0.18, -0.38), p(0.55, -0.38)], thin);
            painter.line_segment([p(0.0, 0.0), p(0.0, 0.45)], st);
            painter.line_segment([p(-0.22, 0.225), p(0.22, 0.225)], st);
        }
        UiIcon::FolderOpen => {
            let pts = vec![
                p(-0.75, -0.55),
                p(-0.18, -0.55),
                p(-0.02, -0.35),
                p(0.72, -0.35),
                p(0.72, -0.05),
                p(-0.52, -0.05),
                p(-0.75, 0.62),
                p(-0.75, -0.55),
            ];
            painter.add(Shape::line(pts, st));
            let front = vec![
                p(-0.75, 0.62),
                p(0.55, 0.62),
                p(0.78, -0.05),
                p(-0.52, -0.05),
            ];
            painter.add(Shape::line(front, st));
        }
        UiIcon::Save => {
            let body = Rect::from_min_max(p(-0.68, -0.68), p(0.68, 0.68));
            painter.rect_stroke(body, CornerRadius::same(2), st, StrokeKind::Inside);
            let slot = Rect::from_min_max(p(-0.36, -0.68), p(0.28, -0.22));
            painter.rect_stroke(slot, CornerRadius::same(1), thin, StrokeKind::Inside);
            let label = Rect::from_min_max(p(-0.38, 0.10), p(0.38, 0.68));
            painter.rect_stroke(label, CornerRadius::same(1), thin, StrokeKind::Inside);
        }
        UiIcon::Undo => {
            let pts = vec![
                p(-0.58, -0.20),
                p(0.22, -0.20),
                p(0.50, -0.08),
                p(0.62, 0.18),
                p(0.50, 0.45),
                p(0.20, 0.55),
                p(-0.15, 0.55),
            ];
            painter.add(Shape::line(pts, st));
            painter.line_segment([p(-0.58, -0.20), p(-0.24, -0.52)], st);
            painter.line_segment([p(-0.58, -0.20), p(-0.24, 0.12)], st);
        }
        UiIcon::Redo => {
            let pts = vec![
                p(0.58, -0.20),
                p(-0.22, -0.20),
                p(-0.50, -0.08),
                p(-0.62, 0.18),
                p(-0.50, 0.45),
                p(-0.20, 0.55),
                p(0.15, 0.55),
            ];
            painter.add(Shape::line(pts, st));
            painter.line_segment([p(0.58, -0.20), p(0.24, -0.52)], st);
            painter.line_segment([p(0.58, -0.20), p(0.24, 0.12)], st);
        }
        UiIcon::ExportXlsx => {
            let grid = Rect::from_min_max(p(-0.68, -0.62), p(0.68, 0.62));
            painter.rect_stroke(grid, CornerRadius::same(2), st, StrokeKind::Inside);
            painter.line_segment([p(-0.68, -0.18), p(0.68, -0.18)], thin);
            painter.line_segment([p(-0.68, 0.22), p(0.68, 0.22)], thin);
            painter.line_segment([p(-0.14, -0.62), p(-0.14, 0.62)], thin);
            painter.line_segment([p(0.28, -0.62), p(0.28, 0.62)], thin);
        }
        UiIcon::ExportCsv => {
            let doc = Rect::from_min_max(p(-0.60, -0.70), p(0.60, 0.70));
            painter.rect_stroke(doc, CornerRadius::same(2), st, StrokeKind::Inside);
            painter.line_segment([p(-0.32, -0.32), p(0.32, -0.32)], st);
            painter.line_segment([p(-0.32, 0.0), p(0.32, 0.0)], st);
            painter.line_segment([p(-0.32, 0.32), p(0.16, 0.32)], st);
        }
        UiIcon::ExportDxf => {
            // Blueprint drafting frame + metric dimension arrow
            let box_r = Rect::from_min_max(p(-0.68, -0.65), p(0.68, 0.25));
            painter.rect_stroke(box_r, CornerRadius::same(1), st, StrokeKind::Inside);
            painter.line_segment([p(-0.22, -0.65), p(-0.22, 0.25)], thin);
            painter.line_segment([p(0.22, -0.65), p(0.22, 0.25)], thin);
            painter.line_segment([p(-0.68, 0.56), p(0.68, 0.56)], st);
            painter.line_segment([p(-0.68, 0.38), p(-0.68, 0.72)], thin);
            painter.line_segment([p(0.68, 0.38), p(0.68, 0.72)], thin);
        }
        UiIcon::ExternalLink => {
            let pts = vec![
                p(0.10, -0.58),
                p(-0.62, -0.58),
                p(-0.62, 0.62),
                p(0.58, 0.62),
                p(0.58, -0.10),
            ];
            painter.add(Shape::line(pts, st));
            painter.line_segment([p(-0.08, 0.08), p(0.62, -0.62)], st);
            painter.line_segment([p(0.22, -0.62), p(0.62, -0.62)], st);
            painter.line_segment([p(0.62, -0.22), p(0.62, -0.62)], st);
        }
        UiIcon::DownloadUpdate => {
            painter.line_segment([p(0.0, -0.68), p(0.0, 0.26)], st);
            painter.line_segment([p(-0.34, -0.06), p(0.0, 0.28)], st);
            painter.line_segment([p(0.34, -0.06), p(0.0, 0.28)], st);
            let tray = vec![
                p(-0.65, 0.32),
                p(-0.65, 0.66),
                p(0.65, 0.66),
                p(0.65, 0.32),
            ];
            painter.add(Shape::line(tray, st));
        }
        UiIcon::RefreshCheck => {
            // Circular arc + arrowheads
            let pts = vec![
                p(-0.55, -0.18),
                p(-0.36, -0.54),
                p(0.08, -0.62),
                p(0.48, -0.36),
                p(0.56, 0.08),
                p(0.30, 0.50),
                p(-0.18, 0.58),
                p(-0.52, 0.32),
            ];
            painter.add(Shape::line(pts, st));
            painter.line_segment([p(0.26, -0.62), p(0.52, -0.34)], st);
            painter.line_segment([p(0.58, -0.62), p(0.52, -0.34)], st);
        }
        UiIcon::Menu => {
            painter.line_segment([p(-0.65, -0.45), p(0.65, -0.45)], st);
            painter.line_segment([p(-0.65, 0.0), p(0.65, 0.0)], st);
            painter.line_segment([p(-0.65, 0.45), p(0.65, 0.45)], st);
        }
        UiIcon::Info => {
            painter.circle_stroke(c, r * 0.72, st);
            painter.circle_filled(p(0.0, -0.30), 1.15, color);
            painter.line_segment([p(0.0, -0.05), p(0.0, 0.40)], st);
        }
        UiIcon::Language => {
            painter.circle_stroke(c, r * 0.72, st);
            painter.line_segment([p(-0.70, 0.0), p(0.70, 0.0)], thin);
            painter.line_segment([p(0.0, -0.72), p(0.0, 0.72)], thin);
            let arc_l = vec![p(0.0, -0.72), p(-0.32, 0.0), p(0.0, 0.72)];
            let arc_r = vec![p(0.0, -0.72), p(0.32, 0.0), p(0.0, 0.72)];
            painter.add(Shape::line(arc_l, thin));
            painter.add(Shape::line(arc_r, thin));
        }
        UiIcon::ThemeSystem => {
            painter.circle_stroke(c, r * 0.70, st);
            let half = vec![
                p(0.0, -0.65),
                p(-0.45, -0.46),
                p(-0.64, 0.0),
                p(-0.45, 0.46),
                p(0.0, 0.65),
            ];
            painter.add(Shape::convex_polygon(half, color, Stroke::NONE));
        }
        UiIcon::ThemeLight => {
            painter.circle_stroke(c, r * 0.38, st);
            for i in 0..8 {
                let ang = (i as f32) * std::f32::consts::FRAC_PI_4;
                let (sn, cs) = ang.sin_cos();
                painter.line_segment(
                    [p(cs * 0.56, sn * 0.56), p(cs * 0.80, sn * 0.80)],
                    thin,
                );
            }
        }
        UiIcon::ThemeDark => {
            let outer = vec![
                p(0.20, -0.68),
                p(-0.30, -0.54),
                p(-0.58, -0.10),
                p(-0.52, 0.40),
                p(-0.10, 0.68),
                p(0.45, 0.62),
                p(0.10, 0.34),
                p(0.02, -0.10),
                p(0.20, -0.68),
            ];
            painter.add(Shape::line(outer, st));
        }
        UiIcon::SidebarLeft(open) | UiIcon::SidebarRight(open) => {
            let is_left = matches!(icon, UiIcon::SidebarLeft(_));
            let box_r = Rect::from_center_size(c, Vec2::new(side * 0.94, side * 0.74));
            let split_x = if is_left {
                box_r.min.x + box_r.width() * 0.36
            } else {
                box_r.max.x - box_r.width() * 0.36
            };
            if open {
                let pane = if is_left {
                    Rect::from_min_max(box_r.min, Pos2::new(split_x, box_r.max.y))
                } else {
                    Rect::from_min_max(Pos2::new(split_x, box_r.min.y), box_r.max)
                };
                painter.rect_filled(pane, CornerRadius::same(1), color);
            }
            painter.rect_stroke(box_r, CornerRadius::same(2), st, StrokeKind::Inside);
            painter.line_segment(
                [
                    Pos2::new(split_x, box_r.min.y + 1.0),
                    Pos2::new(split_x, box_r.max.y - 1.0),
                ],
                st,
            );
        }
        UiIcon::Close => {
            painter.line_segment([p(-0.50, -0.50), p(0.50, 0.50)], st);
            painter.line_segment([p(-0.50, 0.50), p(0.50, -0.50)], st);
        }
        UiIcon::Search => {
            let center = p(-0.14, -0.14);
            painter.circle_stroke(center, r * 0.48, st);
            painter.line_segment([p(0.20, 0.20), p(0.68, 0.68)], st);
        }
        UiIcon::Clear => {
            painter.circle_stroke(c, r * 0.68, thin);
            painter.line_segment([p(-0.30, -0.30), p(0.30, 0.30)], st);
            painter.line_segment([p(-0.30, 0.30), p(0.30, -0.30)], st);
        }
        UiIcon::CadEditor => {
            // Terminal block grid profile
            for i in [-0.46_f32, -0.02, 0.42] {
                let b = Rect::from_min_max(p(i - 0.16, -0.62), p(i + 0.16, 0.62));
                painter.rect_stroke(b, CornerRadius::same(1), thin, StrokeKind::Inside);
            }
            painter.line_segment([p(-0.74, 0.0), p(0.74, 0.0)], st);
        }
        UiIcon::Database => {
            let r1 = Rect::from_min_max(p(-0.62, -0.66), p(0.62, -0.26));
            let r2 = Rect::from_min_max(p(-0.62, -0.18), p(0.62, 0.22));
            let r3 = Rect::from_min_max(p(-0.62, 0.30), p(0.62, 0.70));
            painter.rect_stroke(r1, CornerRadius::same(2), st, StrokeKind::Inside);
            painter.rect_stroke(r2, CornerRadius::same(2), st, StrokeKind::Inside);
            painter.rect_stroke(r3, CornerRadius::same(2), st, StrokeKind::Inside);
        }
        UiIcon::Guide => {
            // Open book
            let left = vec![
                p(0.0, -0.50),
                p(-0.68, -0.62),
                p(-0.68, 0.50),
                p(0.0, 0.62),
            ];
            let right = vec![
                p(0.0, -0.50),
                p(0.68, -0.62),
                p(0.68, 0.50),
                p(0.0, 0.62),
            ];
            painter.add(Shape::line(left, st));
            painter.add(Shape::line(right, st));
            painter.line_segment([p(0.0, -0.50), p(0.0, 0.62)], thin);
        }
        UiIcon::ModePan => {
            painter.line_segment([p(-0.68, 0.0), p(0.68, 0.0)], st);
            painter.line_segment([p(0.0, -0.68), p(0.0, 0.68)], st);
            painter.line_segment([p(-0.68, 0.0), p(-0.44, -0.22)], thin);
            painter.line_segment([p(-0.68, 0.0), p(-0.44, 0.22)], thin);
            painter.line_segment([p(0.68, 0.0), p(0.44, -0.22)], thin);
            painter.line_segment([p(0.68, 0.0), p(0.44, 0.22)], thin);
            painter.line_segment([p(0.0, -0.68), p(-0.22, -0.44)], thin);
            painter.line_segment([p(0.0, -0.68), p(0.22, -0.44)], thin);
            painter.line_segment([p(0.0, 0.68), p(-0.22, 0.44)], thin);
            painter.line_segment([p(0.0, 0.68), p(0.22, 0.44)], thin);
        }
        UiIcon::ModeSelect => {
            // Corner marquee box
            let pts_tl = vec![p(-0.25, -0.62), p(-0.62, -0.62), p(-0.62, -0.25)];
            let pts_tr = vec![p(0.25, -0.62), p(0.62, -0.62), p(0.62, -0.25)];
            let pts_bl = vec![p(-0.25, 0.62), p(-0.62, 0.62), p(-0.62, 0.25)];
            let pts_br = vec![p(0.25, 0.62), p(0.62, 0.62), p(0.62, 0.25)];
            painter.add(Shape::line(pts_tl, st));
            painter.add(Shape::line(pts_tr, st));
            painter.add(Shape::line(pts_bl, st));
            painter.add(Shape::line(pts_br, st));
        }
        UiIcon::ModeBridge | UiIcon::BridgeBar => {
            // Comb jumper bar: spine + 3 contact pins
            let spine = Rect::from_min_max(p(-0.68, -0.52), p(0.68, -0.14));
            painter.rect_filled(spine, CornerRadius::same(1), color);
            for px in [-0.44_f32, 0.0, 0.44] {
                painter.line_segment([p(px, -0.14), p(px, 0.56)], Stroke::new(2.0, color));
            }
        }
        UiIcon::ModeEplan => {
            // Drafting L-square / caliper 1:1
            let pts = vec![
                p(-0.62, -0.66),
                p(-0.62, 0.62),
                p(0.66, 0.62),
                p(0.66, 0.26),
                p(-0.26, 0.26),
                p(-0.26, -0.66),
                p(-0.62, -0.66),
            ];
            painter.add(Shape::line(pts, st));
            painter.line_segment([p(-0.62, -0.20), p(-0.42, -0.20)], thin);
            painter.line_segment([p(0.20, 0.62), p(0.20, 0.42)], thin);
        }
        UiIcon::ZoomIn => {
            let center = p(-0.14, -0.14);
            painter.circle_stroke(center, r * 0.48, st);
            painter.line_segment([p(0.20, 0.20), p(0.68, 0.68)], st);
            painter.line_segment([p(-0.34, -0.14), p(0.06, -0.14)], thin);
            painter.line_segment([p(-0.14, -0.34), p(-0.14, 0.06)], thin);
        }
        UiIcon::ZoomOut => {
            let center = p(-0.14, -0.14);
            painter.circle_stroke(center, r * 0.48, st);
            painter.line_segment([p(0.20, 0.20), p(0.68, 0.68)], st);
            painter.line_segment([p(-0.34, -0.14), p(0.06, -0.14)], thin);
        }
        UiIcon::ZoomReset => {
            let pts_tl = vec![p(-0.26, -0.62), p(-0.62, -0.62), p(-0.62, -0.26)];
            let pts_tr = vec![p(0.26, -0.62), p(0.62, -0.62), p(0.62, -0.26)];
            let pts_bl = vec![p(-0.26, 0.62), p(-0.62, 0.62), p(-0.62, 0.26)];
            let pts_br = vec![p(0.26, 0.62), p(0.62, 0.62), p(0.62, 0.26)];
            painter.add(Shape::line(pts_tl, st));
            painter.add(Shape::line(pts_tr, st));
            painter.add(Shape::line(pts_bl, st));
            painter.add(Shape::line(pts_br, st));
            painter.circle_filled(c, 1.8, color);
        }
        UiIcon::SelectAll => {
            let b = Rect::from_min_max(p(-0.60, -0.60), p(0.60, 0.60));
            painter.rect_stroke(b, CornerRadius::same(2), st, StrokeKind::Inside);
            let ck = vec![p(-0.32, 0.02), p(-0.08, 0.28), p(0.36, -0.26)];
            painter.add(Shape::line(ck, st));
        }
        UiIcon::SelectNone => {
            let b = Rect::from_min_max(p(-0.58, -0.58), p(0.58, 0.58));
            painter.rect_stroke(b, CornerRadius::same(2), thin, StrokeKind::Inside);
            painter.line_segment([p(-0.64, 0.64), p(0.64, -0.64)], st);
        }
        UiIcon::Edit => {
            let pts = vec![
                p(-0.60, 0.60),
                p(-0.56, 0.20),
                p(0.28, -0.64),
                p(0.64, -0.28),
                p(-0.20, 0.56),
                p(-0.60, 0.60),
            ];
            painter.add(Shape::line(pts, st));
        }
        UiIcon::Add => {
            painter.line_segment([p(0.0, -0.58), p(0.0, 0.58)], st);
            painter.line_segment([p(-0.58, 0.0), p(0.58, 0.0)], st);
        }
        UiIcon::Delete => {
            painter.line_segment([p(-0.60, -0.40), p(0.60, -0.40)], st);
            painter.line_segment([p(-0.24, -0.40), p(-0.18, -0.66)], thin);
            painter.line_segment([p(-0.18, -0.66), p(0.18, -0.66)], thin);
            painter.line_segment([p(0.18, -0.66), p(0.24, -0.40)], thin);
            let bin = vec![
                p(-0.46, -0.40),
                p(-0.38, 0.66),
                p(0.38, 0.66),
                p(0.46, -0.40),
            ];
            painter.add(Shape::line(bin, st));
            painter.line_segment([p(-0.16, -0.14), p(-0.14, 0.42)], thin);
            painter.line_segment([p(0.16, -0.14), p(0.14, 0.42)], thin);
        }
        UiIcon::Duplicate => {
            let back = Rect::from_min_max(p(-0.62, -0.62), p(0.22, 0.22));
            let front = Rect::from_min_max(p(-0.22, -0.22), p(0.62, 0.62));
            painter.rect_stroke(back, CornerRadius::same(2), thin, StrokeKind::Inside);
            painter.rect_stroke(front, CornerRadius::same(2), st, StrokeKind::Inside);
        }
        UiIcon::MoveLeft => {
            painter.line_segment([p(0.56, 0.0), p(-0.52, 0.0)], st);
            painter.line_segment([p(-0.12, -0.44), p(-0.56, 0.0)], st);
            painter.line_segment([p(-0.12, 0.44), p(-0.56, 0.0)], st);
        }
        UiIcon::MoveRight => {
            painter.line_segment([p(-0.56, 0.0), p(0.52, 0.0)], st);
            painter.line_segment([p(0.12, -0.44), p(0.56, 0.0)], st);
            painter.line_segment([p(0.12, 0.44), p(0.56, 0.0)], st);
        }
        UiIcon::Apply | UiIcon::Check => {
            let pts = vec![p(-0.56, 0.04), p(-0.14, 0.46), p(0.58, -0.44)];
            painter.add(Shape::line(pts, Stroke::new(1.75, color)));
        }
        UiIcon::InsertBefore => {
            painter.line_segment([p(-0.58, -0.58), p(-0.58, 0.58)], st);
            painter.line_segment([p(-0.18, 0.0), p(0.60, 0.0)], st);
            painter.line_segment([p(0.14, -0.34), p(-0.20, 0.0)], st);
            painter.line_segment([p(0.14, 0.34), p(-0.20, 0.0)], st);
        }
        UiIcon::InsertAfter => {
            painter.line_segment([p(0.58, -0.58), p(0.58, 0.58)], st);
            painter.line_segment([p(-0.60, 0.0), p(0.18, 0.0)], st);
            painter.line_segment([p(-0.14, -0.34), p(0.20, 0.0)], st);
            painter.line_segment([p(-0.14, 0.34), p(0.20, 0.0)], st);
        }
        UiIcon::FlipOrder => {
            painter.line_segment([p(-0.28, -0.54), p(-0.28, 0.54)], st);
            painter.line_segment([p(-0.54, -0.26), p(-0.28, -0.56)], thin);
            painter.line_segment([p(-0.02, -0.26), p(-0.28, -0.56)], thin);
            painter.line_segment([p(0.28, -0.54), p(0.28, 0.54)], st);
            painter.line_segment([p(0.02, 0.26), p(0.28, 0.56)], thin);
            painter.line_segment([p(0.54, 0.26), p(0.28, 0.56)], thin);
        }
        UiIcon::Potential | UiIcon::Bolt => {
            let pts = vec![
                p(0.12, -0.72),
                p(-0.46, 0.08),
                p(0.04, 0.08),
                p(-0.12, 0.72),
                p(0.46, -0.08),
                p(-0.04, -0.08),
                p(0.12, -0.72),
            ];
            painter.add(Shape::line(pts, st));
        }
        UiIcon::EndPlate => {
            let blk = Rect::from_min_max(p(-0.58, -0.58), p(0.14, 0.58));
            painter.rect_stroke(blk, CornerRadius::same(1), thin, StrokeKind::Inside);
            let plate = Rect::from_min_max(p(0.32, -0.58), p(0.58, 0.58));
            painter.rect_filled(plate, CornerRadius::same(1), color);
        }
        UiIcon::EndBracket => {
            let pts = vec![
                p(-0.44, 0.58),
                p(-0.44, -0.30),
                p(0.0, -0.62),
                p(0.44, -0.30),
                p(0.44, 0.58),
                p(-0.44, 0.58),
            ];
            painter.add(Shape::line(pts, st));
            painter.line_segment([p(-0.68, 0.58), p(0.68, 0.58)], thin);
        }
        UiIcon::Settings => {
            painter.circle_stroke(c, r * 0.34, st);
            for i in 0..6 {
                let ang = (i as f32) * std::f32::consts::FRAC_PI_3;
                let (sn, cs) = ang.sin_cos();
                painter.line_segment(
                    [p(cs * 0.48, sn * 0.48), p(cs * 0.76, sn * 0.76)],
                    Stroke::new(1.8, color),
                );
            }
        }
        UiIcon::GitHub => {
            // Code branches icon
            painter.circle_stroke(p(-0.34, -0.44), r * 0.22, st);
            painter.circle_stroke(p(-0.34, 0.44), r * 0.22, st);
            painter.circle_stroke(p(0.36, -0.10), r * 0.22, st);
            painter.line_segment([p(-0.34, -0.22), p(-0.34, 0.22)], st);
            let branch = vec![p(-0.34, 0.22), p(0.0, 0.22), p(0.36, 0.12)];
            painter.add(Shape::line(branch, st));
        }
        UiIcon::Coffee => {
            let cup = vec![
                p(-0.56, -0.14),
                p(-0.46, 0.58),
                p(0.34, 0.58),
                p(0.44, -0.14),
                p(-0.56, -0.14),
            ];
            painter.add(Shape::line(cup, st));
            let handle = vec![
                p(0.44, -0.02),
                p(0.70, 0.04),
                p(0.70, 0.34),
                p(0.40, 0.40),
            ];
            painter.add(Shape::line(handle, thin));
            painter.line_segment([p(-0.24, -0.34), p(-0.18, -0.62)], thin);
            painter.line_segment([p(0.08, -0.34), p(0.14, -0.62)], thin);
        }
        UiIcon::Library => {
            // Three upright catalog / template spines on a shelf
            let b1 = Rect::from_min_max(p(-0.68, -0.64), p(-0.28, 0.64));
            let b2 = Rect::from_min_max(p(-0.18, -0.64), p(0.22, 0.64));
            painter.rect_stroke(b1, CornerRadius::same(1), st, StrokeKind::Inside);
            painter.rect_stroke(b2, CornerRadius::same(1), st, StrokeKind::Inside);
            let b3 = vec![
                p(0.32, -0.58),
                p(0.58, -0.64),
                p(0.76, 0.56),
                p(0.50, 0.64),
                p(0.32, -0.58),
            ];
            painter.add(Shape::line(b3, st));
            painter.line_segment([p(-0.78, 0.68), p(0.78, 0.68)], thin);
        }
        UiIcon::Bookmark => {
            let ribbon = vec![
                p(-0.42, -0.68),
                p(0.42, -0.68),
                p(0.42, 0.72),
                p(0.0, 0.34),
                p(-0.42, 0.72),
                p(-0.42, -0.68),
            ];
            painter.add(Shape::line(ribbon, st));
        }
        UiIcon::ConvertNt => {
            // Bidirectional swap arrows + knife-disconnect slider bar
            painter.line_segment([p(-0.62, -0.34), p(0.54, -0.34)], st);
            painter.line_segment([p(0.26, -0.60), p(0.56, -0.34)], st);
            painter.line_segment([p(0.26, -0.08), p(0.56, -0.34)], st);
            painter.line_segment([p(0.62, 0.34), p(-0.54, 0.34)], st);
            painter.line_segment([p(-0.26, 0.08), p(-0.56, 0.34)], st);
            painter.line_segment([p(-0.26, 0.60), p(-0.56, 0.34)], st);
            painter.circle_filled(p(0.0, 0.0), 1.35, color);
        }
        UiIcon::PoleBlock1 => {
            // Single phase + N + PE compact circuit bar (vertical terminal slice + "+" dot)
            let b1 = Rect::from_min_max(p(-0.62, -0.58), p(-0.22, 0.58));
            let b2 = Rect::from_min_max(p(-0.10, -0.58), p(0.30, 0.58));
            let b3 = Rect::from_min_max(p(0.42, -0.58), p(0.72, 0.58));
            painter.rect_stroke(b1, CornerRadius::same(1), st, StrokeKind::Inside);
            painter.rect_stroke(b2, CornerRadius::same(1), thin, StrokeKind::Inside);
            painter.rect_stroke(b3, CornerRadius::same(1), thin, StrokeKind::Inside);
            painter.line_segment([p(-0.42, -0.18), p(-0.42, 0.18)], thin);
        }
        UiIcon::PoleBlock3 => {
            // Three phase slices + N + PE circuit block
            for (idx, x0) in [-0.74f32, -0.40, -0.06].into_iter().enumerate() {
                let r_slice = Rect::from_min_max(p(x0, -0.58), p(x0 + 0.24, 0.58));
                painter.rect_stroke(
                    r_slice,
                    CornerRadius::same(1),
                    if idx == 0 { st } else { thin },
                    StrokeKind::Inside,
                );
            }
            let r_n = Rect::from_min_max(p(0.28, -0.58), p(0.50, 0.58));
            let r_pe = Rect::from_min_max(p(0.58, -0.58), p(0.80, 0.58));
            painter.rect_stroke(r_n, CornerRadius::same(1), thin, StrokeKind::Inside);
            painter.rect_stroke(r_pe, CornerRadius::same(1), thin, StrokeKind::Inside);
        }
        UiIcon::ImportPdf => {
            // Document outline with folded corner + incoming arrow from left/top
            let doc = vec![
                p(-0.35, -0.75),
                p(0.26, -0.75),
                p(0.62, -0.38),
                p(0.62, 0.75),
                p(-0.55, 0.75),
                p(-0.55, 0.25),
            ];
            painter.add(Shape::line(doc, st));
            painter.line_segment([p(0.26, -0.75), p(0.26, -0.38)], thin);
            painter.line_segment([p(0.26, -0.38), p(0.62, -0.38)], thin);
            // Incoming right-down arrow into sheet
            painter.line_segment([p(-0.78, -0.18), p(0.08, -0.18)], st);
            painter.line_segment([p(-0.18, -0.44), p(0.10, -0.18)], st);
            painter.line_segment([p(-0.18, 0.08), p(0.10, -0.18)], st);
            // Table row lines inside document
            painter.line_segment([p(-0.22, 0.24), p(0.36, 0.24)], thin);
            painter.line_segment([p(-0.22, 0.48), p(0.36, 0.48)], thin);
        }
    }
}

/// Renders a unified button with a crisp `16x16` vector icon on the left, clean spacing, and tactile press feedback.
pub fn icon_button_tone(ui: &mut Ui, icon: UiIcon, label: &str, tone: ButtonTone) -> Response {
    let t = tokens(ui);
    let strong = matches!(
        tone,
        ButtonTone::Primary | ButtonTone::Success | ButtonTone::Selected
    );
    let font_id = ui
        .style()
        .text_styles
        .get(&egui::TextStyle::Button)
        .cloned()
        .unwrap_or_else(|| FontId::proportional(13.5));

    let temp_galley =
        ui.painter()
            .layout_no_wrap(label.to_string(), font_id.clone(), Color32::WHITE);
    let text_size = temp_galley.size();
    let pad = ui.spacing().button_padding;
    let min_h = if cfg!(target_os = "android") {
        34.0
    } else {
        26.0
    };
    let has_label = !label.is_empty();
    let content_w = ICON_SIZE_SM
        + if has_label {
            ICON_TEXT_GAP + text_size.x
        } else {
            0.0
        };
    let total_size = Vec2::new(
        content_w + pad.x * 2.0,
        (text_size.y.max(ICON_SIZE_SM) + pad.y * 2.0).max(min_h),
    );

    let (rect, response) = ui.allocate_exact_size(total_size, Sense::click());
    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        let active = response.is_pointer_button_down_on();
        let enabled = ui.is_enabled();

        let (fill, stroke, fg) = match tone {
            ButtonTone::Primary => {
                let fill = if active {
                    t.accent_active
                } else if hovered {
                    t.accent_hover
                } else {
                    t.accent
                };
                (
                    fill,
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 36)),
                    Color32::WHITE,
                )
            }
            ButtonTone::Success => {
                let fill = if active {
                    Color32::from_rgb(20, 83, 45)
                } else if hovered {
                    t.success_hover
                } else {
                    t.success
                };
                (
                    fill,
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 34)),
                    Color32::WHITE,
                )
            }
            ButtonTone::SubtleSuccess => {
                if active {
                    (t.success, Stroke::new(1.0, t.success_hover), Color32::WHITE)
                } else if hovered {
                    (
                        t.success_soft,
                        Stroke::new(1.0, t.success_hover),
                        if t.dark {
                            Color32::from_rgb(134, 239, 172)
                        } else {
                            Color32::from_rgb(21, 128, 61)
                        },
                    )
                } else {
                    (
                        t.bg_surface,
                        Stroke::new(1.0, t.border_hairline),
                        if t.dark {
                            Color32::from_rgb(134, 239, 172)
                        } else {
                            Color32::from_rgb(21, 128, 61)
                        },
                    )
                }
            }
            ButtonTone::Danger => {
                if active {
                    (
                        Color32::from_rgb(153, 27, 27),
                        Stroke::new(1.0, t.danger_hover),
                        Color32::WHITE,
                    )
                } else if hovered {
                    (t.danger_hover, Stroke::new(1.0, t.danger_hover), Color32::WHITE)
                } else {
                    (
                        t.danger_soft,
                        Stroke::new(
                            1.0,
                            Color32::from_rgba_unmultiplied(239, 68, 68, if t.dark { 75 } else { 65 }),
                        ),
                        if t.dark {
                            Color32::from_rgb(252, 165, 165)
                        } else {
                            Color32::from_rgb(185, 28, 28)
                        },
                    )
                }
            }
            ButtonTone::Selected => (
                t.accent,
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 42)),
                Color32::WHITE,
            ),
            ButtonTone::Ghost => {
                if active {
                    (t.accent_soft, Stroke::new(1.0, t.accent), t.text_primary)
                } else if hovered {
                    (t.bg_surface_hover, Stroke::new(1.0, t.border_hairline), t.text_primary)
                } else {
                    (Color32::TRANSPARENT, Stroke::NONE, t.text_secondary)
                }
            }
            ButtonTone::Default => {
                let fill = if active {
                    t.bg_recessed
                } else if hovered {
                    t.bg_surface_hover
                } else {
                    t.bg_surface
                };
                let stroke = if active {
                    Stroke::new(1.0, t.accent)
                } else if hovered {
                    Stroke::new(1.0, t.border_strong)
                } else {
                    Stroke::new(1.0, t.border_hairline)
                };
                (fill, stroke, t.text_primary)
            }
        };

        let (final_fill, final_stroke, final_fg) = if enabled {
            (fill, stroke, fg)
        } else {
            (
                t.bg_panel,
                Stroke::new(1.0, t.border_hairline),
                t.text_muted.gamma_multiply(0.7),
            )
        };

        // Tactile `:active` micro-press feedback (Emil Kowalski design-eng rule)
        let draw_rect = if enabled && active {
            rect.shrink(0.6)
        } else {
            rect
        };
        let radius = CornerRadius::same(6);
        let painter = ui.painter();
        if final_fill != Color32::TRANSPARENT {
            painter.rect_filled(draw_rect, radius, final_fill);
        }
        if final_stroke != Stroke::NONE {
            painter.rect_stroke(draw_rect, radius, final_stroke, StrokeKind::Inside);
        }

        let icon_rect = Rect::from_center_size(
            Pos2::new(draw_rect.min.x + pad.x + ICON_SIZE_SM * 0.5, draw_rect.center().y),
            Vec2::splat(ICON_SIZE_SM),
        );
        draw_icon(painter, icon_rect, icon, final_fg);

        if has_label {
            let rich = if strong {
                egui::RichText::new(label).strong().color(final_fg)
            } else {
                egui::RichText::new(label).color(final_fg)
            };
            let galley = egui::WidgetText::from(rich).into_galley(
                ui,
                Some(egui::TextWrapMode::Extend),
                f32::INFINITY,
                egui::TextStyle::Button,
            );
            let text_pos = Pos2::new(
                draw_rect.min.x + pad.x + ICON_SIZE_SM + ICON_TEXT_GAP,
                draw_rect.center().y - galley.size().y * 0.5,
            );
            painter.galley(text_pos, galley, final_fg);
        }
    }

    response
}

pub fn icon_button(ui: &mut Ui, icon: UiIcon, label: &str) -> Response {
    icon_button_tone(ui, icon, label, ButtonTone::Default)
}

pub fn ghost_icon_button(ui: &mut Ui, icon: UiIcon, label: &str) -> Response {
    icon_button_tone(ui, icon, label, ButtonTone::Ghost)
}

pub fn primary_icon_button(ui: &mut Ui, icon: UiIcon, label: &str) -> Response {
    icon_button_tone(ui, icon, label, ButtonTone::Primary)
}

pub fn success_icon_button(ui: &mut Ui, icon: UiIcon, label: &str) -> Response {
    icon_button_tone(ui, icon, label, ButtonTone::Success)
}

pub fn subtle_success_icon_button(ui: &mut Ui, icon: UiIcon, label: &str) -> Response {
    icon_button_tone(ui, icon, label, ButtonTone::SubtleSuccess)
}

pub fn danger_icon_button(ui: &mut Ui, icon: UiIcon, label: &str) -> Response {
    icon_button_tone(ui, icon, label, ButtonTone::Danger)
}

pub fn selectable_icon_button(ui: &mut Ui, selected: bool, icon: UiIcon, label: &str) -> Response {
    let tone = if selected {
        ButtonTone::Selected
    } else {
        ButtonTone::Default
    };
    icon_button_tone(ui, icon, label, tone)
}

/// Recessed Base-UI / Linear style segmented control for primary views and canvas modes.
pub fn segmented_control<T: Copy + PartialEq>(
    ui: &mut Ui,
    current: &mut T,
    items: &[(T, UiIcon, &str, &str)],
) -> bool {
    let t = tokens(ui);
    let mut changed = false;
    Frame::NONE
        .fill(t.bg_recessed)
        .stroke(Stroke::new(1.0, t.border_hairline))
        .corner_radius(CornerRadius::same(7))
        .inner_margin(Margin::same(2))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.x = 2.0;
            ui.horizontal(|ui| {
                for &(value, icon, label, tip) in items {
                    let is_active = *current == value;
                    let font_id = ui
                        .style()
                        .text_styles
                        .get(&egui::TextStyle::Button)
                        .cloned()
                        .unwrap_or_else(|| FontId::proportional(13.0));
                    let text_w = ui
                        .painter()
                        .layout_no_wrap(label.to_string(), font_id, Color32::WHITE)
                        .size()
                        .x;
                    let h = if cfg!(target_os = "android") { 30.0 } else { 24.0 };
                    let w = 8.0 + ICON_SIZE_SM + 5.0 + text_w + 10.0;
                    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, h), Sense::click());
                    if ui.is_rect_visible(rect) {
                        let hovered = resp.hovered();
                        let pressed = resp.is_pointer_button_down_on();
                        let draw_rect = if pressed { rect.shrink(0.5) } else { rect };
                        if is_active {
                            ui.painter().rect_filled(
                                draw_rect,
                                CornerRadius::same(5),
                                t.bg_surface_elevated,
                            );
                            ui.painter().rect_stroke(
                                draw_rect,
                                CornerRadius::same(5),
                                Stroke::new(1.0, t.border_strong),
                                StrokeKind::Inside,
                            );
                        } else if hovered {
                            ui.painter().rect_filled(
                                draw_rect,
                                CornerRadius::same(5),
                                t.bg_surface_hover.gamma_multiply(0.7),
                            );
                        }
                        let fg = if is_active || hovered {
                            t.text_primary
                        } else {
                            t.text_secondary
                        };
                        let icon_col = if is_active { t.accent } else { fg };
                        let icon_rect = Rect::from_center_size(
                            Pos2::new(draw_rect.min.x + 8.0 + ICON_SIZE_SM * 0.5, draw_rect.center().y),
                            Vec2::splat(ICON_SIZE_SM),
                        );
                        draw_icon(ui.painter(), icon_rect, icon, icon_col);
                        let rich = if is_active {
                            egui::RichText::new(label).strong().color(fg)
                        } else {
                            egui::RichText::new(label).color(fg)
                        };
                        let galley = egui::WidgetText::from(rich).into_galley(
                            ui,
                            Some(egui::TextWrapMode::Extend),
                            f32::INFINITY,
                            egui::TextStyle::Button,
                        );
                        ui.painter().galley(
                            Pos2::new(
                                draw_rect.min.x + 8.0 + ICON_SIZE_SM + 5.0,
                                draw_rect.center().y - galley.size().y * 0.5,
                            ),
                            galley,
                            fg,
                        );
                    }
                    let resp = if tip.is_empty() {
                        resp
                    } else {
                        resp.on_hover_text(tip)
                    };
                    if resp.clicked() && *current != value {
                        *current = value;
                        changed = true;
                    }
                }
            });
        });
    changed
}

/// Compact square icon-only button for toolbars and window headers.
pub fn icon_only_button(ui: &mut Ui, icon: UiIcon, active: bool, tooltip: &str) -> Response {
    let t = tokens(ui);
    let side = if cfg!(target_os = "android") {
        34.0
    } else {
        26.0
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(side), Sense::click());
    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        let pressed = response.is_pointer_button_down_on();
        let enabled = ui.is_enabled();
        let draw_rect = if enabled && pressed {
            rect.shrink(0.5)
        } else {
            rect
        };
        let radius = CornerRadius::same(6);
        if active {
            ui.painter().rect_filled(draw_rect, radius, t.accent_soft);
            ui.painter().rect_stroke(
                draw_rect,
                radius,
                Stroke::new(1.0, t.accent),
                StrokeKind::Inside,
            );
        } else if !enabled {
            ui.painter().rect_filled(draw_rect, radius, t.bg_panel);
        } else if hovered {
            ui.painter()
                .rect_filled(draw_rect, radius, t.bg_surface_hover);
            ui.painter().rect_stroke(
                draw_rect,
                radius,
                Stroke::new(1.0, t.border_strong),
                StrokeKind::Inside,
            );
        } else {
            ui.painter()
                .rect_filled(draw_rect, radius, t.bg_surface);
            ui.painter().rect_stroke(
                draw_rect,
                radius,
                Stroke::new(1.0, t.border_hairline),
                StrokeKind::Inside,
            );
        }
        let fg = if !enabled {
            t.text_muted.gamma_multiply(0.5)
        } else if active {
            t.accent_text
        } else if hovered {
            t.text_primary
        } else {
            t.text_secondary
        };
        let icon_rect = Rect::from_center_size(draw_rect.center(), Vec2::splat(ICON_SIZE_SM));
        draw_icon(ui.painter(), icon_rect, icon, fg);
    }
    response.on_hover_text(tooltip)
}

/// Renders a crisp, compact manufacturer pill badge (`PHX`, `WAGO`, `WM`) for search results.
pub fn compact_mfg_badge(ui: &mut Ui, manufacturer: &str) {
    let (tag, bg) = match manufacturer {
        "Phoenix Contact" => ("PHX", Color32::from_rgb(13, 148, 136)),
        "WAGO" => ("WAGO", Color32::from_rgb(22, 163, 74)),
        "Weidmüller" => ("WM", Color32::from_rgb(217, 119, 6)),
        _ => ("CUS", Color32::from_rgb(100, 116, 139)),
    };
    Frame::NONE
        .fill(bg)
        .corner_radius(CornerRadius::same(4))
        .inner_margin(Margin::symmetric(6, 1))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(tag)
                    .color(Color32::WHITE)
                    .strong()
                    .size(10.5),
            );
        });
}

/// Renders a responsive modal window sized to `min(92vw, desired_w)` × `max 88vh`,
/// with a subtle backdrop scrim, fixed header, scrollable body, and fixed footer.
#[allow(clippy::too_many_arguments)]
pub fn show_responsive_modal<T>(
    ctx: &Context,
    id: &str,
    icon: UiIcon,
    title: &str,
    desired_max_w: f32,
    open: &mut bool,
    state: &mut T,
    render_body: impl FnOnce(&mut Ui, &mut T),
    render_footer: impl FnOnce(&mut Ui, &mut T, &mut bool),
) {
    if !*open {
        return;
    }

    let dark = ctx.theme() == egui::Theme::Dark;
    let t = crate::ui::theme::ThemeTokens::for_mode(dark);
    let screen_rect = ctx.content_rect();
    egui::Area::new(egui::Id::unique(format!("{id}_backdrop")))
        .order(egui::Order::Middle)
        .fixed_pos(screen_rect.min)
        .interactable(false)
        .show(ctx, |ui| {
            ui.painter().rect_filled(
                screen_rect,
                CornerRadius::ZERO,
                Color32::from_black_alpha(if dark { 125 } else { 70 }),
            );
        });

    let viewport = screen_rect.size();
    let max_w = (viewport.x * 0.92)
        .min(desired_max_w)
        .max(260.0_f32.min((viewport.x - 12.0).max(200.0)));
    let max_h = (viewport.y * 0.88).max(200.0_f32.min((viewport.y - 12.0).max(180.0)));
    let header_and_footer_reserve = 116.0;
    let body_max_h = (max_h - header_and_footer_reserve).max(90.0);

    let mut close_requested = false;

    egui::Window::new("")
        .id(egui::Id::unique(id))
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .frame(
            Frame::NONE
                .fill(t.bg_surface_elevated)
                .stroke(Stroke::new(1.0, t.border_strong))
                .corner_radius(CornerRadius::same(10))
                .inner_margin(Margin::same(16))
                .shadow(egui::Shadow {
                    offset: [0, 10],
                    blur: 28,
                    spread: 0,
                    color: Color32::from_black_alpha(if dark { 115 } else { 38 }),
                }),
        )
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .default_width(max_w)
        .min_width(max_w)
        .max_width(max_w)
        .max_height(max_h)
        .show(ctx, |ui| {
            ui.set_max_width(ui.available_width());

            // Fixed Header
            ui.horizontal(|ui| {
                Frame::NONE
                    .fill(t.accent_soft)
                    .corner_radius(CornerRadius::same(6))
                    .inner_margin(Margin::same(5))
                    .show(ui, |ui| {
                        let (icon_r, _) =
                            ui.allocate_exact_size(Vec2::splat(16.0), egui::Sense::hover());
                        draw_icon(ui.painter(), icon_r, icon, t.accent_text);
                    });
                ui.add_space(6.0);
                ui.add(
                    egui::Label::new(egui::RichText::new(title).strong().size(16.5)).wrap(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if icon_only_button(ui, UiIcon::Close, false, "Schließen").clicked() {
                        close_requested = true;
                    }
                });
            });
            ui.add_space(6.0);
            ui.separator();
            ui.add_space(4.0);

            // Scrollable Middle Content
            egui::ScrollArea::vertical()
                .id_salt(format!("{id}_body_scroll"))
                .max_height(body_max_h)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.set_max_width(ui.available_width());
                    render_body(ui, state);
                });

            // Fixed Footer
            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                render_footer(ui, state, &mut close_requested);
            });
        });

    if close_requested {
        *open = false;
    }
}
