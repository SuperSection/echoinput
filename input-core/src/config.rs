use crate::overlay::{
    AnimationType, BackgroundSettings, BorderSettings, ColorSettings, KeycapStyle, OverlayConfig,
    OverlayPosition, OverlayScale, TextCaps, TextSettings, TextVariant, Theme,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
use tracing::{debug, info, warn};

const CONFIG_DIR: &str = "echoinput";
const CONFIG_FILE: &str = "config.toml";
const DISPLAY_DURATION_MS_DEFAULT: u64 = 1500;

/// File-backed configuration for EchoInput.
///
/// Serializes to `~/.config/echoinput/config.toml`. All fields are
/// optional — missing fields fall back to defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    // ── General ──
    pub position: Option<String>,
    pub scale: Option<String>,
    pub opacity: Option<f32>,
    pub display_duration_ms: Option<u64>,
    pub history_length: Option<usize>,
    pub theme: Option<String>,
    pub monitor: Option<String>,
    // ── Keycap ──
    pub keycap_style: Option<String>,
    // ── Colors ──
    pub keycap_primary: Option<String>,
    pub keycap_secondary: Option<String>,
    pub use_gradient: Option<bool>,
    pub highlight_modifiers: Option<bool>,
    pub modifier_primary: Option<String>,
    pub modifier_secondary: Option<String>,
    // ── Text ──
    pub text_size: Option<f32>,
    pub text_color: Option<String>,
    pub text_modifier_color: Option<String>,
    pub text_caps: Option<String>,
    pub text_variant: Option<String>,
    // ── Border ──
    pub border_enabled: Option<bool>,
    pub border_color: Option<String>,
    pub border_width: Option<f32>,
    pub border_radius: Option<f32>,
    pub border_modifier_color: Option<String>,
    // ── Background ──
    pub background_enabled: Option<bool>,
    pub background_color: Option<String>,
    // ── Animation ──
    pub animation_type: Option<String>,
    pub animation_speed: Option<f32>,
    // ── Positioning ──
    pub margin_x: Option<f32>,
    pub margin_y: Option<f32>,
}

impl FileConfig {
    /// Resolve the config file path: `~/.config/echoinput/config.toml`.
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join(CONFIG_DIR).join(CONFIG_FILE))
    }

    /// Load config from disk. Returns defaults if the file doesn't exist
    /// or can't be parsed.
    pub fn load() -> Self {
        let path = match Self::config_path() {
            Some(p) => p,
            None => {
                warn!("Could not determine config directory, using defaults");
                return Self::defaults_toml();
            }
        };

        if !path.exists() {
            info!("No config file found at {}, creating default", path.display());
            let defaults = Self::defaults_toml();
            if let Err(e) = defaults.save() {
                warn!("Failed to write default config: {}", e);
            }
            return defaults;
        }

        match std::fs::read_to_string(&path) {
            Ok(contents) => match toml::from_str::<FileConfig>(&contents) {
                Ok(config) => {
                    debug!("Loaded config from {}", path.display());
                    config
                }
                Err(e) => {
                    warn!("Failed to parse config at {}: {}. Using defaults.", path.display(), e);
                    Self::defaults_toml()
                }
            },
            Err(e) => {
                warn!("Failed to read config at {}: {}. Using defaults.", path.display(), e);
                Self::defaults_toml()
            }
        }
    }

    /// Save this config to disk.
    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)?;
        std::fs::write(&path, contents)?;
        info!("Config saved to {}", path.display());
        Ok(())
    }

    /// Export config to a JSON string (for sharing/import).
    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Import config from a JSON string.
    pub fn from_json(json: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    /// Convert to `OverlayConfig`, filling in defaults for missing fields.
    pub fn to_overlay_config(&self) -> OverlayConfig {
        OverlayConfig {
            position: self.position.as_deref().and_then(parse_position).unwrap_or(OverlayPosition::BottomCenter),
            scale: self.scale.as_deref().and_then(parse_scale).unwrap_or(OverlayScale::Medium),
            opacity: self.opacity.unwrap_or(0.9),
            display_duration: Duration::from_millis(
                self.display_duration_ms.unwrap_or(DISPLAY_DURATION_MS_DEFAULT),
            ),
            history_length: self.history_length.unwrap_or(3),
            theme: self.theme.as_deref().and_then(parse_theme).unwrap_or(Theme::Dark),
            monitor: self.monitor.clone(),
            keycap_style: self.keycap_style.as_deref().and_then(parse_keycap_style).unwrap_or(KeycapStyle::Laptop),
            colors: ColorSettings {
                keycap_primary: self.keycap_primary.clone().unwrap_or_else(|| "#1e1e24".into()),
                keycap_secondary: self.keycap_secondary.clone().unwrap_or_else(|| "#38383f".into()),
                use_gradient: self.use_gradient.unwrap_or(true),
                highlight_modifiers: self.highlight_modifiers.unwrap_or(true),
                modifier_primary: self.modifier_primary.clone().unwrap_or_else(|| "#3358a8".into()),
                modifier_secondary: self.modifier_secondary.clone().unwrap_or_else(|| "#4d80e6".into()),
            },
            text: TextSettings {
                size: self.text_size,
                color: self.text_color.clone().unwrap_or_else(|| "#f5f5f5".into()),
                modifier_color: self.text_modifier_color.clone().unwrap_or_else(|| "#b3d4fc".into()),
                caps: self.text_caps.as_deref().and_then(parse_text_caps).unwrap_or(TextCaps::Uppercase),
                variant: self.text_variant.as_deref().and_then(parse_text_variant).unwrap_or(TextVariant::Full),
            },
            border: BorderSettings {
                enabled: self.border_enabled.unwrap_or(true),
                color: self.border_color.clone().unwrap_or_else(|| "#5a5a60".into()),
                width: self.border_width.unwrap_or(1.0),
                radius: self.border_radius.unwrap_or(0.25),
                modifier_color: self.border_modifier_color.clone().unwrap_or_else(|| "#4d80e6".into()),
            },
            background: BackgroundSettings {
                enabled: self.background_enabled.unwrap_or(false),
                color: self.background_color.clone().unwrap_or_else(|| "#00000080".into()),
            },
            animation_type: self.animation_type.as_deref().and_then(parse_animation_type).unwrap_or(AnimationType::Slide),
            animation_speed: self.animation_speed.unwrap_or(0.5),
            margin_x: self.margin_x.unwrap_or(16.0),
            margin_y: self.margin_y.unwrap_or(16.0),
        }
    }

    /// Build a FileConfig from an OverlayConfig (for saving current state).
    pub fn from_overlay_config(config: &OverlayConfig) -> Self {
        Self {
            position: Some(format!("{:?}", config.position)),
            scale: Some(format!("{:?}", config.scale)),
            opacity: Some(config.opacity),
            display_duration_ms: Some(config.display_duration.as_millis() as u64),
            history_length: Some(config.history_length),
            theme: Some(format!("{:?}", config.theme)),
            monitor: config.monitor.clone(),
            keycap_style: Some(format!("{:?}", config.keycap_style)),
            keycap_primary: Some(config.colors.keycap_primary.clone()),
            keycap_secondary: Some(config.colors.keycap_secondary.clone()),
            use_gradient: Some(config.colors.use_gradient),
            highlight_modifiers: Some(config.colors.highlight_modifiers),
            modifier_primary: Some(config.colors.modifier_primary.clone()),
            modifier_secondary: Some(config.colors.modifier_secondary.clone()),
            text_size: config.text.size,
            text_color: Some(config.text.color.clone()),
            text_modifier_color: Some(config.text.modifier_color.clone()),
            text_caps: Some(format!("{:?}", config.text.caps)),
            text_variant: Some(format!("{:?}", config.text.variant)),
            border_enabled: Some(config.border.enabled),
            border_color: Some(config.border.color.clone()),
            border_width: Some(config.border.width),
            border_radius: Some(config.border.radius),
            border_modifier_color: Some(config.border.modifier_color.clone()),
            background_enabled: Some(config.background.enabled),
            background_color: Some(config.background.color.clone()),
            animation_type: Some(format!("{:?}", config.animation_type)),
            animation_speed: Some(config.animation_speed),
            margin_x: Some(config.margin_x),
            margin_y: Some(config.margin_y),
        }
    }

    fn defaults_toml() -> Self {
        Self {
            position: Some("BottomCenter".into()),
            scale: Some("Medium".into()),
            opacity: Some(0.9),
            display_duration_ms: Some(DISPLAY_DURATION_MS_DEFAULT),
            history_length: Some(3),
            theme: Some("Dark".into()),
            monitor: None,
            keycap_style: Some("Laptop".into()),
            keycap_primary: Some("#1e1e24".into()),
            keycap_secondary: Some("#38383f".into()),
            use_gradient: Some(true),
            highlight_modifiers: Some(true),
            modifier_primary: Some("#3358a8".into()),
            modifier_secondary: Some("#4d80e6".into()),
            text_size: None,
            text_color: Some("#f5f5f5".into()),
            text_modifier_color: Some("#b3d4fc".into()),
            text_caps: Some("Uppercase".into()),
            text_variant: Some("Full".into()),
            border_enabled: Some(true),
            border_color: Some("#5a5a60".into()),
            border_width: Some(1.0),
            border_radius: Some(0.25),
            border_modifier_color: Some("#4d80e6".into()),
            background_enabled: Some(false),
            background_color: Some("#00000080".into()),
            animation_type: Some("Slide".into()),
            animation_speed: Some(0.5),
            margin_x: Some(16.0),
            margin_y: Some(16.0),
        }
    }
}

impl Default for FileConfig {
    fn default() -> Self {
        Self::defaults_toml()
    }
}

// ── Parsers ────────────────────────────────────────────────────

fn parse_position(s: &str) -> Option<OverlayPosition> {
    match s {
        "TopLeft" => Some(OverlayPosition::TopLeft),
        "TopRight" => Some(OverlayPosition::TopRight),
        "TopCenter" => Some(OverlayPosition::TopCenter),
        "BottomLeft" => Some(OverlayPosition::BottomLeft),
        "BottomRight" => Some(OverlayPosition::BottomRight),
        "BottomCenter" => Some(OverlayPosition::BottomCenter),
        "Center" => Some(OverlayPosition::Center),
        _ => None,
    }
}

fn parse_scale(s: &str) -> Option<OverlayScale> {
    match s {
        "Small" => Some(OverlayScale::Small),
        "Medium" => Some(OverlayScale::Medium),
        "Large" => Some(OverlayScale::Large),
        "ExtraLarge" => Some(OverlayScale::ExtraLarge),
        _ => None,
    }
}

fn parse_theme(s: &str) -> Option<Theme> {
    match s {
        "Dark" => Some(Theme::Dark),
        "Light" => Some(Theme::Light),
        "System" => Some(Theme::System),
        _ => None,
    }
}

fn parse_keycap_style(s: &str) -> Option<KeycapStyle> {
    match s {
        "Minimal" => Some(KeycapStyle::Minimal),
        "Laptop" => Some(KeycapStyle::Laptop),
        "LowProfile" => Some(KeycapStyle::LowProfile),
        "PBT" => Some(KeycapStyle::PBT),
        _ => None,
    }
}

fn parse_text_caps(s: &str) -> Option<TextCaps> {
    match s {
        "Uppercase" => Some(TextCaps::Uppercase),
        "Capitalize" => Some(TextCaps::Capitalize),
        "Lowercase" => Some(TextCaps::Lowercase),
        _ => None,
    }
}

fn parse_text_variant(s: &str) -> Option<TextVariant> {
    match s {
        "Full" => Some(TextVariant::Full),
        "Short" => Some(TextVariant::Short),
        "Icon" => Some(TextVariant::Icon),
        _ => None,
    }
}

fn parse_animation_type(s: &str) -> Option<AnimationType> {
    match s {
        "None" => Some(AnimationType::None),
        "Fade" => Some(AnimationType::Fade),
        "Zoom" => Some(AnimationType::Zoom),
        "Float" => Some(AnimationType::Float),
        "Slide" => Some(AnimationType::Slide),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_config_defaults() {
        let config = FileConfig::default();
        assert_eq!(config.position.as_deref(), Some("BottomCenter"));
        assert_eq!(config.scale.as_deref(), Some("Medium"));
        assert_eq!(config.opacity, Some(0.9));
        assert_eq!(config.display_duration_ms, Some(1500));
        assert_eq!(config.history_length, Some(3));
        assert_eq!(config.theme.as_deref(), Some("Dark"));
        assert!(config.monitor.is_none());
        assert_eq!(config.keycap_style.as_deref(), Some("Laptop"));
    }

    #[test]
    fn test_to_overlay_config() {
        let file_config = FileConfig::default();
        let overlay_config = file_config.to_overlay_config();
        assert_eq!(overlay_config.position, OverlayPosition::BottomCenter);
        assert_eq!(overlay_config.scale, OverlayScale::Medium);
        assert_eq!(overlay_config.opacity, 0.9);
        assert_eq!(overlay_config.display_duration, Duration::from_millis(1500));
        assert_eq!(overlay_config.history_length, 3);
        assert_eq!(overlay_config.theme, Theme::Dark);
        assert_eq!(overlay_config.keycap_style, KeycapStyle::Laptop);
        assert_eq!(overlay_config.colors.keycap_primary, "#1e1e24");
        assert_eq!(overlay_config.text.caps, TextCaps::Uppercase);
        assert_eq!(overlay_config.border.radius, 0.25);
        assert_eq!(overlay_config.animation_type, AnimationType::Slide);
    }

    #[test]
    fn test_roundtrip() {
        let original = FileConfig {
            position: Some("TopLeft".into()),
            scale: Some("Large".into()),
            opacity: Some(0.7),
            display_duration_ms: Some(2000),
            history_length: Some(5),
            theme: Some("Light".into()),
            monitor: Some("DP-1".into()),
            keycap_style: Some("PBT".into()),
            keycap_primary: Some("#ff0000".into()),
            keycap_secondary: Some("#cc0000".into()),
            use_gradient: Some(false),
            highlight_modifiers: Some(false),
            modifier_primary: Some("#00ff00".into()),
            modifier_secondary: Some("#00cc00".into()),
            text_size: Some(32.0),
            text_color: Some("#ffffff".into()),
            text_modifier_color: Some("#aaaaaa".into()),
            text_caps: Some("Lowercase".into()),
            text_variant: Some("Short".into()),
            border_enabled: Some(false),
            border_color: Some("#333333".into()),
            border_width: Some(2.0),
            border_radius: Some(0.5),
            border_modifier_color: Some("#444444".into()),
            background_enabled: Some(true),
            background_color: Some("#00000099".into()),
            animation_type: Some("Fade".into()),
            animation_speed: Some(0.8),
            margin_x: Some(24.0),
            margin_y: Some(32.0),
        };

        let overlay_config = original.to_overlay_config();
        let restored = FileConfig::from_overlay_config(&overlay_config);

        assert_eq!(restored.position.as_deref(), Some("TopLeft"));
        assert_eq!(restored.scale.as_deref(), Some("Large"));
        assert_eq!(restored.opacity, Some(0.7));
        assert_eq!(restored.display_duration_ms, Some(2000));
        assert_eq!(restored.history_length, Some(5));
        assert_eq!(restored.theme.as_deref(), Some("Light"));
        assert_eq!(restored.monitor.as_deref(), Some("DP-1"));
        assert_eq!(restored.keycap_style.as_deref(), Some("PBT"));
        assert_eq!(restored.keycap_primary.as_deref(), Some("#ff0000"));
        assert_eq!(restored.animation_type.as_deref(), Some("Fade"));
    }

    #[test]
    fn test_partial_config() {
        let toml_str = "\
position = \"TopRight\"
opacity = 0.5
keycap_style = \"Minimal\"
text_color = \"ff0000\"
";
        let config: FileConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.position.as_deref(), Some("TopRight"));
        assert_eq!(config.opacity, Some(0.5));
        assert_eq!(config.keycap_style.as_deref(), Some("Minimal"));
        assert_eq!(config.text_color.as_deref(), Some("ff0000"));
        assert!(config.scale.is_none());

        let overlay_config = config.to_overlay_config();
        assert_eq!(overlay_config.position, OverlayPosition::TopRight);
        assert_eq!(overlay_config.opacity, 0.5);
        assert_eq!(overlay_config.keycap_style, KeycapStyle::Minimal);
        assert_eq!(overlay_config.scale, OverlayScale::Medium); // default
    }

    #[test]
    fn test_json_roundtrip() {
        let original = FileConfig::default();
        let json = original.to_json().unwrap();
        let restored = FileConfig::from_json(&json).unwrap();
        assert_eq!(restored.position, original.position);
        assert_eq!(restored.keycap_primary, original.keycap_primary);
    }
}
