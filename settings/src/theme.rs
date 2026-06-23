//! Theme definition and application for the settings GUI.

use eframe::egui::Color32;

#[derive(Clone)]
pub struct Theme {
    pub bg: Color32,
    pub bg_card: Color32,
    pub bg_hover: Color32,
    pub bg_input: Color32,
    pub accent: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_muted: Color32,
    pub border: Color32,
    pub separator: Color32,
    pub tab_active: Color32,
    pub tab_inactive: Color32,
    pub success: Color32,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            bg: Color32::from_rgb(32, 33, 36),
            bg_card: Color32::from_rgb(45, 46, 50),
            bg_hover: Color32::from_rgb(55, 56, 62),
            bg_input: Color32::from_rgb(38, 39, 43),
            accent: Color32::from_rgb(88, 166, 255),
            text: Color32::from_rgb(232, 232, 232),
            text_dim: Color32::from_rgb(180, 180, 185),
            text_muted: Color32::from_rgb(120, 120, 128),
            border: Color32::from_rgb(55, 56, 62),
            separator: Color32::from_rgb(50, 51, 56),
            tab_active: Color32::from_rgb(88, 166, 255),
            tab_inactive: Color32::from_rgb(140, 140, 148),
            success: Color32::from_rgb(76, 175, 80),
        }
    }
}

pub fn apply_theme(ctx: &eframe::egui::Context, theme: &Theme) {
    let mut style = (*ctx.style()).clone();

    let visuals = &mut style.visuals;
    visuals.dark_mode = true;
    visuals.override_text_color = Some(theme.text);
    visuals.widgets.noninteractive.bg_fill = theme.bg_card;
    visuals.widgets.noninteractive.fg_stroke = eframe::egui::Stroke::new(1.0, theme.text_dim);
    visuals.widgets.noninteractive.bg_stroke = eframe::egui::Stroke::new(0.5, theme.border);
    visuals.widgets.noninteractive.corner_radius = eframe::egui::CornerRadius::same(6);

    visuals.widgets.inactive.bg_fill = theme.bg_input;
    visuals.widgets.inactive.fg_stroke = eframe::egui::Stroke::new(1.0, theme.text);
    visuals.widgets.inactive.bg_stroke = eframe::egui::Stroke::new(0.5, theme.border);
    visuals.widgets.inactive.corner_radius = eframe::egui::CornerRadius::same(6);

    visuals.widgets.hovered.bg_fill = theme.bg_hover;
    visuals.widgets.hovered.fg_stroke = eframe::egui::Stroke::new(1.0, theme.text);
    visuals.widgets.hovered.bg_stroke = eframe::egui::Stroke::new(1.0, theme.accent);
    visuals.widgets.hovered.corner_radius = eframe::egui::CornerRadius::same(6);

    visuals.widgets.active.bg_fill = theme.accent;
    visuals.widgets.active.fg_stroke = eframe::egui::Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.active.corner_radius = eframe::egui::CornerRadius::same(6);

    visuals.selection.bg_fill = theme.accent.linear_multiply(0.3);
    visuals.selection.stroke = eframe::egui::Stroke::new(1.0, theme.accent);

    visuals.extreme_bg_color = theme.bg;
    visuals.faint_bg_color = theme.bg_card;
    visuals.striped = false;

    visuals.slider_trailing_fill = true;

    style.spacing.item_spacing = eframe::egui::vec2(8.0, 6.0);
    style.spacing.indent = 18.0;
    style.spacing.button_padding = eframe::egui::vec2(12.0, 6.0);
    style.spacing.window_margin = eframe::egui::Margin::same(16);

    ctx.set_style(style);
}
