use crate::colors;
use crate::colors::*;
use crate::config::*;
use crate::design::{ColorTheme, GameText, TypographyStyle, UIComponent};
use crate::game::Game;
use macroquad::prelude::*;

pub fn draw_level_complete(game: &Game) {
    // Semi-transparent overlay
    draw_rectangle(
        0.0,
        0.0,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        secondary_palette::BACKGROUND,
    );

    // Main message
    GameText::heading_centered(
        &game.level_complete_message,
        SCREEN_WIDTH / 2.0,
        SCREEN_HEIGHT / 2.0 - 20.0,
        &game.fonts,
    );

    // Submessage
    if !game.level_complete_submessage.is_empty() {
        UIComponent::draw_text_centered(
            &game.level_complete_submessage,
            SCREEN_WIDTH / 2.0,
            SCREEN_HEIGHT / 2.0,
            TypographyStyle::CodeLarge,
            ColorTheme::Primary,
            &game.fonts,
        );
    }

    // Show score bonus
    let level = game.level - 1; // We've already incremented level
    let bonus = crate::game::scoring::calculate_level_score_bonus(level);
    let bonus_text = format!("+{} Level Bonus!", bonus);
    UIComponent::draw_text_centered(
        &bonus_text,
        SCREEN_WIDTH / 2.0,
        SCREEN_HEIGHT / 2.0 + 30.0,
        TypographyStyle::BodyLarge,
        ColorTheme::Success,
        &game.fonts,
    );

    // Animated progress indicator
    let progress = 1.0 - (game.level_complete_timer / 2.5);
    let bar_width = 200.0;
    let bar_height = 8.0;
    let bar_x = SCREEN_WIDTH / 2.0 - bar_width / 2.0;
    let bar_y = SCREEN_HEIGHT / 2.0 + 60.0;

    // Background bar
    draw_rectangle(bar_x, bar_y, bar_width, bar_height, METAL);

    // Progress bar
    draw_rectangle(
        bar_x,
        bar_y,
        bar_width * progress,
        bar_height,
        colors::SUCCESS_GREEN,
    );
}
