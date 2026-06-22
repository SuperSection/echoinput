//! Display settings tab.

use crate::theme::Theme;
use crate::app::{card, labeled_slider_f64, labeled_slider, save_bar};
use eframe::egui::{Context, RichText, Ui};

pub fn render_display_tab(ui: &mut Ui, theme: &Theme, ctx: &Context, app: &mut crate::app::SettingsApp) {
    ui.add_space(4.0);
    card(ui, theme, |ui| {
        let mut opacity = app.config.opacity.unwrap_or(0.9) as f64;
        labeled_slider_f64(ui, theme, "Opacity", &mut opacity, 0.1..=1.0, "%");
        app.config.opacity = Some(opacity as f32);

        ui.add_space(4.0);

        let mut hist = app.config.history_length.unwrap_or(3) as f32;
        labeled_slider(ui, theme, "History Length", &mut hist, 1.0..=10.0, "");
        app.config.history_length = Some(hist as usize);
    });

    save_bar(ui, theme, ctx, app);
}