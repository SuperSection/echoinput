//! EchoInput — Single binary entry point for all platforms.

use input_core::config::FileConfig;
use input_core::events::{ModifierState, ProcessedEvent, ShortcutCombo};
use input_core::ipc::MessageBus;
use input_core::overlay::{DisplayEvent, OverlayConfig};
use input_core::processor::DefaultEventProcessor;
use input_core::traits::{EventProcessor, ProcessorConfig};
use platform::capture::{KeyboardCaptureProvider, KeyboardCaptureFactory};
use platform::overlay::{OverlayRenderer, OverlayRendererFactory};
use settings::run_settings_gui;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast::error::RecvError;
use tracing::{error, info, warn};

// ── CLI & Main ──────────────────────────────────────────────────

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
    println!("EchoInput — keyboard visualization overlay");
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
    println!("PLATFORM-SPECIFIC NOTES:");
    #[cfg(target_os = "linux")]
    {
        println!("  Linux: Requires read access to /dev/input/event* devices");
        println!("  Fix permissions: sudo usermod -aG input $USER  (then relogin)");
        println!("  Auto-detects Wayland (WAYLAND_DISPLAY) or X11");
    }
    #[cfg(target_os = "windows")]
    {
        println!("  Windows: Global keyboard hook requires the app to be running");
        println!("  The overlay will appear on top of all windows");
    }
    #[cfg(target_os = "macos")]
    {
        println!("  macOS: Requires Accessibility permissions");
        println!("  System Preferences > Privacy & Security > Accessibility > EchoInput");
    }
    println!();
    println!("Config saved to: ~/.config/echoinput/config.toml");
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

// ── Overlay mode ──────────────────────────────────────────────────

fn run_overlay(config: OverlayConfig) {
    info!("Starting EchoInput overlay");
    eprintln!("EchoInput overlay running. Press keys to see visualization.");
    eprintln!("Press Ctrl+C to quit.");

    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    let bus = MessageBus::new(4096);
    let shutdown = Arc::new(AtomicBool::new(false));

    rt.block_on(async {
        // Create platform-specific renderer and capture
        let (mut renderer, mut capture) = create_platform_components(bus.clone(), shutdown.clone());

        if let Err(e) = renderer.start(config.clone()).await {
            error!("Failed to start overlay: {}", e);
            eprintln!("Error: Failed to start overlay: {}", e);
            return;
        }

        if let Err(e) = capture.start().await {
            error!("Failed to start keyboard capture: {}", e);
            eprintln!("Error: Failed to start keyboard capture: {}", e);
            #[cfg(target_os = "linux")]
            {
                eprintln!("Hint: No keyboard devices found. Check /dev/input/event* permissions.");
                eprintln!("      Try: sudo usermod -aG input $USER  (then relogin)");
            }
            #[cfg(target_os = "macos")]
            {
                eprintln!("Hint: Check System Preferences > Privacy & Security > Accessibility");
            }
            return;
        }

        let mut input_rx = capture.subscribe();
        let mut processor = DefaultEventProcessor::new(ProcessorConfig {
            group_shortcuts: true,
            history_length: config.history_length,
            dedup_window: Duration::from_millis(50),
        });

        let ctrl_c = tokio::signal::ctrl_c();
        tokio::pin!(ctrl_c);

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

            if shutdown.load(Ordering::Relaxed) {
                eprintln!("\nShutting down...");
                break;
            }
        }

        let _ = capture.stop().await;
        let _ = renderer.stop().await;
    });
}

// ── Platform-specific component creation ────────────────────────

#[cfg(target_os = "linux")]
fn create_platform_components(
    bus: MessageBus,
    shutdown: Arc<AtomicBool>,
) -> (
    Box<dyn OverlayRenderer>,
    Box<dyn KeyboardCaptureProvider>,
) {
    use platform_linux::{EvdevCaptureFactory, LinuxWaylandRendererFactory, LinuxX11RendererFactory};

    let is_wayland = std::env::var("WAYLAND_DISPLAY").is_ok();

    let renderer: Box<dyn OverlayRenderer> = if is_wayland {
        eprintln!("Detected Wayland display server");
        let factory = LinuxWaylandRendererFactory::new();
        factory.create(bus)
    } else {
        eprintln!("Detected X11 display server");
        let factory = LinuxX11RendererFactory::new();
        factory.create(bus)
    };

    let capture: Box<dyn KeyboardCaptureProvider> =
        EvdevCaptureFactory::new().create();

    (renderer, capture)
}

#[cfg(target_os = "windows")]
fn create_platform_components(
    bus: MessageBus,
    shutdown: Arc<AtomicBool>,
) -> (
    Box<dyn OverlayRenderer>,
    Box<dyn KeyboardCaptureProvider>,
) {
    use platform_windows::{WindowsCaptureFactory, WindowsRendererFactory};

    let renderer: Box<dyn OverlayRenderer> =
        WindowsRendererFactory::new().create(bus);
    let capture: Box<dyn KeyboardCaptureProvider> =
        WindowsCaptureFactory::new().create();

    (renderer, capture)
}

#[cfg(target_os = "macos")]
fn create_platform_components(
    bus: MessageBus,
    shutdown: Arc<AtomicBool>,
) -> (
    Box<dyn OverlayRenderer>,
    Box<dyn KeyboardCaptureProvider>,
) {
    use platform_macos::{MacosCaptureFactory, MacRendererFactory};

    let renderer: Box<dyn OverlayRenderer> =
        MacRendererFactory::new().create(bus);
    let capture: Box<dyn KeyboardCaptureProvider> =
        MacosCaptureFactory::new().create();

    (renderer, capture)
}