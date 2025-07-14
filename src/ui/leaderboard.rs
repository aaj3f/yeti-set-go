use crate::colors::*;
use crate::config::*;
use crate::design::ordinal_suffix;
use crate::design::{ColorTheme, GameText, TypographyStyle, UIComponent};
use crate::game::Game;
use macroquad::prelude::*;

pub fn draw_leaderboard_view(game: &Game) {
    // Background
    draw_rectangle(
        0.0,
        0.0,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        secondary_palette::BACKGROUND,
    );

    // Title
    GameText::heading_centered("!! SWEATY YETIS !!", SCREEN_WIDTH / 2.0, 40.0, &game.fonts);

    // Headers
    GameText::ui_secondary("RANK", 50.0, 80.0, &game.fonts);
    GameText::ui_secondary("NAME", 120.0, 80.0, &game.fonts);
    GameText::ui_secondary("SCORE", 300.0, 80.0, &game.fonts);
    GameText::ui_secondary("LEVEL", 400.0, 80.0, &game.fonts);

    // Leaderboard entries
    let start_y = 100.0 - game.leaderboard_scroll;
    let line_height = 25.0;

    for (i, high_score) in game.leaderboard.scores.iter().enumerate() {
        let y = start_y + (i as f32 * line_height);

        // Skip if outside visible area
        if !(80.0..=SCREEN_HEIGHT - 20.0).contains(&y) {
            continue;
        }

        // Determine color theme based on rank
        let theme = match i {
            0..=2 => ColorTheme::Primary,
            _ => ColorTheme::Neutral,
        };

        // Rank
        let rank_text = format!("#{}", i + 1);
        UIComponent::draw_text(
            &rank_text,
            50.0,
            y + 5.0,
            TypographyStyle::BodyMedium,
            theme,
            &game.fonts,
        );

        // Name (truncate if too long)
        let name = if high_score.name.len() > 15 {
            format!("{}...", &high_score.name[..12])
        } else {
            high_score.name.clone()
        };
        UIComponent::draw_text(
            &name,
            120.0,
            y + 5.0,
            TypographyStyle::BodyMedium,
            theme,
            &game.fonts,
        );

        // Score
        let score_text = format!("{}", high_score.score);
        UIComponent::draw_text(
            &score_text,
            300.0,
            y + 5.0,
            TypographyStyle::BodyMedium,
            theme,
            &game.fonts,
        );

        // Level
        let level_text = format!("{}", high_score.level);
        UIComponent::draw_text(
            &level_text,
            400.0,
            y + 5.0,
            TypographyStyle::BodyMedium,
            theme,
            &game.fonts,
        );

        // Date (right aligned, smaller)
        let date_str = high_score.timestamp.format("%m/%d").to_string();
        let date_size = TypographyStyle::UICaption.measure_text(&date_str, &game.fonts);
        UIComponent::draw_text(
            &date_str,
            SCREEN_WIDTH - 60.0 - date_size.width,
            y + 5.0,
            TypographyStyle::UICaption,
            ColorTheme::Neutral,
            &game.fonts,
        );
    }

    // No scores message or loading indicator
    if game.leaderboard.scores.is_empty() && game.api_loading {
        UIComponent::draw_text_centered(
            "Loading leaderboard...",
            SCREEN_WIDTH / 2.0,
            SCREEN_HEIGHT / 2.0,
            TypographyStyle::BodyLarge,
            ColorTheme::Secondary,
            &game.fonts,
        );
    }

    // Instructions
    GameText::instructions(
        "[UP]/[DOWN] Scroll  //  [SPACE] Return to Menu",
        SCREEN_WIDTH / 2.0
            - TypographyStyle::CodeMedium
                .measure_text(
                    "[UP]/[DOWN] Scroll  //  [SPACE] Return to Menu",
                    &game.fonts,
                )
                .width
                / 2.0,
        SCREEN_HEIGHT - 60.0,
        &game.fonts,
    );

    // Scroll indicator
    if game.leaderboard.scores.len() > 8 {
        let scroll_progress = game.leaderboard_scroll / 400.0;
        let indicator_height = 100.0;
        let indicator_y = 100.0 + scroll_progress * (SCREEN_HEIGHT - 200.0 - indicator_height);

        draw_rectangle(
            SCREEN_WIDTH - 10.0,
            indicator_y,
            6.0,
            indicator_height,
            UI_HIGHLIGHT,
        );
    }
}

// pub fn draw_mini_leaderboard(game: &Game, x: f32, y: f32) {
//     let top_3 = game.leaderboard.get_top_3();

//     if top_3.is_empty() {
//         let message = "No high scores yet!";
//         draw_text(message, x, y, 14.0, TEXT_SECONDARY);
//         return;
//     }

//     let title = "🏆 TOP SCORES";
//     draw_text(title, x, y, 16.0, WARNING_YELLOW);

//     for (i, high_score) in top_3.iter().enumerate() {
//         let rank_y = y + 25.0 + (i as f32 * 20.0);

//         let medal = match i {
//             0 => "🥇",
//             1 => "🥈",
//             2 => "🥉",
//             _ => "•",
//         };

//         let text = format!("{} {} - {}", medal, high_score.name, high_score.score);
//         let text_color = match i {
//             0 => Color::new(1.0, 0.84, 0.0, 1.0),   // Gold
//             1 => Color::new(0.75, 0.75, 0.75, 1.0), // Silver
//             2 => Color::new(0.8, 0.5, 0.2, 1.0),    // Bronze
//             _ => WHITE,
//         };

//         draw_text(&text, x, rank_y, 14.0, text_color);
//     }
// }

pub fn draw_scrolling_mini_leaderboard(game: &Game, x: f32, y: f32) {
    if game.leaderboard.scores.is_empty() {
        return;
    }

    // Title
    UIComponent::draw_text(
        "-- TOP SCORES --",
        x,
        y,
        TypographyStyle::BodyMedium,
        ColorTheme::Warning,
        &game.fonts,
    );

    // Create a clipping area for scrolling effect
    let visible_height = 80.0; // Height to show 3-4 entries
    let line_height = 20.0;

    let num_scores = game.leaderboard.scores.len();
    let total_height = num_scores as f32 * line_height;
    let max_visible_entries = (visible_height / line_height).ceil() as usize + 1;

    // Only scroll if we have more entries than can fit
    if num_scores <= max_visible_entries {
        // Static display - no scrolling needed
        for (i, high_score) in game.leaderboard.scores.iter().enumerate() {
            let entry_y = y + 25.0 + (i as f32 * line_height);

            let rank_string = ordinal_suffix(i + 1);
            let text = format!(
                "{} {} - {}",
                rank_string.as_str(),
                high_score.name,
                high_score.score
            );
            let text_color = match i {
                0 => MEDAL_GOLD,
                1 => MEDAL_SILVER,
                2 => MEDAL_BRONZE,
                _ => TEXT_LIGHT,
            };

            let params = TypographyStyle::BodySmall.get_params(&game.fonts, text_color);
            draw_text_ex(&text, x, entry_y, params);
        }
    } else {
        // Scrolling display with seamless looping
        let wrapped_scroll = game.mini_leaderboard_scroll % total_height;

        let mut entries_drawn = 0;

        // Draw two cycles of entries to ensure seamless looping
        for cycle in 0..2 {
            for (i, high_score) in game.leaderboard.scores.iter().enumerate() {
                let entry_y = y + 25.0 + (i as f32 * line_height) + (cycle as f32 * total_height)
                    - wrapped_scroll;

                // Only draw if in visible area
                if entry_y >= y + 20.0
                    && entry_y <= y + visible_height + 20.0
                    && entries_drawn < max_visible_entries
                {
                    let rank_string = ordinal_suffix(i + 1);

                    let text = format!(
                        "{} {} - {}",
                        rank_string.as_str(),
                        high_score.name,
                        high_score.score
                    );
                    let text_color = match i {
                        0 => MEDAL_GOLD,
                        1 => MEDAL_SILVER,
                        2 => MEDAL_BRONZE,
                        _ => TEXT_LIGHT,
                    };

                    // Fade effect for entries at edges
                    let fade_alpha = if entry_y < y + 30.0 || entry_y > y + visible_height {
                        0.5
                    } else {
                        1.0
                    };

                    let faded_color = Color::new(
                        text_color.r,
                        text_color.g,
                        text_color.b,
                        text_color.a * fade_alpha,
                    );
                    let params = TypographyStyle::BodySmall.get_params(&game.fonts, faded_color);
                    draw_text_ex(&text, x, entry_y, params);
                    entries_drawn += 1;
                }
            }
        }
    }
}
