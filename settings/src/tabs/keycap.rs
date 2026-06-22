//! Keycap settings tab.

use crate::theme::Theme;
use crate::app::{KEYCAP_STYLES, TEXT_CAPS, TEXT_VARIANTS, card, dropdown, labeled_slider, color_row, toggle_row, section_header, save_bar};
use eframe::egui::{ComboBox, Context, RichText, Ui};
use input_core::presets::ThemePreset;

pub fn render_keycap_tab(ui: &mut Ui, theme: &Theme, ctx: &Context, app: &mut crate::app::SettingsApp) {
    // ── Preset ──
    card(ui, theme, |ui| {
        let presets = ThemePreset::all();
        let preset_names: Vec<String> = presets.iter().map(|p| p.name.clone()).collect();

        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Preset")
                    .color(theme.text_dim)
                    .size(13.0),
            );
            ui.with_layout(eframe::egui::Layout::right_to_left(eframe::egui::Align::Center), |ui| {
                let apply_btn = ui.add(
                    eframe::egui::Button::new(
                        RichText::new("Apply").size(12.0),
                    )
                    .fill(theme.accent)
                    .corner_radius(eframe::egui::CornerRadius::same(4)),
                );
                if apply_btn.clicked() {
                    if let Some(preset) = presets.get(app.preset_index) {
                        app.apply_preset(preset);
                    }
                }

                ComboBox::from_id_salt("preset")
                    .selected_text(
                        RichText::new(
                            preset_names.get(app.preset_index).cloned().unwrap_or_default(),
                        )
                        .size(13.0),
                    )
                    .width(120.0)
                    .show_ui(ui, |ui| {
                        for (i, name) in preset_names.iter().enumerate() {
                            ui.selectable_value(
                                &mut app.preset_index,
                                i,
                                RichText::new(name.as_str()).size(13.0),
                            );
                        }
                    });
            });
        });
    });

    ui.add_space(4.0);

    // ── Style ──
    card(ui, theme, |ui| {
        dropdown(
            ui,
            theme,
            "keycap_style",
            "Style",
            KEYCAP_STYLES,
            &mut app.keycap_style_index,
        );
    });

    ui.add_space(4.0);

    // ── Colors ──
    card(ui, theme, |ui| {
        section_header(ui, theme, "Colors");

        color_row(ui, theme, "Primary", &mut app.config.keycap_primary.clone().unwrap_or_default());
        color_row(ui, theme, "Secondary", &mut app.config.keycap_secondary.clone().unwrap_or_default());

        let mut use_gradient = app.config.use_gradient.unwrap_or(true);
        toggle_row(ui, theme, "Gradient", &mut use_gradient);
        app.config.use_gradient = Some(use_gradient);

        ui.add_space(4.0);

        let mut highlight_mods = app.config.highlight_modifiers.unwrap_or(true);
        toggle_row(ui, theme, "Highlight Modifiers", &mut highlight_mods);
        app.config.highlight_modifiers = Some(highlight_mods);

        if highlight_mods {
            color_row(ui, theme, "Modifier Primary", &mut app.config.modifier_primary.clone().unwrap_or_default());
            color_row(ui, theme, "Modifier Secondary", &mut app.config.modifier_secondary.clone().unwrap_or_default());
        }
    });

    ui.add_space(4.0);

    // ── Text ──
    card(ui, theme, |ui| {
        section_header(ui, theme, "Text");

        let mut text_size = app.config.text_size.unwrap_or(0.0);
        labeled_slider(ui, theme, "Font Size", &mut text_size, 0.0..=64.0, "px");
        app.config.text_size = if text_size <= 0.0 { None } else { Some(text_size) };

        color_row(ui, theme, "Color", &mut app.config.text_color.clone().unwrap_or_default());

        dropdown(
            ui,
            theme,
            "text_caps",
            "Capitalization",
            TEXT_CAPS,
            &mut app.text_caps_index,
        );

        dropdown(
            ui,
            theme,
            "text_variant",
            "Variant",
            TEXT_VARIANTS,
            &mut app.text_variant_index,
        );

        let highlight_mods = app.config.highlight_modifiers.unwrap_or(true);
        if highlight_mods {
            color_row(ui, theme, "Modifier Color", &mut app.config.text_modifier_color.clone().unwrap_or_default());
        }
    });

    ui.add_space(4.0);

    // ── Border ──
    card(ui, theme, |ui| {
        section_header(ui, theme, "Border");

        let mut border_enabled = app.config.border_enabled.unwrap_or(true);
        toggle_row(ui, theme, "Enabled", &mut border_enabled);
        app.config.border_enabled = Some(border_enabled);

        if border_enabled {
            color_row(ui, theme, "Color", &mut app.config.border_color.clone().unwrap_or_default());

            let mut border_width = app.config.border_width.unwrap_or(1.0);
            labeled_slider(ui, theme, "Width", &mut border_width, 0.5..=4.0, "px");
            app.config.border_width = Some(border_width);

            let mut border_radius = app.config.border_radius.unwrap_or(0.25);
            labeled_slider(ui, theme, "Radius", &mut border_radius, 0.0..=1.0, "%");
            app.config.border_radius = Some(border_radius);

            let highlight_mods = app.config.highlight_modifiers.unwrap_or(true);
            if highlight_mods {
                color_row(ui, theme, "Modifier Color", &mut app.config.border_modifier_color.clone().unwrap_or_default());
            }
        }
    });

    ui.add_space(4.0);

    // ── Background ──
    card(ui, theme, |ui| {
        section_header(ui, theme, "Background");

        let mut bg_enabled = app.config.background_enabled.unwrap_or(false);
        toggle_row(ui, theme, "Fill", &mut bg_enabled);
        app.config.background_enabled = Some(bg_enabled);

        if bg_enabled {
            color_row(ui, theme, "Color", &mut app.config.background_color.clone().unwrap_or_default());
        }
    });

    save_bar(ui, theme, ctx, app);
}