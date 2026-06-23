//! About settings tab.

use crate::app::{card, section_header};
use crate::theme::Theme;
use eframe::egui::{Context, RichText, Ui};

pub fn render_about_tab(
    ui: &mut Ui,
    theme: &Theme,
    _ctx: &Context,
    _app: &mut crate::app::SettingsApp,
) {
    ui.add_space(12.0);

    ui.centered_and_justified(|ui| {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new("EchoInput")
                    .size(24.0)
                    .color(theme.accent)
                    .strong(),
            );
            ui.add_space(4.0);
            ui.label(RichText::new("v0.1.0").size(13.0).color(theme.text_muted));
            ui.add_space(12.0);
            ui.label(
                RichText::new("A privacy-first keyboard visualization overlay")
                    .size(13.0)
                    .color(theme.text_dim),
            );
            ui.add_space(24.0);

            card(ui, theme, |ui| {
                section_header(ui, theme, "Config file");
                ui.add_space(2.0);
                if let Some(path) = input_core::config::FileConfig::config_path() {
                    ui.label(
                        RichText::new(path.display().to_string())
                            .size(12.0)
                            .color(theme.text)
                            .monospace(),
                    );
                }
            });

            ui.add_space(12.0);

            if ui
                .add(
                    eframe::egui::Button::new(RichText::new("Open Config Directory").size(13.0))
                        .fill(theme.bg_hover)
                        .corner_radius(eframe::egui::CornerRadius::same(6)),
                )
                .clicked()
            {
                if let Some(path) = input_core::config::FileConfig::config_path() {
                    if let Some(parent) = path.parent() {
                        // Cross-platform directory open
                        #[cfg(target_os = "linux")]
                        {
                            let _ = std::process::Command::new("xdg-open").arg(parent).spawn();
                        }
                        #[cfg(target_os = "macos")]
                        {
                            let _ = std::process::Command::new("open").arg(parent).spawn();
                        }
                        #[cfg(target_os = "windows")]
                        {
                            let _ = std::process::Command::new("explorer").arg(parent).spawn();
                        }
                    }
                }
            }
        });
    });
}
