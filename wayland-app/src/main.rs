use eframe::egui;
use input_core::config::FileConfig;
use input_core::events::{ModifierState, ProcessedEvent, ShortcutCombo};
use input_core::ipc::MessageBus;
use input_core::overlay::{DisplayEvent, OverlayConfig};
use input_core::presets::ThemePreset;
use input_core::processor::DefaultEventProcessor;
use input_core::traits::{EventProcessor, KeyboardCaptureProvider, OverlayRenderer, ProcessorConfig};
use overlay_wayland::WaylandRenderer;
use platform_linux::evdev_capture::EvdevCapture;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast::error::RecvError;
use tracing::{error, info, warn};

fn parse_log_level() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--trace") {
        return "trace".into();
    }
    if args.iter().any(|a| a == "--debug") {
        return "debug".into();
    }
    std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into())
}

fn print_help() {
    println!("EchoInput — keyboard visualization overlay for Wayland");
    println!();
    println!("USAGE:");
    println!("  echoinput                 Run the overlay (default)");
    println!("  echoinput --settings      Open settings GUI");
    println!("  echoinput --help          Show this help");
    println!();
    println!("OPTIONS:");
    println!("  --debug     Enable debug logging");
    println!("  --trace     Enable trace logging (very verbose)");
    println!();
    println!("NOTES:");
    println!("  - Requires read access to /dev/input/event* devices");
    println!("  - If overlay doesn't appear, check: ls -la /dev/input/event*");
    println!("  - Fix permissions: sudo usermod -aG input $USER  (then relogin)");
    println!("  - Config saved to: ~/.config/echoinput/config.toml");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_new(parse_log_level())
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let settings_mode = args.iter().any(|a| a == "--settings");

    let file_config = FileConfig::load();
    let overlay_config = file_config.to_overlay_config();

    if settings_mode {
        run_settings_gui(file_config);
    } else {
        run_overlay(overlay_config);
    }
}

// ── Overlay mode (default) ──────────────────────────────────────

fn run_overlay(config: OverlayConfig) {
    info!("Starting EchoInput overlay");

    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

    let bus = MessageBus::new(4096);
    let shutdown = Arc::new(AtomicBool::new(false));

    rt.block_on(async {
        let mut renderer = WaylandRenderer::with_shutdown(bus.clone(), shutdown.clone());

        if let Err(e) = renderer.start(config.clone()).await {
            error!("Failed to start overlay: {}", e);
            eprintln!("Error: Failed to start overlay: {}", e);
            return;
        }

        // Start evdev keyboard capture
        let mut capture = EvdevCapture::with_shutdown(shutdown.clone());

        let mut input_rx = capture.subscribe();

        if let Err(e) = capture.start().await {
            error!("Failed to start evdev capture: {}", e);
            eprintln!("Error: Failed to start keyboard capture: {}", e);
            eprintln!("Hint: No keyboard devices found. Check /dev/input/event* permissions.");
            eprintln!("      Try: sudo usermod -aG input $USER  (then relogin)");
            return;
        }

        eprintln!("EchoInput overlay running. Press keys to see visualization.");
        eprintln!("Press Ctrl+C to quit.");

        let mut processor = DefaultEventProcessor::new(ProcessorConfig {
            group_shortcuts: true,
            history_length: config.history_length,
            dedup_window: Duration::from_millis(50),
        });

        let ctrl_c = tokio::signal::ctrl_c();
        tokio::pin!(ctrl_c);

        // Process keyboard events from evdev and forward to overlay
        loop {
            tokio::select! {
                result = input_rx.recv() => {
                    match result {
                        Ok(event) => {
                            let processed = processor.process(event);
                            for pe in processed {
                                match pe {
                                    ProcessedEvent::Shortcut(combo) => {
                                        if let Err(e) = renderer.update(DisplayEvent::Shortcut(combo)) {
                                            warn!("Failed to send shortcut to renderer: {}", e);
                                        }
                                    }
                                    ProcessedEvent::RawKey(kbd) => {
                                        let combo = ShortcutCombo::new(
                                            ModifierState::default(),
                                            Some(kbd.key),
                                        );
                                        if let Err(e) = renderer.update(DisplayEvent::Shortcut(combo)) {
                                            warn!("Failed to send key to renderer: {}", e);
                                        }
                                    }
                                    ProcessedEvent::ModifierChange(_) => {}
                                }
                            }
                        }
                        Err(RecvError::Lagged(n)) => {
                            warn!("Input channel lagged, dropped {} events", n);
                        }
                        Err(RecvError::Closed) => {
                            error!("Input channel closed — capture thread may have exited");
                            eprintln!("Error: Input capture channel closed.");
                            break;
                        }
                    }
                }
                _ = &mut ctrl_c => {
                    eprintln!("\nShutting down...");
                    shutdown.store(true, Ordering::Relaxed);
                    break;
                }
            }

            // Also check shutdown flag (set by evdev Ctrl+C detection in terminal)
            if shutdown.load(Ordering::Relaxed) {
                eprintln!("\nShutting down...");
                break;
            }
        }

        let _ = capture.stop().await;
        let _ = renderer.stop().await;
    });
}

// ── Settings GUI mode ──────────────────────────────────────────

fn run_settings_gui(initial_config: FileConfig) {
    info!("Starting EchoInput settings GUI");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 550.0])
            .with_min_inner_size([600.0, 450.0])
            .with_title("EchoInput Settings"),
        ..Default::default()
    };

    eframe::run_native(
        "EchoInput Settings",
        options,
        Box::new(move |_cc| Ok(Box::new(SettingsApp::new(initial_config)))),
    )
    .unwrap();
}

#[derive(PartialEq, Clone, Copy)]
enum SettingsTab {
    General,
    Position,
    Keycap,
    Display,
    About,
}

struct SettingsApp {
    config: FileConfig,
    active_tab: SettingsTab,
    position_index: usize,
    scale_index: usize,
    theme_index: usize,
    keycap_style_index: usize,
    animation_type_index: usize,
    text_caps_index: usize,
    text_variant_index: usize,
    preset_index: usize,
    save_status: String,
}

const POSITIONS: &[&str] = &[
    "BottomCenter",
    "TopLeft",
    "TopRight",
    "TopCenter",
    "BottomLeft",
    "BottomRight",
    "Center",
];

const SCALES: &[&str] = &["Small", "Medium", "Large", "ExtraLarge"];
const THEMES: &[&str] = &["Dark", "Light", "System"];
const KEYCAP_STYLES: &[&str] = &["Minimal", "Laptop", "LowProfile", "PBT"];
const ANIMATION_TYPES: &[&str] = &["None", "Fade", "Zoom", "Float", "Slide"];
const TEXT_CAPS: &[&str] = &["Uppercase", "Capitalize", "Lowercase"];
const TEXT_VARIANTS: &[&str] = &["Full", "Short", "Icon"];

impl SettingsApp {
    fn new(config: FileConfig) -> Self {
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

        let _preset_names = ThemePreset::name_list();
        let preset_index = 0;

        Self {
            config,
            active_tab: SettingsTab::General,
            position_index,
            scale_index,
            theme_index,
            keycap_style_index,
            animation_type_index,
            text_caps_index,
            text_variant_index,
            preset_index,
            save_status: String::new(),
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

    fn apply_preset(&mut self, preset: &ThemePreset) {
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

        // Update UI indices
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
                self.save_status = "Settings saved!".into();
            }
            Err(e) => {
                self.save_status = format!("Error: {}", e);
            }
        }
    }

    fn sidebar_button(ui: &mut egui::Ui, label: &str, tab: SettingsTab, active: &mut SettingsTab) {
        let is_active = *active == tab;
        let response = ui.selectable_label(is_active, label);
        if response.clicked() {
            *active = tab;
        }
    }

    fn color_edit(ui: &mut egui::Ui, label: &str, value: &mut String) {
        ui.horizontal(|ui| {
            ui.label(label);
            let mut color = value.clone();
            let response = ui.text_edit_singleline(&mut color);
            if response.changed() {
                *value = color;
            }
            // Show color preview
            if let Some(hex) = value.strip_prefix('#') {
                if hex.len() >= 6 {
                    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
                    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
                    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 2.0, egui::Color32::from_rgb(
                        (r * 255.0) as u8,
                        (g * 255.0) as u8,
                        (b * 255.0) as u8,
                    ));
                }
            }
        });
    }
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            ui.heading("EchoInput");
            ui.add_space(8.0);
            Self::sidebar_button(ui, "General", SettingsTab::General, &mut self.active_tab);
            Self::sidebar_button(ui, "Position", SettingsTab::Position, &mut self.active_tab);
            Self::sidebar_button(ui, "Keycap Style", SettingsTab::Keycap, &mut self.active_tab);
            Self::sidebar_button(ui, "Display", SettingsTab::Display, &mut self.active_tab);
            Self::sidebar_button(ui, "About", SettingsTab::About, &mut self.active_tab);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_tab {
                SettingsTab::General => self.render_general_tab(ui, ctx),
                SettingsTab::Position => self.render_position_tab(ui, ctx),
                SettingsTab::Keycap => self.render_keycap_tab(ui, ctx),
                SettingsTab::Display => self.render_display_tab(ui, ctx),
                SettingsTab::About => self.render_about_tab(ui),
            }
        });
    }
}

impl SettingsApp {
    fn render_general_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("General Settings");
        ui.separator();

        ui.add_space(8.0);
        ui.label("Theme:");
        egui::ComboBox::from_id_salt("theme")
            .selected_text(THEMES[self.theme_index])
            .show_ui(ui, |ui| {
                for (i, &theme) in THEMES.iter().enumerate() {
                    ui.selectable_value(&mut self.theme_index, i, theme);
                }
            });

        ui.add_space(8.0);

        ui.label("Monitor (leave empty for default):");
        let mut monitor = self.config.monitor.clone().unwrap_or_default();
        ui.text_edit_singleline(&mut monitor);
        self.config.monitor = if monitor.is_empty() {
            None
        } else {
            Some(monitor)
        };

        ui.add_space(16.0);
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                self.save();
            }
            if ui.button("Save & Close").clicked() {
                self.sync_to_config();
                if self.config.save().is_ok() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        });

        if !self.save_status.is_empty() {
            ui.label(&self.save_status);
        }
    }

    fn render_position_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Position & Animation");
        ui.separator();

        ui.add_space(8.0);
        ui.label("Overlay Position:");
        egui::ComboBox::from_id_salt("position")
            .selected_text(POSITIONS[self.position_index])
            .show_ui(ui, |ui| {
                for (i, &pos) in POSITIONS.iter().enumerate() {
                    ui.selectable_value(&mut self.position_index, i, pos);
                }
            });

        ui.add_space(8.0);

        ui.label("Scale:");
        egui::ComboBox::from_id_salt("scale")
            .selected_text(SCALES[self.scale_index])
            .show_ui(ui, |ui| {
                for (i, &scale) in SCALES.iter().enumerate() {
                    ui.selectable_value(&mut self.scale_index, i, scale);
                }
            });

        ui.add_space(8.0);

        let mut margin_x = self.config.margin_x.unwrap_or(16.0);
        ui.label(format!("Horizontal Margin: {:.0}px", margin_x));
        ui.add(egui::Slider::new(&mut margin_x, 0.0..=100.0).suffix("px"));
        self.config.margin_x = Some(margin_x);

        let mut margin_y = self.config.margin_y.unwrap_or(16.0);
        ui.label(format!("Vertical Margin: {:.0}px", margin_y));
        ui.add(egui::Slider::new(&mut margin_y, 0.0..=100.0).suffix("px"));
        self.config.margin_y = Some(margin_y);

        ui.add_space(16.0);
        ui.label("Animation:");
        egui::ComboBox::from_id_salt("animation_type")
            .selected_text(ANIMATION_TYPES[self.animation_type_index])
            .show_ui(ui, |ui| {
                for (i, &anim) in ANIMATION_TYPES.iter().enumerate() {
                    ui.selectable_value(&mut self.animation_type_index, i, anim);
                }
            });

        ui.add_space(8.0);

        let mut anim_speed = self.config.animation_speed.unwrap_or(0.5);
        ui.label(format!("Animation Speed: {:.2}", anim_speed));
        ui.add(egui::Slider::new(&mut anim_speed, 0.05..=1.0));
        self.config.animation_speed = Some(anim_speed);

        ui.add_space(8.0);

        let mut duration_ms = self.config.display_duration_ms.unwrap_or(1500) as f32;
        ui.label(format!("Display Duration: {}ms", duration_ms as u64));
        ui.add(egui::Slider::new(&mut duration_ms, 500.0..=5000.0).suffix("ms"));
        self.config.display_duration_ms = Some(duration_ms as u64);

        ui.add_space(16.0);
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                self.save();
            }
            if ui.button("Save & Close").clicked() {
                self.sync_to_config();
                if self.config.save().is_ok() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        });
    }

    fn render_keycap_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Keycap Style");
        ui.separator();

        // ── Preset Selector ──
        ui.add_space(4.0);
        ui.collapsing("Theme Presets", |ui| {
            let presets = ThemePreset::all();
            let preset_names: Vec<String> = presets.iter().map(|p| p.name.clone()).collect();

            ui.horizontal(|ui| {
                egui::ComboBox::from_id_salt("preset")
                    .selected_text(preset_names.get(self.preset_index).cloned().unwrap_or_default())
                    .show_ui(ui, |ui| {
                        for (i, name) in preset_names.iter().enumerate() {
                            ui.selectable_value(&mut self.preset_index, i, name.as_str());
                        }
                    });

                if ui.button("Apply Preset").clicked() {
                    if let Some(preset) = presets.get(self.preset_index) {
                        self.apply_preset(preset);
                    }
                }
            });
        });

        ui.add_space(8.0);

        // ── Keycap Style ──
        ui.collapsing("Keycap Style", |ui| {
            ui.label("Style:");
            egui::ComboBox::from_id_salt("keycap_style")
                .selected_text(KEYCAP_STYLES[self.keycap_style_index])
                .show_ui(ui, |ui| {
                    for (i, &style) in KEYCAP_STYLES.iter().enumerate() {
                        ui.selectable_value(&mut self.keycap_style_index, i, style);
                    }
                });
        });

        ui.add_space(4.0);

        // ── Colors ──
        ui.collapsing("Colors", |ui| {
            Self::color_edit(ui, "Keycap Primary:", &mut self.config.keycap_primary.clone().unwrap_or_default());
            Self::color_edit(ui, "Keycap Secondary:", &mut self.config.keycap_secondary.clone().unwrap_or_default());

            let mut use_gradient = self.config.use_gradient.unwrap_or(true);
            ui.checkbox(&mut use_gradient, "Use Gradient");
            self.config.use_gradient = Some(use_gradient);

            ui.add_space(4.0);

            let mut highlight_mods = self.config.highlight_modifiers.unwrap_or(true);
            ui.checkbox(&mut highlight_mods, "Highlight Modifiers");
            self.config.highlight_modifiers = Some(highlight_mods);

            if highlight_mods {
                Self::color_edit(ui, "Modifier Primary:", &mut self.config.modifier_primary.clone().unwrap_or_default());
                Self::color_edit(ui, "Modifier Secondary:", &mut self.config.modifier_secondary.clone().unwrap_or_default());
            }
        });

        ui.add_space(4.0);

        // ── Text ──
        ui.collapsing("Text", |ui| {
            ui.label("Font Size (leave empty for scale default):");
            let mut text_size = self.config.text_size.unwrap_or(0.0);
            ui.add(egui::Slider::new(&mut text_size, 0.0..=64.0).prefix("px "));
            self.config.text_size = if text_size <= 0.0 { None } else { Some(text_size) };

            Self::color_edit(ui, "Text Color:", &mut self.config.text_color.clone().unwrap_or_default());

            ui.label("Capitalization:");
            egui::ComboBox::from_id_salt("text_caps")
                .selected_text(TEXT_CAPS[self.text_caps_index])
                .show_ui(ui, |ui| {
                    for (i, &cap) in TEXT_CAPS.iter().enumerate() {
                        ui.selectable_value(&mut self.text_caps_index, i, cap);
                    }
                });

            ui.label("Variant:");
            egui::ComboBox::from_id_salt("text_variant")
                .selected_text(TEXT_VARIANTS[self.text_variant_index])
                .show_ui(ui, |ui| {
                    for (i, &variant) in TEXT_VARIANTS.iter().enumerate() {
                        ui.selectable_value(&mut self.text_variant_index, i, variant);
                    }
                });

            let highlight_mods = self.config.highlight_modifiers.unwrap_or(true);
            if highlight_mods {
                Self::color_edit(ui, "Modifier Text Color:", &mut self.config.text_modifier_color.clone().unwrap_or_default());
            }
        });

        ui.add_space(4.0);

        // ── Border ──
        ui.collapsing("Border", |ui| {
            let mut border_enabled = self.config.border_enabled.unwrap_or(true);
            ui.checkbox(&mut border_enabled, "Enable Border");
            self.config.border_enabled = Some(border_enabled);

            if border_enabled {
                Self::color_edit(ui, "Border Color:", &mut self.config.border_color.clone().unwrap_or_default());

                let mut border_width = self.config.border_width.unwrap_or(1.0);
                ui.label(format!("Width: {:.1}px", border_width));
                ui.add(egui::Slider::new(&mut border_width, 0.5..=4.0).suffix("px"));
                self.config.border_width = Some(border_width);

                let mut border_radius = self.config.border_radius.unwrap_or(0.25);
                ui.label(format!("Radius: {:.0}%", border_radius * 100.0));
                ui.add(egui::Slider::new(&mut border_radius, 0.0..=1.0)
                    .custom_formatter(|v, _| format!("{:.0}%", v * 100.0)));
                self.config.border_radius = Some(border_radius);

                let highlight_mods = self.config.highlight_modifiers.unwrap_or(true);
                if highlight_mods {
                    Self::color_edit(ui, "Modifier Border Color:", &mut self.config.border_modifier_color.clone().unwrap_or_default());
                }
            }
        });

        ui.add_space(4.0);

        // ── Background ──
        ui.collapsing("Background", |ui| {
            let mut bg_enabled = self.config.background_enabled.unwrap_or(false);
            ui.checkbox(&mut bg_enabled, "Enable Background Fill");
            self.config.background_enabled = Some(bg_enabled);

            if bg_enabled {
                Self::color_edit(ui, "Background Color:", &mut self.config.background_color.clone().unwrap_or_default());
            }
        });

        ui.add_space(16.0);
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                self.save();
            }
            if ui.button("Save & Close").clicked() {
                self.sync_to_config();
                if self.config.save().is_ok() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        });

        if !self.save_status.is_empty() {
            ui.label(&self.save_status);
        }
    }

    fn render_display_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Display Settings");
        ui.separator();

        ui.add_space(8.0);

        let mut opacity = self.config.opacity.unwrap_or(0.9) as f64;
        ui.label(format!("Opacity: {:.0}%", opacity * 100.0));
        ui.add(egui::Slider::from_get_set(0.1..=1.0, |v| {
            if let Some(new_val) = v {
                opacity = new_val;
            }
            opacity
        })
        .suffix("%")
        .custom_formatter(|v, _| format!("{:.0}%", v * 100.0)));
        self.config.opacity = Some(opacity as f32);

        ui.add_space(8.0);

        let mut hist = self.config.history_length.unwrap_or(3) as f32;
        ui.label(format!("History Length: {}", hist as usize));
        ui.add(egui::Slider::new(&mut hist, 1.0..=10.0).step_by(1.0));
        self.config.history_length = Some(hist as usize);

        ui.add_space(16.0);
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                self.save();
            }
            if ui.button("Save & Close").clicked() {
                self.sync_to_config();
                if self.config.save().is_ok() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        });
    }

    fn render_about_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("About EchoInput");
        ui.separator();

        ui.add_space(8.0);
        ui.label("Version: 0.1.0");
        ui.label("A privacy-first keyboard visualization overlay for Wayland.");
        ui.add_space(8.0);

        ui.label("Config file location:");
        if let Some(path) = FileConfig::config_path() {
            ui.monospace(path.display().to_string());
        } else {
            ui.label("Could not determine config path");
        }

        ui.add_space(16.0);

        if ui.button("Open Config Directory").clicked() {
            if let Some(path) = FileConfig::config_path() {
                if let Some(parent) = path.parent() {
                    let _ = std::process::Command::new("xdg-open")
                        .arg(parent)
                        .spawn();
                }
            }
        }
    }
}
