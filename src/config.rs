use macroquad::prelude::*;

// Screen dimensions
pub const SCREEN_WIDTH: f32 = 640.0;
pub const SCREEN_HEIGHT: f32 = 270.0;
pub const GROUND_Y: f32 = 210.0;

// Game physics
pub const JUMP_VELOCITY: f32 = -350.0;
pub const GRAVITY: f32 = 800.0;
pub const COLLISION_GRACE_MARGIN: f32 = 8.0;

// Game mechanics
pub const INITIAL_SPAWN_RATE: f32 = 2.0;
pub const MIN_SPAWN_RATE: f32 = 0.5;
pub const SPEED_INCREASE_PER_LEVEL: f32 = 20.0;
pub const BASE_ITEM_SPEED: f32 = 200.0;
pub const PIPELINE_BASE_SPEED: f32 = 100.0;
pub const PIPELINE_SPEED_INCREASE: f32 = 10.0;
pub const PIPELINE_SCROLL_RESET: f32 = 128.0;

// Entity sizes
pub const YETI_WIDTH: f32 = 48.0;
pub const YETI_HEIGHT: f32 = 48.0;
pub const ITEM_WIDTH: f32 = 32.0;
pub const ITEM_HEIGHT: f32 = 32.0;

// Probabilities
pub const GOOD_ITEM_PROBABILITY: f32 = 0.65;

// UI constants
pub const FEEDBACK_BOX_WIDTH: f32 = 300.0;
pub const FEEDBACK_BOX_HEIGHT: f32 = 60.0;
pub const FEEDBACK_DISPLAY_TIME: f32 = 10.0;
pub const COLLISION_GRACE_TIME: f32 = 0.5;

// Platform-specific configurations
#[cfg(target_os = "android")]
pub const TOUCH_ENABLED: bool = true;

#[cfg(target_os = "ios")]
pub const TOUCH_ENABLED: bool = true;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub const TOUCH_ENABLED: bool = false;

// Dev mode settings
pub const DEV_MODE_ENABLED: bool = false; // Set to false to disable dev mode completely

// Window configuration
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Yeti, Set, Go!".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}
