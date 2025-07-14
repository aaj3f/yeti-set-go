use macroquad::prelude::*;

pub fn ordinal_suffix(n: usize) -> String {
    let suffix = match n % 100 {
        11..=13 => "th", // Special case for 11th, 12th, 13th
        _ => match n % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };
    format!("{}{}", n, suffix)
}

#[derive(Debug, Clone)]
pub struct GameFonts {
    pub primary: Option<Font>, // Gotham-Medium.otf - for headings, UI elements, scores
    pub monospace: Option<Font>, // For code, technical feedback, instructions
}

impl GameFonts {
    pub fn new() -> Self {
        Self {
            primary: None,
            monospace: None,
        }
    }
}

// Typography styles based on semantic meaning
#[derive(Debug, Clone, Copy)]
pub enum TypographyStyle {
    // Display styles - for large, prominent text
    DisplayLarge,  // Game title, major headings
    DisplayMedium, // Level complete messages, game over
    DisplaySmall,  // Section headings

    // Body styles - for readable content
    BodyLarge,  // Score displays, important info
    BodyMedium, // Standard UI text, player names
    BodySmall,  // Secondary info, dates

    // Technical styles - for code/system content (monospace)
    CodeLarge,  // Large technical feedback
    CodeMedium, // Standard feedback messages
    CodeSmall,  // Instructions, small technical text

    // UI styles - for interface elements
    UIButton,  // Button text
    UILabel,   // Form labels
    UIInput,   // Input field text
    UICaption, // Small captions, hints
}

impl TypographyStyle {
    pub fn get_params<'a>(&self, fonts: &'a GameFonts, color: Color) -> TextParams<'a> {
        let (font, size) = match self {
            // Display styles use primary font with large sizes
            TypographyStyle::DisplayLarge => (fonts.primary.as_ref(), 40),
            TypographyStyle::DisplayMedium => (fonts.primary.as_ref(), 32),
            TypographyStyle::DisplaySmall => (fonts.primary.as_ref(), 24),

            // Body styles use primary font with medium sizes
            TypographyStyle::BodyLarge => (fonts.primary.as_ref(), 20),
            TypographyStyle::BodyMedium => (fonts.primary.as_ref(), 16),
            TypographyStyle::BodySmall => (fonts.primary.as_ref(), 14),

            // Technical styles use monospace font
            TypographyStyle::CodeLarge => (fonts.monospace.as_ref(), 18),
            TypographyStyle::CodeMedium => (fonts.monospace.as_ref(), 16),
            TypographyStyle::CodeSmall => (fonts.monospace.as_ref(), 14),

            // UI styles use primary font with specific sizing
            TypographyStyle::UIButton => (fonts.primary.as_ref(), 18),
            TypographyStyle::UILabel => (fonts.primary.as_ref(), 16),
            TypographyStyle::UIInput => (fonts.primary.as_ref(), 18),
            TypographyStyle::UICaption => (fonts.primary.as_ref(), 12),
        };

        TextParams {
            font,
            font_size: size,
            color,
            ..Default::default()
        }
    }

    pub fn measure_text(&self, text: &str, fonts: &GameFonts) -> TextDimensions {
        let (font, size) = match self {
            TypographyStyle::DisplayLarge => (fonts.primary.as_ref(), 40),
            TypographyStyle::DisplayMedium => (fonts.primary.as_ref(), 32),
            TypographyStyle::DisplaySmall => (fonts.primary.as_ref(), 24),
            TypographyStyle::BodyLarge => (fonts.primary.as_ref(), 20),
            TypographyStyle::BodyMedium => (fonts.primary.as_ref(), 16),
            TypographyStyle::BodySmall => (fonts.primary.as_ref(), 14),
            TypographyStyle::CodeLarge => (fonts.monospace.as_ref(), 18),
            TypographyStyle::CodeMedium => (fonts.monospace.as_ref(), 16),
            TypographyStyle::CodeSmall => (fonts.monospace.as_ref(), 14),
            TypographyStyle::UIButton => (fonts.primary.as_ref(), 18),
            TypographyStyle::UILabel => (fonts.primary.as_ref(), 16),
            TypographyStyle::UIInput => (fonts.primary.as_ref(), 18),
            TypographyStyle::UICaption => (fonts.primary.as_ref(), 12),
        };

        measure_text(text, font, size, 1.0)
    }
}

// Color themes for different contexts
#[derive(Debug, Clone, Copy)]
pub enum ColorTheme {
    Primary,   // Main game UI - light on dark
    Secondary, // Overlay/modal content - dark on light
    Success,   // Positive feedback, achievements
    Warning,   // Important but not critical
    Error,     // Critical issues, failures
    Neutral,   // Inactive/disabled content
    Technical, // Code/system feedback
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub foreground: Color,
    pub background: Color,
    pub accent: Color,
    pub border: Color,
}

impl ColorTheme {
    pub fn get_colors(&self) -> ThemeColors {
        use crate::colors::*;

        match self {
            ColorTheme::Primary => ThemeColors {
                foreground: ICE_BLUE,
                background: DEEP,
                accent: VIBRANT_BLUE,
                border: PEAK,
            },
            ColorTheme::Secondary => ThemeColors {
                foreground: DEEP,
                background: ICE_BLUE,
                accent: PLUM,
                border: PURPLE,
            },
            ColorTheme::Success => ThemeColors {
                foreground: SUCCESS_GREEN,
                background: Color::new(0.0, 0.4, 0.0, 0.1),
                accent: SUCCESS_GREEN,
                border: SUCCESS_GREEN,
            },
            ColorTheme::Warning => ThemeColors {
                foreground: WARNING_YELLOW,
                background: Color::new(1.0, 0.8, 0.0, 0.1),
                accent: WARNING_YELLOW,
                border: WARNING_YELLOW,
            },
            ColorTheme::Error => ThemeColors {
                foreground: ERROR_RED,
                background: Color::new(1.0, 0.3, 0.1, 0.1),
                accent: ERROR_RED,
                border: ERROR_RED,
            },
            ColorTheme::Neutral => ThemeColors {
                foreground: GREY,
                background: METAL,
                accent: GREY,
                border: GREY,
            },
            ColorTheme::Technical => ThemeColors {
                foreground: TEAL,
                background: Color::new(0.1, 0.1, 0.2, 0.8),
                accent: VIBRANT_BLUE,
                border: METAL,
            },
        }
    }
}

// Spacing system for consistent layouts
#[derive(Debug, Clone, Copy)]
pub enum Spacing {
    None = 0,
    XSmall = 4,
    Small = 8,
    Medium = 16,
    Large = 24,
    XLarge = 32,
    XXLarge = 48,
}

impl Spacing {
    pub fn as_f32(self) -> f32 {
        self as i32 as f32
    }
}

// UI component patterns
pub struct UIComponent;

impl UIComponent {
    // Draw text with semantic styling
    pub fn draw_text(
        text: &str,
        x: f32,
        y: f32,
        style: TypographyStyle,
        theme: ColorTheme,
        fonts: &GameFonts,
    ) {
        let colors = theme.get_colors();
        let params = style.get_params(fonts, colors.foreground);
        draw_text_ex(text, x, y, params);
    }

    // Draw centered text
    pub fn draw_text_centered(
        text: &str,
        center_x: f32,
        y: f32,
        style: TypographyStyle,
        theme: ColorTheme,
        fonts: &GameFonts,
    ) {
        let size = style.measure_text(text, fonts);
        let x = center_x - size.width / 2.0;
        Self::draw_text(text, x, y, style, theme, fonts);
    }

    // Draw text with background box
    pub fn draw_text_box(
        text: &str,
        x: f32,
        y: f32,
        padding: Spacing,
        style: TypographyStyle,
        theme: ColorTheme,
        fonts: &GameFonts,
    ) {
        let colors = theme.get_colors();
        let size = style.measure_text(text, fonts);
        let pad = padding.as_f32();

        // Draw background
        draw_rectangle(
            x - pad,
            y - size.height - pad,
            size.width + pad * 2.0,
            size.height + pad * 2.0,
            colors.background,
        );

        // Draw border
        draw_rectangle_lines(
            x - pad,
            y - size.height - pad,
            size.width + pad * 2.0,
            size.height + pad * 2.0,
            2.0,
            colors.border,
        );

        // Draw text
        Self::draw_text(text, x, y, style, theme, fonts);
    }
}

// Game-specific semantic styles for easy use
pub struct GameText;

impl GameText {
    pub fn title(text: &str, x: f32, y: f32, fonts: &GameFonts) {
        UIComponent::draw_text(
            text,
            x,
            y,
            TypographyStyle::DisplayLarge,
            ColorTheme::Primary,
            fonts,
        );
    }

    pub fn title_centered(text: &str, center_x: f32, y: f32, fonts: &GameFonts) {
        UIComponent::draw_text_centered(
            text,
            center_x,
            y,
            TypographyStyle::DisplayLarge,
            ColorTheme::Primary,
            fonts,
        );
    }

    pub fn heading(text: &str, x: f32, y: f32, fonts: &GameFonts) {
        UIComponent::draw_text(
            text,
            x,
            y,
            TypographyStyle::DisplayMedium,
            ColorTheme::Primary,
            fonts,
        );
    }

    pub fn heading_centered(text: &str, center_x: f32, y: f32, fonts: &GameFonts) {
        UIComponent::draw_text_centered(
            text,
            center_x,
            y,
            TypographyStyle::DisplayMedium,
            ColorTheme::Primary,
            fonts,
        );
    }

    pub fn score(text: &str, x: f32, y: f32, fonts: &GameFonts) {
        UIComponent::draw_text(
            text,
            x,
            y,
            TypographyStyle::BodyLarge,
            ColorTheme::Secondary,
            fonts,
        );
    }

    pub fn ui_label(text: &str, x: f32, y: f32, fonts: &GameFonts) {
        UIComponent::draw_text(
            text,
            x,
            y,
            TypographyStyle::BodyMedium,
            ColorTheme::Secondary,
            fonts,
        );
    }

    pub fn ui_secondary(text: &str, x: f32, y: f32, fonts: &GameFonts) {
        UIComponent::draw_text(
            text,
            x,
            y,
            TypographyStyle::BodySmall,
            ColorTheme::Neutral,
            fonts,
        );
    }

    pub fn technical_feedback(text: &str, x: f32, y: f32, fonts: &GameFonts) {
        UIComponent::draw_text(
            text,
            x,
            y,
            TypographyStyle::CodeMedium,
            ColorTheme::Technical,
            fonts,
        );
    }

    pub fn instructions(text: &str, x: f32, y: f32, fonts: &GameFonts) {
        UIComponent::draw_text(
            text,
            x,
            y,
            TypographyStyle::CodeMedium,
            ColorTheme::Technical,
            fonts,
        );
    }

    pub fn success_message(text: &str, center_x: f32, y: f32, fonts: &GameFonts) {
        UIComponent::draw_text_centered(
            text,
            center_x,
            y,
            TypographyStyle::CodeLarge,
            ColorTheme::Success,
            fonts,
        );
    }

    pub fn error_message(text: &str, center_x: f32, y: f32, fonts: &GameFonts) {
        UIComponent::draw_text_centered(
            text,
            center_x,
            y,
            TypographyStyle::BodyMedium,
            ColorTheme::Error,
            fonts,
        );
    }
}
