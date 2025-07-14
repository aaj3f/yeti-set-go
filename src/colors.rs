use macroquad::prelude::Color;

pub mod main_palette {
    use super::{DEEP, ICE_BLUE, PLUM, PURPLE};
    use macroquad::prelude::Color;

    pub const BACKGROUND: Color = Color::new(ICE_BLUE.r, ICE_BLUE.g, ICE_BLUE.b, 0.7); // #CEF1FF
    pub const PRIMARY: Color = DEEP; // #091133
    pub const ACCENT: Color = PLUM; // #171F69
    pub const SUBSCRIPT: Color = PURPLE; // #4B56A5
}

pub mod secondary_palette {
    use super::{DEEP, ICE_BLUE, PEAK, VIBRANT_BLUE};
    use macroquad::prelude::Color;

    pub const BACKGROUND: Color = Color::new(DEEP.r, DEEP.g, DEEP.b, 0.7); // #091133
    pub const PRIMARY: Color = ICE_BLUE; // #CEF1FF
    pub const ACCENT: Color = PEAK; // #C6D4FF
    pub const SUBSCRIPT: Color = VIBRANT_BLUE; // #13C6FF
}

// Primary Fluree Brand Colors
pub const ICE_BLUE: Color = Color::new(0.808, 0.945, 1.0, 1.0); // #CEF1FF (206, 241, 255)
pub const VIBRANT_BLUE: Color = Color::new(0.075, 0.776, 1.0, 1.0); // #13C6FF (19, 198, 255)
pub const DEEP: Color = Color::new(0.035, 0.067, 0.204, 1.0); // #091133 (9, 17, 51)
pub const FLUREE_SAFE_BLUE: Color = Color::new(0.0, 0.627, 0.820, 1.0); // #00A0D1 (0, 160, 209)
pub const PEAK: Color = Color::new(0.776, 0.831, 1.0, 1.0); // #C6D4FF (198, 212, 255)
pub const VIOLET: Color = Color::new(0.718, 0.459, 0.839, 1.0); // #B775D6 (183, 117, 214)
pub const PURPLE: Color = Color::new(0.294, 0.337, 0.647, 1.0); // #4B56A5 (75, 86, 165)
pub const PLUM: Color = Color::new(0.090, 0.122, 0.412, 1.0); // #171F69 (23, 31, 105)

// Supporting Colors
pub const GREY: Color = Color::new(0.592, 0.592, 0.592, 1.0); // #979797 (151, 151, 151)
pub const METAL: Color = Color::new(0.365, 0.412, 0.439, 1.0); // #5D6970 (93, 105, 112)
pub const TEAL: Color = Color::new(0.094, 0.812, 0.859, 1.0); // #18CFDB (24, 207, 219)
pub const EMBER: Color = Color::new(1.0, 0.298, 0.075, 1.0); // #FF4C13 (255, 76, 19)

// Game-specific color applications
pub const TEXT_PRIMARY: Color = DEEP;
pub const TEXT_SECONDARY: Color = GREY;
pub const TEXT_LIGHT: Color = ICE_BLUE;
pub const TEXT_ACCENT: Color = VIBRANT_BLUE;

pub const BACKGROUND_PRIMARY: Color = DEEP;
pub const BACKGROUND_SECONDARY: Color = PEAK;
pub const BACKGROUND_OVERLAY: Color = Color::new(0.035, 0.067, 0.204, 0.7); // DEEP with alpha

pub const UI_BACKGROUND: Color = PEAK;
pub const UI_BORDER: Color = DEEP;
pub const UI_HIGHLIGHT: Color = VIBRANT_BLUE;

// Success/Error colors (maintaining brand consistency while using recognizable colors)
pub const SUCCESS_GREEN: Color = Color::new(0.0, 0.667, 0.0, 1.0); // #00AA00 (standard green)
pub const ERROR_RED: Color = EMBER; // Use brand Ember for errors
pub const WARNING_YELLOW: Color = Color::new(1.0, 0.843, 0.0, 1.0); // #FFD700 (gold for warnings)

// Medal colors for leaderboard
pub const MEDAL_GOLD: Color = WARNING_YELLOW;
pub const MEDAL_SILVER: Color = ICE_BLUE;
pub const MEDAL_BRONZE: Color = Color::new(0.8, 0.5, 0.2, 1.0); // Bronze color

// Feedback message colors
pub const FEEDBACK_SUCCESS: Color = SUCCESS_GREEN;
pub const FEEDBACK_INFO: Color = VIBRANT_BLUE;
pub const FEEDBACK_WARNING: Color = WARNING_YELLOW;
