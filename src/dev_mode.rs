use crate::colors::*;
use crate::config::*;
use crate::design::{ColorTheme, GameText, TypographyStyle, UIComponent};
use crate::game::{Game, GameState};
use crate::highscores::{HighScore, Leaderboard};
use chrono::Utc;
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum DevScreen {
    MainMenu,
    GameOver,
    LevelComplete,
    NameInput,
    Leaderboard,
    Playing,
    TypographyShowcase,
    ColorShowcase,
}

impl DevScreen {
    pub fn next(&self) -> DevScreen {
        match self {
            DevScreen::MainMenu => DevScreen::GameOver,
            DevScreen::GameOver => DevScreen::LevelComplete,
            DevScreen::LevelComplete => DevScreen::NameInput,
            DevScreen::NameInput => DevScreen::Leaderboard,
            DevScreen::Leaderboard => DevScreen::Playing,
            DevScreen::Playing => DevScreen::TypographyShowcase,
            DevScreen::TypographyShowcase => DevScreen::ColorShowcase,
            DevScreen::ColorShowcase => DevScreen::MainMenu,
        }
    }

    pub fn prev(&self) -> DevScreen {
        match self {
            DevScreen::MainMenu => DevScreen::ColorShowcase,
            DevScreen::GameOver => DevScreen::MainMenu,
            DevScreen::LevelComplete => DevScreen::GameOver,
            DevScreen::NameInput => DevScreen::LevelComplete,
            DevScreen::Leaderboard => DevScreen::NameInput,
            DevScreen::Playing => DevScreen::Leaderboard,
            DevScreen::TypographyShowcase => DevScreen::Playing,
            DevScreen::ColorShowcase => DevScreen::TypographyShowcase,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            DevScreen::MainMenu => "Main Menu",
            DevScreen::GameOver => "Game Over",
            DevScreen::LevelComplete => "Level Complete",
            DevScreen::NameInput => "Name Input",
            DevScreen::Leaderboard => "Leaderboard",
            DevScreen::Playing => "Playing",
            DevScreen::TypographyShowcase => "Typography Showcase",
            DevScreen::ColorShowcase => "Color Showcase",
        }
    }
}

pub struct DevMode {
    pub enabled: bool,
    pub current_screen: DevScreen,
    pub mock_game: Game,
    pub show_overlay: bool,
}

impl DevMode {
    pub fn new() -> Self {
        let mut mock_game = Game::new();
        Self::populate_mock_data(&mut mock_game);

        Self {
            enabled: false,
            current_screen: DevScreen::MainMenu,
            mock_game,
            show_overlay: true,
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
        if self.enabled {
            // Reset to first screen and show overlay when entering dev mode
            self.current_screen = DevScreen::MainMenu;
            self.show_overlay = true;
        }
    }

    pub fn handle_input(&mut self) {
        if !self.enabled {
            return;
        }

        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::N) {
            self.current_screen = self.current_screen.next();
            Self::populate_mock_data(&mut self.mock_game);
        }

        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::P) {
            self.current_screen = self.current_screen.prev();
            Self::populate_mock_data(&mut self.mock_game);
        }

        if is_key_pressed(KeyCode::Escape) {
            self.enabled = false;
        }

        if is_key_pressed(KeyCode::H) {
            self.show_overlay = !self.show_overlay;
        }
    }

    pub fn get_current_game_state(&self) -> GameState {
        match self.current_screen {
            DevScreen::MainMenu => GameState::MainMenu,
            DevScreen::GameOver => GameState::GameOver,
            DevScreen::LevelComplete => GameState::LevelComplete,
            DevScreen::NameInput => GameState::NameInput,
            DevScreen::Leaderboard => GameState::ViewingLeaderboard,
            DevScreen::Playing => GameState::Playing,
            DevScreen::TypographyShowcase | DevScreen::ColorShowcase => GameState::MainMenu,
        }
    }

    fn populate_mock_data(game: &mut Game) {
        // Basic game stats
        game.score = 42850;
        game.level = 7;
        game.checks_completed = 12;
        game.checks_required = 15;
        game.is_new_high_score = true;
        game.player_name_input = "DevMaster".to_string();
        game.feedback_message =
            "CI pipeline succeeded! All tests passing, deployment ready for staging environment."
                .to_string();
        game.feedback_timer = 3.0;
        game.level_complete_message = "Issue #42 Done!".to_string();
        game.level_complete_submessage = "What else is assigned to me...".to_string();
        game.level_complete_timer = 2.0;

        // Mock leaderboard data
        let mut leaderboard = Leaderboard::new();

        let mock_scores = vec![
            HighScore {
                name: "CodeNinja".to_string(),
                score: 89650,
                level: 15,
                timestamp: Utc::now(),
            },
            HighScore {
                name: "PipelinePro".to_string(),
                score: 76420,
                level: 12,
                timestamp: Utc::now() - chrono::Duration::hours(2),
            },
            HighScore {
                name: "DevOpsGuru".to_string(),
                score: 68350,
                level: 11,
                timestamp: Utc::now() - chrono::Duration::days(1),
            },
            HighScore {
                name: "GitMaster".to_string(),
                score: 59870,
                level: 10,
                timestamp: Utc::now() - chrono::Duration::days(2),
            },
            HighScore {
                name: "TestRunner".to_string(),
                score: 52140,
                level: 9,
                timestamp: Utc::now() - chrono::Duration::days(3),
            },
            HighScore {
                name: "YetiHunter".to_string(),
                score: 48920,
                level: 8,
                timestamp: Utc::now() - chrono::Duration::days(5),
            },
            HighScore {
                name: "BuildBot".to_string(),
                score: 43750,
                level: 7,
                timestamp: Utc::now() - chrono::Duration::days(7),
            },
            HighScore {
                name: "MergeKing".to_string(),
                score: 38640,
                level: 6,
                timestamp: Utc::now() - chrono::Duration::days(10),
            },
        ];

        for score in mock_scores {
            leaderboard.add_score(score);
        }

        game.leaderboard = leaderboard;
    }

    pub fn draw_dev_overlay(&self, fonts: &crate::design::GameFonts) {
        if !self.enabled {
            return;
        }

        if !self.show_overlay {
            // When overlay is hidden, show a small indicator in the corner
            draw_rectangle(
                SCREEN_WIDTH - 80.0,
                5.0,
                75.0,
                25.0,
                Color::new(0.0, 0.0, 0.0, 0.7),
            );
            UIComponent::draw_text(
                "DEV [H]",
                SCREEN_WIDTH - 75.0,
                20.0,
                TypographyStyle::UICaption,
                ColorTheme::Warning,
                fonts,
            );
            return;
        }

        // Semi-transparent overlay at top
        draw_rectangle(0.0, 0.0, SCREEN_WIDTH, 60.0, Color::new(0.0, 0.0, 0.0, 0.8));

        // Current screen info
        let screen_text = format!(
            "DEV MODE: {} ({}/8)",
            self.current_screen.name(),
            self.get_screen_index() + 1
        );
        GameText::ui_label(&screen_text, 10.0, 25.0, fonts);

        // Navigation instructions
        let nav_text = "[←/P] Prev  [→/N] Next  [H] Hide Overlay  [ESC] Exit  [D] Toggle Dev Mode";
        GameText::instructions(&nav_text, 10.0, 45.0, fonts);
    }

    fn get_screen_index(&self) -> usize {
        match self.current_screen {
            DevScreen::MainMenu => 0,
            DevScreen::GameOver => 1,
            DevScreen::LevelComplete => 2,
            DevScreen::NameInput => 3,
            DevScreen::Leaderboard => 4,
            DevScreen::Playing => 5,
            DevScreen::TypographyShowcase => 6,
            DevScreen::ColorShowcase => 7,
        }
    }

    pub fn draw_custom_screen(&self, fonts: &crate::design::GameFonts) {
        match self.current_screen {
            DevScreen::TypographyShowcase => self.draw_typography_showcase(fonts),
            DevScreen::ColorShowcase => self.draw_color_showcase(fonts),
            _ => {} // Regular screens are handled by normal rendering
        }
    }

    fn draw_typography_showcase(&self, fonts: &crate::design::GameFonts) {
        // Background
        draw_rectangle(
            0.0,
            0.0,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            secondary_palette::BACKGROUND,
        );

        let mut y = 80.0;
        let x_left = 50.0;
        let x_right = SCREEN_WIDTH / 2.0 + 50.0;
        let line_spacing = 45.0;

        // Title
        GameText::title_centered("Typography Showcase", SCREEN_WIDTH / 2.0, y, fonts);
        y += 60.0;

        // Display styles column
        UIComponent::draw_text(
            "DISPLAY STYLES",
            x_left,
            y,
            TypographyStyle::BodyMedium,
            ColorTheme::Warning,
            fonts,
        );
        UIComponent::draw_text(
            "BODY STYLES",
            x_right,
            y,
            TypographyStyle::BodyMedium,
            ColorTheme::Warning,
            fonts,
        );
        y += 30.0;

        // Display Large
        UIComponent::draw_text(
            "Display Large",
            x_left,
            y,
            TypographyStyle::DisplayLarge,
            ColorTheme::Primary,
            fonts,
        );
        UIComponent::draw_text(
            "Body Large",
            x_right,
            y,
            TypographyStyle::BodyLarge,
            ColorTheme::Primary,
            fonts,
        );
        y += line_spacing;

        // Display Medium
        UIComponent::draw_text(
            "Display Medium",
            x_left,
            y,
            TypographyStyle::DisplayMedium,
            ColorTheme::Primary,
            fonts,
        );
        UIComponent::draw_text(
            "Body Medium",
            x_right,
            y,
            TypographyStyle::BodyMedium,
            ColorTheme::Primary,
            fonts,
        );
        y += line_spacing;

        // Display Small
        UIComponent::draw_text(
            "Display Small",
            x_left,
            y,
            TypographyStyle::DisplaySmall,
            ColorTheme::Primary,
            fonts,
        );
        UIComponent::draw_text(
            "Body Small",
            x_right,
            y,
            TypographyStyle::BodySmall,
            ColorTheme::Primary,
            fonts,
        );
        y += line_spacing + 20.0;

        // Technical styles section
        UIComponent::draw_text(
            "TECHNICAL STYLES (Monospace)",
            x_left,
            y,
            TypographyStyle::BodyMedium,
            ColorTheme::Warning,
            fonts,
        );
        UIComponent::draw_text(
            "UI STYLES",
            x_right,
            y,
            TypographyStyle::BodyMedium,
            ColorTheme::Warning,
            fonts,
        );
        y += 30.0;

        UIComponent::draw_text(
            "Code Large: git commit -m \"fix\"",
            x_left,
            y,
            TypographyStyle::CodeLarge,
            ColorTheme::Technical,
            fonts,
        );
        UIComponent::draw_text(
            "UI Button",
            x_right,
            y,
            TypographyStyle::UIButton,
            ColorTheme::Secondary,
            fonts,
        );
        y += line_spacing;

        UIComponent::draw_text(
            "Code Medium: npm run test",
            x_left,
            y,
            TypographyStyle::CodeMedium,
            ColorTheme::Technical,
            fonts,
        );
        UIComponent::draw_text(
            "UI Label",
            x_right,
            y,
            TypographyStyle::UILabel,
            ColorTheme::Secondary,
            fonts,
        );
        y += line_spacing;

        UIComponent::draw_text(
            "Code Small: > [SPACE] to jump",
            x_left,
            y,
            TypographyStyle::CodeSmall,
            ColorTheme::Technical,
            fonts,
        );
        UIComponent::draw_text(
            "UI Input Text",
            x_right,
            y,
            TypographyStyle::UIInput,
            ColorTheme::Secondary,
            fonts,
        );
        y += line_spacing;

        UIComponent::draw_text(
            "",
            x_left,
            y,
            TypographyStyle::CodeSmall,
            ColorTheme::Technical,
            fonts,
        );
        UIComponent::draw_text(
            "UI Caption",
            x_right,
            y,
            TypographyStyle::UICaption,
            ColorTheme::Neutral,
            fonts,
        );
    }

    fn draw_color_showcase(&self, fonts: &crate::design::GameFonts) {
        // Background
        draw_rectangle(
            0.0,
            0.0,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            secondary_palette::BACKGROUND,
        );

        let mut y = 80.0;
        let line_spacing = 35.0;

        // Title
        GameText::title_centered("Color Theme Showcase", SCREEN_WIDTH / 2.0, y, fonts);
        y += 60.0;

        let themes = [
            (ColorTheme::Primary, "Primary Theme"),
            (ColorTheme::Secondary, "Secondary Theme"),
            (ColorTheme::Success, "Success Theme"),
            (ColorTheme::Warning, "Warning Theme"),
            (ColorTheme::Error, "Error Theme"),
            (ColorTheme::Neutral, "Neutral Theme"),
            (ColorTheme::Technical, "Technical Theme"),
        ];

        for (theme, name) in themes.iter() {
            // Theme name
            UIComponent::draw_text(
                name,
                50.0,
                y,
                TypographyStyle::BodyMedium,
                ColorTheme::Warning,
                fonts,
            );

            // Sample text with theme
            let sample_text = format!("The quick brown fox jumps over the lazy dog - {}", name);
            UIComponent::draw_text(
                &sample_text,
                200.0,
                y,
                TypographyStyle::BodyMedium,
                *theme,
                fonts,
            );

            y += line_spacing;
        }

        y += 20.0;

        // Color palette section
        UIComponent::draw_text(
            "FLUREE BRAND COLORS",
            50.0,
            y,
            TypographyStyle::BodyMedium,
            ColorTheme::Warning,
            fonts,
        );
        y += 40.0;

        let color_samples = [
            (ICE_BLUE, "Ice Blue"),
            (VIBRANT_BLUE, "Vibrant Blue"),
            (DEEP, "Deep"),
            (PEAK, "Peak"),
            (PLUM, "Plum"),
            (crate::colors::PURPLE, "Purple"),
        ];

        let mut x = 50.0;
        for (color, name) in color_samples.iter() {
            // Color swatch
            draw_rectangle(x, y, 30.0, 30.0, *color);
            draw_rectangle_lines(x, y, 30.0, 30.0, 2.0, WHITE);

            // Color name
            UIComponent::draw_text(
                name,
                x,
                y + 45.0,
                TypographyStyle::BodySmall,
                ColorTheme::Primary,
                fonts,
            );

            x += 80.0;
        }
    }
}
