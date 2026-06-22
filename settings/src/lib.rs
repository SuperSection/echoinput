//! Settings GUI for EchoInput.

pub mod app;
pub mod tabs;
pub mod theme;

pub use app::SettingsApp;
pub use theme::Theme;

/// Run the settings GUI with the given initial configuration.
pub fn run_settings_gui(initial_config: input_core::config::FileConfig) {
    input_core::config::FileConfig::load(); // Ensure config directory exists

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([520.0, 480.0])
            .with_min_inner_size([420.0, 380.0])
            .with_title("EchoInput Settings"),
        ..Default::default()
    };

    eframe::run_native(
        "EchoInput Settings",
        options,
        Box::new(move |cc| {
            let theme = Theme::dark();
            theme::apply_theme(&cc.egui_ctx, &theme);
            Ok(Box::new(SettingsApp::new(initial_config, theme)))
        }),
    )
    .unwrap();
}