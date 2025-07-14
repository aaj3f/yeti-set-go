use crate::colors::*;
use crate::config::*;
use crate::design::ColorTheme;
use crate::design::UIComponent;
use crate::design::{GameText, Spacing, TypographyStyle};
use crate::game::Game;
use macroquad::prelude::*;

pub fn draw_game_ui(game: &Game) {
    draw_score_panel(game);
    draw_feedback_message(game);
}

fn draw_score_panel(game: &Game) {
    let x = 15.0;
    let mut y = 30.0;

    // Progress display
    let progress_text = format!(
        "{} / {} Passing Checks",
        game.checks_completed, game.checks_required
    );
    GameText::score(&progress_text, x, y, &game.fonts);
    y += Spacing::Large.as_f32();

    // Level display
    let level_text = format!("Level: {}", game.level);
    GameText::ui_label(&level_text, x, y, &game.fonts);
    y += Spacing::Medium.as_f32();

    // Score display
    let score_text = format!("Score: {}", game.score);
    GameText::ui_label(&score_text, x, y, &game.fonts);
}

fn draw_feedback_message(game: &Game) {
    if !game.feedback_message.is_empty() && game.feedback_timer > 0.0 {
        let box_x = SCREEN_WIDTH - FEEDBACK_BOX_WIDTH - 20.0;
        let box_y = 20.0;

        // Draw black border
        draw_rectangle(
            box_x - 3.0,
            box_y - 3.0,
            FEEDBACK_BOX_WIDTH + 6.0,
            FEEDBACK_BOX_HEIGHT + 6.0,
            UI_BORDER,
        );

        // Draw white background
        draw_rectangle(
            box_x,
            box_y,
            FEEDBACK_BOX_WIDTH,
            FEEDBACK_BOX_HEIGHT,
            UI_BACKGROUND,
        );

        // Draw technical feedback with word wrapping
        let text_x = box_x + 10.0;
        let text_y = box_y + 20.0;
        let line_height = Spacing::Medium.as_f32();

        let words: Vec<&str> = game.feedback_message.split_whitespace().collect();
        let mut current_line = String::new();
        let mut y_offset = 0.0;

        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            let test_width = TypographyStyle::CodeLarge
                .measure_text(&test_line, &game.fonts)
                .width;

            if test_width <= FEEDBACK_BOX_WIDTH - 20.0 {
                current_line = test_line;
            } else {
                if !current_line.is_empty() {
                    // GameText::technical_feedback(&current_line, text_x, text_y + y_offset, &game.fonts);
                    UIComponent::draw_text(
                        &current_line,
                        text_x,
                        text_y + y_offset,
                        TypographyStyle::CodeLarge,
                        ColorTheme::Secondary,
                        &game.fonts,
                    );
                    y_offset += line_height;
                }
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            // GameText::technical_feedback(&current_line, text_x, text_y + y_offset, &game.fonts);
            UIComponent::draw_text(
                &current_line,
                text_x,
                text_y + y_offset,
                TypographyStyle::CodeLarge,
                ColorTheme::Secondary,
                &game.fonts,
            );
        }
    }
}

pub fn draw_instructions(game: &Game) {
    let instructions = "SPACE or Click to Jump | Collect Good Items | Avoid Bad Items";
    GameText::instructions(instructions, 10.0, SCREEN_HEIGHT - 20.0, &game.fonts);
}
