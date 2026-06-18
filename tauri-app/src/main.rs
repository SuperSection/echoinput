#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use input_core::ipc::{MessageBus, OverlayCommand, SettingsUpdate};
use input_core::overlay::OverlayConfig;
use input_core::processor::DefaultEventProcessor;
use input_core::traits::{
    EventProcessor, KeyboardCaptureProvider, OverlayRendererFactory, ProcessorConfig,
};
use overlay::OverlayManager;
use tokio::sync::broadcast;
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

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_new(parse_log_level())
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    info!("EchoInput starting");

    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

    rt.block_on(async {
        if let Err(e) = run().await {
            error!(error = %e, "Fatal error");
            std::process::exit(1);
        }
    });
}

async fn run() -> anyhow::Result<()> {
    // ── 1. Create the message bus ──────────────────────────────
    let bus = MessageBus::new(1024);

    // ── 2. Create platform-specific capture provider ───────────
    #[cfg(target_os = "linux")]
    let mut capture = {
        info!(platform = "linux", capture = "evdev", "Using input capture");
        platform_linux::evdev_capture::EvdevCapture::with_sender(bus.input_sender())
    };

    #[cfg(target_os = "windows")]
    let mut capture = {
        info!(platform = "windows", capture = "hooks", "Using input capture");
        platform_windows::WindowsCapture::with_sender(bus.input_sender())
    };

    #[cfg(target_os = "macos")]
    let mut capture = {
        info!(platform = "macos", capture = "cgtap", "Using input capture");
        platform_macos::MacosCapture::with_sender(bus.input_sender())
    };

    // ── 3. Create event processor ──────────────────────────────
    let mut processor = DefaultEventProcessor::new(ProcessorConfig {
        group_shortcuts: true,
        history_length: 10,
        ..Default::default()
    });

    // ── 4. Create and start overlay manager ────────────────────
    let mut overlay = OverlayManager::new(OverlayConfig::default());
    overlay.run(bus.clone());

    // ── 5. Create and start Wayland renderer ───────────────────
    #[cfg(target_os = "linux")]
    let mut renderer = {
        let factory = overlay_wayland::WaylandRendererFactory::new();
        factory.create(bus.clone())
    };

    #[cfg(not(target_os = "linux"))]
    let mut renderer = {
        Box::new(overlay::MockRenderer::with_bus(bus.clone()))
    };

    renderer.start(OverlayConfig::default()).await?;
    info!(renderer = renderer.name(), "Renderer started");

    // ── 6. Subscribe to bus channels ───────────────────────────
    let mut input_rx = bus.subscribe_input();

    // ── 7. Start capture ───────────────────────────────────────
    capture.start().await?;

    // ── 8. Spawn task: input → processor → shortcut bus ────────
    let bus_clone = bus.clone();
    tokio::spawn(async move {
        loop {
            match input_rx.recv().await {
                Ok(event) => {
                    let processed = processor.process(event);
                    for pe in processed {
                        match pe {
                            input_core::events::ProcessedEvent::Shortcut(combo) => {
                                let shortcut_event =
                                    input_core::ipc::ShortcutEvent::new(combo.clone());
                                bus_clone.publish_shortcut(shortcut_event);
                            }
                            input_core::events::ProcessedEvent::ModifierChange(_mods) => {}
                            input_core::events::ProcessedEvent::RawKey(_kbd) => {}
                        }
                    }
                }
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    warn!(missed = n, "Input events lagged");
                }
                Err(broadcast::error::RecvError::Closed) => {
                    info!("Input channel closed");
                    break;
                }
            }
        }
    });

    // ── 9. Demonstrate settings updates (simulated Tauri) ─────
    let bus_demo = bus.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        bus_demo.publish_settings(SettingsUpdate::Theme(
            input_core::overlay::Theme::Light,
        ));

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        bus_demo.publish_command(OverlayCommand::Restart);

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        bus_demo.publish_settings(SettingsUpdate::Batch(vec![
            SettingsUpdate::Opacity(0.6),
            SettingsUpdate::Position(input_core::overlay::OverlayPosition::TopCenter),
            SettingsUpdate::Theme(input_core::overlay::Theme::Dark),
        ]));
    });

    info!("Ready — press Ctrl+C to exit");

    // ── 10. Wait for shutdown ─────────────────────────────────
    tokio::signal::ctrl_c().await?;
    info!("Shutting down");

    renderer.stop().await?;
    bus.publish_command(OverlayCommand::Stop);
    capture.stop().await?;

    info!("EchoInput stopped");
    Ok(())
}
