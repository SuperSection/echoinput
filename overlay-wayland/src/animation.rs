use std::time::{Duration, Instant};
use input_core::overlay::OverlayConfig;

const DEFAULT_FADE_DURATION: Duration = Duration::from_millis(300);
const SLIDE_DURATION: Duration = Duration::from_millis(200);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Sliding,
    Visible,
    Fading,
}

pub struct Animation {
    state: AnimationState,
    shown_at: Instant,
    fade_start: Instant,
    slide_start: Instant,
    display_duration: Duration,
    fade_duration: Duration,
    slide_duration: Duration,
    current_opacity: f32,
    target_opacity: f32,
    /// Vertical offset for slide animation (0.0 = final position, positive = offset down)
    slide_offset: f32,
    /// Scale for zoom animation (0.0 = tiny, 1.0 = full size)
    scale: f32,
    dirty: bool,
}

impl Animation {
    pub fn new(config: &OverlayConfig) -> Self {
        Self {
            state: AnimationState::Idle,
            shown_at: Instant::now(),
            fade_start: Instant::now(),
            slide_start: Instant::now(),
            display_duration: config.display_duration,
            fade_duration: DEFAULT_FADE_DURATION,
            slide_duration: SLIDE_DURATION,
            current_opacity: 0.0,
            target_opacity: config.opacity,
            slide_offset: 20.0,
            scale: 0.8,
            dirty: false,
        }
    }

    pub fn show(&mut self, opacity: f32) {
        self.state = AnimationState::Sliding;
        self.shown_at = Instant::now();
        self.slide_start = Instant::now();
        self.current_opacity = opacity;
        self.target_opacity = opacity;
        self.slide_offset = 20.0;
        self.scale = 0.8;
        self.dirty = true;
    }

    /// Keep the overlay visible without restarting the slide animation.
    /// Used when appending keystrokes to an already-visible row.
    pub fn refresh(&mut self) {
        match self.state {
            AnimationState::Idle => {
                // Was idle, need a full show
                self.state = AnimationState::Sliding;
                self.slide_start = Instant::now();
                self.slide_offset = 20.0;
                self.scale = 0.8;
            }
            AnimationState::Fading => {
                // Was fading, bring back to full visible
                self.state = AnimationState::Visible;
                self.current_opacity = self.target_opacity;
            }
            AnimationState::Sliding | AnimationState::Visible => {
                // Already visible, just keep it that way
                self.state = AnimationState::Visible;
            }
        }
        self.shown_at = Instant::now();
        self.dirty = true;
    }

    pub fn update_config(&mut self, config: &OverlayConfig) {
        self.display_duration = config.display_duration;
        self.target_opacity = config.opacity;
        if self.state == AnimationState::Visible || self.state == AnimationState::Sliding {
            self.current_opacity = config.opacity;
        }
    }

    pub fn tick(&mut self) -> bool {
        let now = Instant::now();
        let mut changed = false;
        match self.state {
            AnimationState::Idle => {}
            AnimationState::Sliding => {
                let elapsed = now.duration_since(self.slide_start);
                if elapsed >= self.slide_duration {
                    self.state = AnimationState::Visible;
                    self.shown_at = now;
                    self.slide_offset = 0.0;
                    self.scale = 1.0;
                    changed = true;
                } else {
                    let t = elapsed.as_secs_f32() / self.slide_duration.as_secs_f32();
                    // Ease-out cubic
                    let eased = 1.0 - (1.0 - t).powi(3);
                    self.slide_offset = 20.0 * (1.0 - eased);
                    self.scale = 0.8 + 0.2 * eased;
                    changed = true;
                }
            }
            AnimationState::Visible => {
                if now.duration_since(self.shown_at) >= self.display_duration {
                    self.state = AnimationState::Fading;
                    self.fade_start = now;
                    changed = true;
                }
            }
            AnimationState::Fading => {
                let elapsed = now.duration_since(self.fade_start);
                if elapsed >= self.fade_duration {
                    self.current_opacity = 0.0;
                    self.state = AnimationState::Idle;
                    self.slide_offset = 0.0;
                    self.scale = 1.0;
                    changed = true;
                } else {
                    let progress = elapsed.as_secs_f32() / self.fade_duration.as_secs_f32();
                    // Ease-in quad
                    let eased = progress * progress;
                    self.current_opacity = self.target_opacity * (1.0 - eased);
                    changed = true;
                }
            }
        }
        if self.dirty {
            self.dirty = false;
            changed = true;
        }
        changed
    }

    pub fn current_opacity(&self) -> f32 {
        self.current_opacity
    }

    pub fn slide_offset(&self) -> f32 {
        self.slide_offset
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn is_visible(&self) -> bool {
        self.state != AnimationState::Idle
    }

    pub fn state(&self) -> AnimationState {
        self.state
    }

    pub fn time_until_fade(&self) -> Duration {
        match self.state {
            AnimationState::Visible => {
                let elapsed = self.shown_at.elapsed();
                if elapsed >= self.display_duration {
                    Duration::ZERO
                } else {
                    self.display_duration - elapsed
                }
            }
            _ => Duration::ZERO,
        }
    }
}
