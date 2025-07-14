use super::leaderboard;
use crate::colors::*;
use crate::config::*;
use crate::design::{ColorTheme, GameText, Spacing, TypographyStyle, UIComponent};
use crate::game::Game;
use macroquad::prelude::*;

pub fn draw_main_menu(game: &Game) {
    draw_rectangle(
        0.0,
        0.0,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        secondary_palette::BACKGROUND,
    );

    // Game title using new design system
    GameText::title_centered(
        "Yeti, Set, Go!",
        SCREEN_WIDTH / 2.0,
        SCREEN_HEIGHT / 2.0 - 80.0,
        &game.fonts,
    );

    // Instructions on the left side - using technical styling for code-like content
    let left_x = 40.0;
    let mut y_offset = SCREEN_HEIGHT / 2.0 - 50.0;

    // Mini leaderboard on the right side with scrolling
    leaderboard::draw_scrolling_mini_leaderboard(game, SCREEN_WIDTH - 240.0, y_offset);

    let subtitle = [
        (0.0, "A CI/CD Pipeline Runner"),
        (10.0, "for Impatient Devs"),
    ];

    for (x_offset, line) in subtitle {
        UIComponent::draw_text(
            line,
            left_x + x_offset,
            y_offset,
            TypographyStyle::BodyMedium,
            ColorTheme::Primary,
            &game.fonts,
        );
        y_offset += Spacing::Medium.as_f32();
    }

    y_offset += Spacing::Large.as_f32();

    // Game instructions - technical content
    let game_instructions = [
        "> [SPACE] or [Click] to Jump over problems",
        "> Collect good statuses // Avoid bad ones",
        "> Bonus points for dodging red items!",
        "> Complete checks to advance levels",
    ];

    for instruction in game_instructions {
        GameText::instructions(instruction, left_x, y_offset, &game.fonts);
        y_offset += Spacing::Medium.as_f32();
    }

    // Controls - highlighted
    UIComponent::draw_text(
        "[SPACE]: Start  //  [L]: Leaderboard",
        left_x,
        SCREEN_HEIGHT - 60.0,
        TypographyStyle::CodeMedium,
        ColorTheme::Primary,
        &game.fonts,
    );

    // Personal best in bottom left
    let personal_best = game.leaderboard.get_local_best_score();
    if personal_best > 0 {
        let personal_text = format!("Your Best: {}", personal_best);
        UIComponent::draw_text(
            &personal_text,
            SCREEN_WIDTH - 240.0,
            SCREEN_HEIGHT - 60.0,
            TypographyStyle::BodyMedium,
            ColorTheme::Warning,
            &game.fonts,
        );
    }
}

pub fn draw_game_over(game: &Game) {
    draw_rectangle(
        0.0,
        0.0,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        secondary_palette::BACKGROUND,
    );

    // Game over title
    GameText::title_centered(
        "GAME OVER",
        SCREEN_WIDTH / 2.0,
        SCREEN_HEIGHT / 2.0 - 80.0,
        &game.fonts,
    );

    // Show level reached
    let level_text = format!("Reached Level {}", game.level);
    UIComponent::draw_text_centered(
        &level_text,
        SCREEN_WIDTH / 2.0,
        SCREEN_HEIGHT / 2.0 - 45.0,
        TypographyStyle::CodeLarge,
        ColorTheme::Primary,
        &game.fonts,
    );

    // Final score
    let final_score_text = format!("Final Score: {}", game.score);
    GameText::score(
        &final_score_text,
        SCREEN_WIDTH / 2.0
            - TypographyStyle::BodyLarge
                .measure_text(&final_score_text, &game.fonts)
                .width
                / 2.0,
        SCREEN_HEIGHT / 2.0 - 15.0,
        &game.fonts,
    );

    // High score notification
    if game.is_new_high_score {
        GameText::success_message(
            "* NEW HIGH SCORE *",
            SCREEN_WIDTH / 2.0,
            SCREEN_HEIGHT / 2.0 + 15.0,
            &game.fonts,
        );
    }

    // Show rank if applicable
    if let Some(rank) = game.leaderboard.get_rank(game.score) {
        let rank_text = if game.is_new_high_score {
            format!("Leaderboard Rank: #{}", rank)
        } else {
            format!("Would rank #{} on leaderboard", rank)
        };
        UIComponent::draw_text_centered(
            &rank_text,
            SCREEN_WIDTH / 2.0,
            SCREEN_HEIGHT / 2.0 + 35.0,
            TypographyStyle::BodySmall,
            ColorTheme::Primary,
            &game.fonts,
        );
    }

    // Instructions
    let instructions = if game.is_new_high_score {
        "Press [SPACE] to enter your name!"
    } else {
        "Press [SPACE] to play again or [L] for leaderboard"
    };
    GameText::instructions(
        instructions,
        SCREEN_WIDTH / 2.0
            - TypographyStyle::CodeMedium
                .measure_text(instructions, &game.fonts)
                .width
                / 2.0,
        SCREEN_HEIGHT - 50.0,
        &game.fonts,
    );
}
