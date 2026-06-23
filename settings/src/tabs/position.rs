//! Position settings tab.

use crate::app::{card, dropdown, labeled_slider, save_bar, ANIMATION_TYPES, POSITIONS, SCALES};
use crate::theme::Theme;
use eframe::egui::{Context, Ui};

pub fn render_position_tab(
    ui: &mut Ui,
    theme: &Theme,
    ctx: &Context,
    app: &mut crate::app::SettingsApp,
) {
    ui.add_space(4.0);
    card(ui, theme, |ui| {
        dropdown(
            ui,
            theme,
            "position",
            "Position",
            POSITIONS,
            &mut app.position_index,
        );
        ui.add_space(4.0);
        dropdown(ui, theme, "scale", "Scale", SCALES, &mut app.scale_index);

        ui.add_space(4.0);
        let mut margin_x = app.config.margin_x.unwrap_or(16.0);
        labeled_slider(ui, theme, "Margin X", &mut margin_x, 0.0..=100.0, "px");
        app.config.margin_x = Some(margin_x);

        let mut margin_y = app.config.margin_y.unwrap_or(16.0);
        labeled_slider(ui, theme, "Margin Y", &mut margin_y, 0.0..=100.0, "px");
        app.config.margin_y = Some(margin_y);
    });

    ui.add_space(8.0);
    card(ui, theme, |ui| {
        dropdown(
            ui,
            theme,
            "animation_type",
            "Animation",
            ANIMATION_TYPES,
            &mut app.animation_type_index,
        );

        ui.add_space(4.0);
        let mut anim_speed = app.config.animation_speed.unwrap_or(0.5);
        labeled_slider(ui, theme, "Speed", &mut anim_speed, 0.05..=1.0, "");
        app.config.animation_speed = Some(anim_speed);

        let mut duration_ms = app.config.display_duration_ms.unwrap_or(1500) as f32;
        labeled_slider(
            ui,
            theme,
            "Duration",
            &mut duration_ms,
            500.0..=5000.0,
            "ms",
        );
        app.config.display_duration_ms = Some(duration_ms as u64);
    });

    save_bar(ui, theme, ctx, app);
}
