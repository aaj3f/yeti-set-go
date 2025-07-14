use crate::colors::*;
use crate::config::*;
use crate::design::{ColorTheme, GameText, TypographyStyle, UIComponent};
use crate::game::Game;
use macroquad::prelude::*;

pub fn draw_name_input(game: &Game) {
    // Semi-transparent overlay
    draw_rectangle(
        0.0,
        0.0,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        main_palette::BACKGROUND,
    );

    // Celebration message
    GameText::success_message(
        "* NEW HIGH SCORE! *",
        SCREEN_WIDTH / 2.0,
        SCREEN_HEIGHT / 2.0 - 80.0,
        &game.fonts,
    );

    // Score display
    let score_text = format!("Score: {}", game.score);
    GameText::score(
        &score_text,
        SCREEN_WIDTH / 2.0
            - TypographyStyle::BodyLarge
                .measure_text(&score_text, &game.fonts)
                .width
                / 2.0,
        SCREEN_HEIGHT / 2.0 - 40.0,
        &game.fonts,
    );

    // Name input prompt
    UIComponent::draw_text_centered(
        "Enter your name:",
        SCREEN_WIDTH / 2.0,
        SCREEN_HEIGHT / 2.0,
        TypographyStyle::BodyLarge,
        ColorTheme::Secondary,
        &game.fonts,
    );

    // Input box
    let box_width = 300.0;
    let box_height = 40.0;
    let box_x = SCREEN_WIDTH / 2.0 - box_width / 2.0;
    let box_y = SCREEN_HEIGHT / 2.0 + 20.0;

    // Input box border
    draw_rectangle(
        box_x - 2.0,
        box_y - 2.0,
        box_width + 4.0,
        box_height + 4.0,
        main_palette::PRIMARY,
    );

    // Input box background
    draw_rectangle(box_x, box_y, box_width, box_height, PEAK);

    // Input text
    let input_display = if game.player_name_input.is_empty() {
        "Type here...".to_string()
    } else {
        game.player_name_input.clone()
    };

    UIComponent::draw_text(
        &input_display,
        box_x + 10.0,
        box_y + 25.0,
        TypographyStyle::UIInput,
        ColorTheme::Secondary,
        &game.fonts,
    );

    // Blinking cursor
    if !game.player_name_input.is_empty() {
        let cursor_time = get_time() % 1.0;
        if cursor_time < 0.5 {
            let text_width = TypographyStyle::UIInput
                .measure_text(&game.player_name_input, &game.fonts)
                .width;
            UIComponent::draw_text(
                "|",
                box_x + 10.0 + text_width,
                box_y + 25.0,
                TypographyStyle::UIInput,
                ColorTheme::Secondary,
                &game.fonts,
            );
        }
    }

    // Instructions
    // GameText::instructions(
    //     "Type your name and press [ENTER]",
    //     SCREEN_WIDTH / 2.0
    //         - TypographyStyle::CodeMedium
    //             .measure_text("Type your name and press [ENTER]", &game.fonts)
    //             .width
    //             / 2.0,
    //     SCREEN_HEIGHT / 2.0 + 80.0,
    //     &game.fonts,
    // );

    UIComponent::draw_text(
        "Type your name and press [ENTER]",
        SCREEN_WIDTH / 2.0
            - TypographyStyle::CodeMedium
                .measure_text("Type your name and press [ENTER]", &game.fonts)
                .width
                / 2.0,
        SCREEN_HEIGHT / 2.0 + 80.0,
        TypographyStyle::CodeMedium,
        ColorTheme::Secondary,
        &game.fonts,
    );
}
