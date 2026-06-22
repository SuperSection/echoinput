# EchoInput New Architecture

## Current Problems

1. **Multiple entry points**: `app`, `wayland-app`, `tauri-app` - confusing and redundant
2. **Platform-specific code in main**: `app/src/main.rs` has massive `#[cfg]` blocks for platform-specific component creation
3. **Traits in wrong crate**: `OverlayRenderer`, `KeyboardCaptureProvider`, `EventProcessor` traits are in `input-core` but should be in a platform abstraction layer
4. **Settings GUI mixed with main app**: Large settings GUI code in `app/src/main.rs`
5. **Factory pattern not fully utilized**: Factory traits exist but aren't properly implemented/used

## New Crate Structure

```
echoinput/
├── Cargo.toml (workspace)
├── core/                    # Core types, events, config, processor, IPC
│   ├── Cargo.toml
│   └── src/
│       ├── config.rs
│       ├── events.rs
│       ├── ipc.rs
│       ├── keys.rs
│       ├── lib.rs
│       ├── overlay.rs       # OverlayConfig, DisplayEvent (data types only)
│       ├── presets.rs
│       ├── processor.rs
│       └── traits.rs        # EventProcessor trait only
├── platform/                # Platform abstraction layer (NEW)
│   ├── Cargo.toml
│   └── src/
│       ├── capture.rs       # KeyboardCaptureProvider, CaptureFeatures
│       ├── overlay.rs       # OverlayRenderer, OverlayConfig, DisplayEvent
│       ├── processor.rs     # EventProcessor trait (re-export from core)
│       ├── factory.rs       # OverlayRendererFactory, KeyboardCaptureFactory
│       └── lib.rs
├── platform-linux/          # Linux implementations
│   ├── Cargo.toml
│   └── src/
│       ├── capture.rs       # EvdevCapture
│       ├── keymap.rs
│       ├── overlay_wayland.rs  # WaylandRenderer + WaylandFactory
│       ├── overlay_x11.rs   # X11Renderer + X11Factory
│       └── lib.rs
├── platform-windows/        # Windows implementations
│   ├── Cargo.toml
│   └── src/
│       ├── capture.rs       # WindowsCapture + WindowsCaptureFactory
│       ├── overlay.rs       # WindowsRenderer + WindowsRendererFactory
│       └── lib.rs
├── platform-macos/          # macOS implementations
│   ├── Cargo.toml
│   └── src/
│       ├── capture.rs       # MacosCapture + MacosCaptureFactory
│       ├── overlay.rs       # MacRenderer + MacRendererFactory
│       └── lib.rs
├── settings/                # Settings GUI (NEW)
│   ├── Cargo.toml
│   └── src/
│       ├── app.rs           # SettingsApp
│       ├── theme.rs
│       ├── tabs/
│       │   ├── general.rs
│       │   ├── position.rs
│       │   ├── keycap.rs
│       │   ├── display.rs
│       │   └── about.rs
│       └── lib.rs
├── echoinput/               # Single binary entry point (NEW)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs          # Platform detection, factory selection
│       ├── platform.rs      # Platform detection logic
│       ├── overlay.rs       # Overlay runner
│       └── cli.rs           # CLI parsing
└── overlay/                 # Overlay state manager (keep for now, may merge)
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

## Dependency Graph

```
echoinput (binary)
  ├── core
  ├── platform
  ├── settings
  ├── platform-linux (cfg(target_os = "linux"))
  ├── platform-windows (cfg(target_os = "windows"))
  └── platform-macos (cfg(target_os = "macos"))

platform
  └── core (for EventProcessor trait, types)

platform-linux
  ├── core
  ├── platform
  ├── overlay-wayland (internal)
  └── overlay-x11 (internal)

platform-windows
  ├── core
  ├── platform
  └── overlay-windows (internal)

platform-macos
  ├── core
  ├── platform
  └── overlay-macos (internal)

settings
  ├── core
  └── platform (for OverlayConfig)

core
  └── (no internal deps)
```

## Key Changes

### 1. Platform Abstraction Crate (`platform`)
- Moves `KeyboardCaptureProvider`, `OverlayRenderer`, `OverlayConfig`, `DisplayEvent` from `input-core`
- Adds factory traits: `OverlayRendererFactory`, `KeyboardCaptureFactory`
- `input-core` depends on `platform` for traits, re-exports `EventProcessor`

### 2. Settings Crate (`settings`)
- Extracts all egui settings GUI code
- Provides `SettingsApp` that can be embedded
- Depends on `core` for `FileConfig` and `platform` for `OverlayConfig`

### 3. Single Binary (`echoinput`)
- Replaces `app`, `wayland-app`, `tauri-app`
- Uses factory pattern for platform detection:
  ```rust
  let factory = detect_platform_factory();
  let renderer = factory.create_renderer(bus);
  let capture = factory.create_capture();
  ```
- No `#[cfg]` blocks in main.rs - all platform logic in platform crates

### 4. Platform Implementation Crates
- Each provides factory implementations
- `platform-linux` provides both Wayland and X11 factories
- Platform detection at runtime (not compile time)

## Migration Plan

1. Create `platform` crate with traits and factories
2. Update `input-core` to depend on `platform`, remove trait definitions
3. Create `settings` crate with extracted GUI code
4. Create `echoinput` binary crate with platform detection
5. Update `platform-linux`, `platform-windows`, `platform-macos` to implement factories
6. Remove old `app`, `wayland-app`, `tauri-app` crates
7. Update workspace Cargo.toml

## Platform Detection Logic

```rust
fn detect_platform() -> Platform {
    #[cfg(target_os = "linux")]
    {
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            Platform::LinuxWayland
        } else {
            Platform::LinuxX11
        }
    }
    #[cfg(target_os = "windows")]
    Platform::Windows,
    #[cfg(target_os = "macos")]
    Platform::MacOS,
}
```

The binary crate is the ONLY place with `#[cfg]` blocks - for selecting which factory to use at compile time. The actual platform-specific code lives in the platform crates.