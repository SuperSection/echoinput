use input_core::events::ProcessedEvent;
use input_core::ipc::{MessageBus, ShortcutEvent};
use input_core::overlay::OverlayConfig;
use input_core::processor::DefaultEventProcessor;
use input_core::traits::{EventProcessor, KeyboardCaptureProvider, OverlayRenderer, ProcessorConfig};
use platform_linux::evdev_capture::EvdevCapture;
use overlay_wayland::WaylandRenderer;
use std::time::Duration;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn".into()),
        )
        .init();

    let bus = MessageBus::new(4096);
    let mut capture = EvdevCapture::with_sender(bus.input_sender());

    let mut renderer = WaylandRenderer::new(bus.clone());
    let overlay_config = OverlayConfig::default();
    if let Err(e) = renderer.start(overlay_config).await {
        error!("Failed to start overlay: {}", e);
        return;
    }

    if let Err(e) = capture.start().await {
        error!("Failed to start capture: {}", e);
        return;
    }

    info!("EchoInput running");

    let mut input_rx = capture.subscribe();
    let mut processor = DefaultEventProcessor::new(ProcessorConfig {
        group_shortcuts: true,
        history_length: 3,
        dedup_window: Duration::from_millis(50),
    });

    loop {
        match input_rx.recv().await {
            Ok(event) => {
                let processed = processor.process(event);
                for pe in processed {
                    match pe {
                        ProcessedEvent::Shortcut(combo) => {
                            let _ = bus.publish_shortcut(ShortcutEvent::new(combo));
                        }
                        ProcessedEvent::RawKey(kbd) => {
                            use input_core::events::{ModifierState, ShortcutCombo};
                            let combo = ShortcutCombo::new(
                                ModifierState::default(),
                                Some(kbd.key),
                            );
                            let _ = bus.publish_shortcut(ShortcutEvent::new(combo));
                        }
                        ProcessedEvent::ModifierChange(_) => {}
                    }
                }
            }
            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
            Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
        }
    }

    let _ = capture.stop().await;
    let _ = renderer.stop().await;
}
