//! Main SettingsApp implementation.

use crate::tabs::{general, position, keycap, display, about};
use crate::theme::Theme;
use eframe::egui::{Align2, Align, ComboBox, Context, FontId, Frame, Layout, Margin, RichText, Stroke, TextEdit, Ui, Vec2};
use input_core::config::FileConfig;
use input_core::presets::ThemePreset;
use std::time::Instant;

#[derive(PartialEq, Clone, Copy)]
enum SettingsTab {
    General,
    Position,
    Keycap,
    Display,
    About,
}

impl SettingsTab {
    fn label(self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Position => "Position",
            Self::Keycap => "Keycap",
            Self::Display => "Display",
            Self::About => "About",
        }
    }
    fn all() -> &'static [SettingsTab] {
        &[Self::General, Self::Position, Self::Keycap, Self::Display, Self::About]
    }
}

pub const POSITIONS: &[&str] = &[
    "BottomCenter",
    "TopLeft",
    "TopRight",
    "TopCenter",
    "BottomLeft",
    "BottomRight",
    "Center",
];

pub const SCALES: &[&str] = &["Small", "Medium", "Large", "ExtraLarge"];
pub const THEMES: &[&str] = &["Dark", "Light", "System"];
pub const KEYCAP_STYLES: &[&str] = &["Minimal", "Laptop", "LowProfile", "PBT"];
pub const ANIMATION_TYPES: &[&str] = &["None", "Fade", "Zoom", "Float", "Slide"];
pub const TEXT_CAPS: &[&str] = &["Uppercase", "Capitalize", "Lowercase"];
pub const TEXT_VARIANTS: &[&str] = &["Full", "Short", "Icon"];

pub struct SettingsApp {
    pub config: FileConfig,
    pub theme: Theme,
    pub active_tab: SettingsTab,
    pub position_index: usize,
    pub scale_index: usize,
    pub theme_index: usize,
    pub keycap_style_index: usize,
    pub animation_type_index: usize,
    pub text_caps_index: usize,
    pub text_variant_index: usize,
    pub preset_index: usize,
    pub save_status: String,
    pub save_status_time: Option<Instant>,
}

impl SettingsApp {
    pub fn new(config: FileConfig, theme: Theme) -> Self {
        let position_index = config
            .position
            .as_deref()
            .and_then(|p| POSITIONS.iter().position(|&s| s == p))
            .unwrap_or(0);
        let scale_index = config
            .scale
            .as_deref()
            .and_then(|s| SCALES.iter().position(|&x| x == s))
            .unwrap_or(1);
        let theme_index = config
            .theme
            .as_deref()
            .and_then(|t| THEMES.iter().position(|&x| x == t))
            .unwrap_or(0);
        let keycap_style_index = config
            .keycap_style
            .as_deref()
            .and_then(|s| KEYCAP_STYLES.iter().position(|&x| x == s))
            .unwrap_or(1);
        let animation_type_index = config
            .animation_type
            .as_deref()
            .and_then(|a| ANIMATION_TYPES.iter().position(|&x| x == a))
            .unwrap_or(4);
        let text_caps_index = config
            .text_caps
            .as_deref()
            .and_then(|c| TEXT_CAPS.iter().position(|&x| x == c))
            .unwrap_or(0);
        let text_variant_index = config
            .text_variant
            .as_deref()
            .and_then(|v| TEXT_VARIANTS.iter().position(|&x| x == v))
            .unwrap_or(0);

        Self {
            config,
            theme,
            active_tab: SettingsTab::General,
            position_index,
            scale_index,
            theme_index,
            keycap_style_index,
            animation_type_index,
            text_caps_index,
            text_variant_index,
            preset_index: 0,
            save_status: String::new(),
            save_status_time: None,
        }
    }

    fn sync_to_config(&mut self) {
        self.config.position = Some(POSITIONS[self.position_index].into());
        self.config.scale = Some(SCALES[self.scale_index].into());
        self.config.theme = Some(THEMES[self.theme_index].into());
        self.config.keycap_style = Some(KEYCAP_STYLES[self.keycap_style_index].into());
        self.config.animation_type = Some(ANIMATION_TYPES[self.animation_type_index].into());
        self.config.text_caps = Some(TEXT_CAPS[self.text_caps_index].into());
        self.config.text_variant = Some(TEXT_VARIANTS[self.text_variant_index].into());
    }

    pub fn apply_preset(&mut self, preset: &ThemePreset) {
        self.config.keycap_primary = Some(preset.colors.keycap_primary.clone());
        self.config.keycap_secondary = Some(preset.colors.keycap_secondary.clone());
        self.config.use_gradient = Some(preset.colors.use_gradient);
        self.config.highlight_modifiers = Some(preset.colors.highlight_modifiers);
        self.config.modifier_primary = Some(preset.colors.modifier_primary.clone());
        self.config.modifier_secondary = Some(preset.colors.modifier_secondary.clone());
        self.config.text_size = preset.text.size;
        self.config.text_color = Some(preset.text.color.clone());
        self.config.text_modifier_color = Some(preset.text.modifier_color.clone());
        self.config.text_caps = Some(format!("{:?}", preset.text.caps));
        self.config.text_variant = Some(format!("{:?}", preset.text.variant));
        self.config.border_enabled = Some(preset.border.enabled);
        self.config.border_color = Some(preset.border.color.clone());
        self.config.border_width = Some(preset.border.width);
        self.config.border_radius = Some(preset.border.radius);
        self.config.border_modifier_color = Some(preset.border.modifier_color.clone());
        self.keycap_style_index = KEYCAP_STYLES
            .iter()
            .position(|&s| s == format!("{:?}", preset.keycap_style))
            .unwrap_or(1);
        self.text_caps_index = TEXT_CAPS
            .iter()
            .position(|&s| s == format!("{:?}", preset.text.caps))
            .unwrap_or(0);
        self.text_variant_index = TEXT_VARIANTS
            .iter()
            .position(|&s| s == format!("{:?}", preset.text.variant))
            .unwrap_or(0);
    }

    fn save(&mut self) {
        self.sync_to_config();
        match self.config.save() {
            Ok(()) => {
                self.save_status = "Saved".into();
                self.save_status_time = Some(Instant::now());
            }
            Err(e) => {
                self.save_status = format!("Error: {}", e);
                self.save_status_time = Some(Instant::now());
            }
        }
    }
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let theme = self.theme.clone();

        // ── Top Tab Bar ──
        eframe::egui::TopBottomPanel::top("tab_bar")
            .frame(
                Frame::NONE
                    .fill(theme.bg)
                    .stroke(Stroke::new(0.5, theme.separator))
                    .inner_margin(Margin::symmetric(16, 0)),
            )
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    for &tab in SettingsTab::all() {
                        let is_active = self.active_tab == tab;
                        let text_color = if is_active {
                            theme.tab_active
                        } else {
                            theme.tab_inactive
                        };
                        let btn = ui.add(
                            eframe::egui::Button::new(
                                RichText::new(tab.label())
                                    .color(text_color)
                                    .size(13.0)
                                    .strong(),
                            )
                            .fill(eframe::egui::Color32::TRANSPARENT)
                            .stroke(Stroke::NONE),
                        );

                        // Underline for active tab
                        if is_active {
                            let rect = btn.rect;
                            ui.painter().line_segment(
                                [
                                    eframe::egui::pos2(rect.left(), rect.bottom()),
                                    eframe::egui::pos2(rect.right(), rect.bottom()),
                                ],
                                Stroke::new(2.0, theme.tab_active),
                            );
                        }

                        if btn.clicked() {
                            self.active_tab = tab;
                        }
                    }
                });
                ui.add_space(4.0);
            });

        // ── Content Panel ──
        eframe::egui::CentralPanel::default()
            .frame(
                Frame::NONE
                    .fill(theme.bg)
                    .inner_margin(Margin::same(16)),
            )
            .show(ctx, |ui| {
                match self.active_tab {
                    SettingsTab::General => general::render_general_tab(ui, &theme, ctx, self),
                    SettingsTab::Position => position::render_position_tab(ui, &theme, ctx, self),
                    SettingsTab::Keycap => keycap::render_keycap_tab(ui, &theme, ctx, self),
                    SettingsTab::Display => display::render_display_tab(ui, &theme, ctx, self),
                    SettingsTab::About => about::render_about_tab(ui, &theme, ctx, self),
                }
            });
    }
}

// ── UI Helper Functions (free functions, not methods) ──

pub fn section_header(ui: &mut Ui, theme: &Theme, label: &str) {
    ui.add_space(4.0);
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), 0.0),
        eframe::egui::Sense::hover(),
    );
    ui.painter().text(
        rect.min,
        Align2::LEFT_CENTER,
        label,
        FontId::proportional(14.0),
        theme.text_dim,
    );
    ui.add_space(18.0);
}

pub fn card<F: FnOnce(&mut Ui)>(ui: &mut Ui, theme: &Theme, content: F) -> eframe::egui::Response {
    let frame = Frame::NONE
        .fill(theme.bg_card)
        .corner_radius(eframe::egui::CornerRadius::same(8))
        .stroke(Stroke::new(0.5, theme.border))
        .inner_margin(Margin::same(12));
    frame.show(ui, |ui| {
        content(ui);
    }).response
}

pub fn labeled_slider(
    ui: &mut Ui,
    theme: &Theme,
    label: &str,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    suffix: &str,
) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(theme.text_dim).size(13.0));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(
                RichText::new(format!("{:.0}{}", value, suffix))
                    .color(theme.accent)
                    .size(13.0)
                    .strong(),
            );
        });
    });
    ui.spacing_mut().slider_width = ui.available_width();
    ui.add(eframe::egui::Slider::new(value, range).suffix(suffix).show_value(false));
}

pub fn labeled_slider_f64(
    ui: &mut Ui,
    theme: &Theme,
    label: &str,
    value: &mut f64,
    range: std::ops::RangeInclusive<f64>,
    suffix: &str,
) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(theme.text_dim).size(13.0));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(
                RichText::new(format!("{:.0}{}", value, suffix))
                    .color(theme.accent)
                    .size(13.0)
                    .strong(),
            );
        });
    });
    ui.spacing_mut().slider_width = ui.available_width();
    ui.add(eframe::egui::Slider::new(value, range).suffix(suffix).show_value(false));
}

pub fn dropdown(
    ui: &mut Ui,
    theme: &Theme,
    id: &str,
    label: &str,
    options: &[&str],
    selected: &mut usize,
) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(theme.text_dim).size(13.0));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ComboBox::from_id_salt(id)
                .selected_text(RichText::new(options[*selected]).color(theme.text).size(13.0))
                .width(130.0)
                .show_ui(ui, |ui| {
                    for (i, &opt) in options.iter().enumerate() {
                        ui.selectable_value(selected, i, RichText::new(opt).size(13.0));
                    }
                });
        });
    });
}

pub fn color_row(ui: &mut Ui, theme: &Theme, label: &str, value: &mut String) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(theme.text_dim).size(13.0));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            // Color preview swatch
            if let Some(hex) = value.strip_prefix('#') {
                if hex.len() >= 6 {
                    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                    let (rect, _) = ui.allocate_exact_size(Vec2::new(14.0, 14.0), eframe::egui::Sense::hover());
                    ui.painter().rect_filled(
                        rect,
                        eframe::egui::CornerRadius::same(3),
                        eframe::egui::Color32::from_rgb(r, g, b),
                    );
                    ui.add_space(4.0);
                }
            }
            let mut color = value.clone();
            let response = ui.add(
                TextEdit::singleline(&mut color)
                    .desired_width(80.0)
                    .font(FontId::monospace(12.0)),
            );
            if response.changed() {
                *value = color;
            }
        });
    });
}

pub fn toggle_row(ui: &mut Ui, theme: &Theme, label: &str, value: &mut bool) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(theme.text_dim).size(13.0));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.toggle_value(value, "");
        });
    });
}

pub fn save_bar(ui: &mut Ui, theme: &Theme, ctx: &Context, app: &mut SettingsApp) {
    ui.add_space(8.0);
    let frame = Frame::NONE
        .fill(theme.bg_card)
        .corner_radius(eframe::egui::CornerRadius::same(8))
        .stroke(Stroke::new(0.5, theme.border))
        .inner_margin(Margin::same(12));
    frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            let save_btn = ui.add(
                eframe::egui::Button::new(
                    RichText::new("Save").size(13.0).strong(),
                )
                .fill(theme.accent)
                .corner_radius(eframe::egui::CornerRadius::same(6))
                .min_size(Vec2::new(80.0, 30.0)),
            );
            if save_btn.clicked() {
                app.save();
            }

            let close_btn = ui.add(
                eframe::egui::Button::new(
                    RichText::new("Save & Close").size(13.0),
                )
                .fill(theme.bg_hover)
                .corner_radius(eframe::egui::CornerRadius::same(6))
                .min_size(Vec2::new(100.0, 30.0)),
            );
            if close_btn.clicked() {
                app.sync_to_config();
                if app.config.save().is_ok() {
                    ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Close);
                }
            }

            // Show save status with auto-fade
            if !app.save_status.is_empty() {
                let show = match app.save_status_time {
                    Some(t) => t.elapsed() < std::time::Duration::from_secs(3),
                    None => false,
                };
                if show {
                    let color = if app.save_status == "Saved" {
                        theme.success
                    } else {
                        eframe::egui::Color32::from_rgb(255, 100, 100)
                    };
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            RichText::new(&app.save_status)
                                .color(color)
                                .size(12.0),
                        );
                    });
                } else {
                    app.save_status.clear();
                }
            }
        });
    });
}