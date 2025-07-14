use super::{physics, scoring, spawning};
use crate::api::{ApiClient, load_leaderboard_with_fallback, submit_score_with_fallback};
use crate::colors::*;
use crate::config::*;
use crate::design::GameFonts;
use crate::entities::{Item, Yeti};
use crate::highscores::{HighScore, Leaderboard};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::sync::mpsc;

#[derive(Debug)]
pub enum GameState {
    MainMenu,
    Playing,
    LevelComplete,
    GameOver,
    NameInput,
    ViewingLeaderboard,
}

#[derive(Debug)]
pub enum ApiMessage {
    LeaderboardSynced(Leaderboard),
    ScoreSubmitted(bool), // success flag
}

pub struct Game {
    pub yeti: Yeti,
    pub items: Vec<Item>,
    pub score: u32,
    pub level: u32,
    pub checks_completed: u32,
    pub checks_required: u32,
    pub spawn_timer: f32,
    pub spawn_rate: f32,
    pub textures: HashMap<String, Texture2D>,
    pub fonts: GameFonts,
    pub state: GameState,
    pub leaderboard: Leaderboard,
    pub api_client: ApiClient,
    pub pipeline_scroll: f32,
    pub collision_grace: f32,
    pub feedback_message: String,
    pub feedback_timer: f32,
    pub feedback_color: Color,
    pub level_complete_timer: f32,
    pub level_complete_message: String,
    pub level_complete_submessage: String,
    pub player_name_input: String,
    pub is_new_high_score: bool,
    pub leaderboard_scroll: f32,
    pub menu_time: f32,
    pub mini_leaderboard_scroll: f32,
    pub api_loading: bool,
    pub last_api_sync: f32, // Time since last sync attempt
    api_receiver: mpsc::Receiver<ApiMessage>,
    api_sender: mpsc::Sender<ApiMessage>,
}

impl Game {
    pub fn new() -> Self {
        let (api_sender, api_receiver) = mpsc::channel();
        
        let mut game = Self {
            yeti: Yeti::new(),
            items: Vec::new(),
            score: 0,
            level: 1,
            checks_completed: 0,
            checks_required: 5,
            spawn_timer: 0.0,
            spawn_rate: INITIAL_SPAWN_RATE,
            textures: HashMap::new(),
            fonts: GameFonts::new(),
            state: GameState::MainMenu,
            leaderboard: Leaderboard::new(),
            api_client: ApiClient::new(),
            pipeline_scroll: 0.0,
            collision_grace: 0.0,
            feedback_message: String::new(),
            feedback_timer: 0.0,
            feedback_color: TEXT_PRIMARY,
            level_complete_timer: 0.0,
            level_complete_message: String::new(),
            level_complete_submessage: String::new(),
            player_name_input: String::new(),
            is_new_high_score: false,
            leaderboard_scroll: 0.0,
            menu_time: 0.0,
            mini_leaderboard_scroll: 0.0,
            api_loading: false,
            last_api_sync: 0.0,
            api_receiver,
            api_sender,
        };
        
        // Trigger initial leaderboard sync on startup
        game.sync_leaderboard_with_api();
        
        game
    }

    pub fn update(&mut self, dt: f32) {
        // Process any pending API messages
        self.process_api_messages();
        
        match self.state {
            GameState::MainMenu => {
                self.menu_time += dt;
                self.last_api_sync += dt;
                self.update_mini_leaderboard_scroll(dt);
                
                // Sync with API every 30 seconds when on main menu
                if self.last_api_sync > 30.0 && !self.api_loading {
                    self.sync_leaderboard_with_api();
                }

                if is_key_pressed(KeyCode::Space) {
                    self.start_game();
                } else if is_key_pressed(KeyCode::L) {
                    self.state = GameState::ViewingLeaderboard;
                }
            }
            GameState::Playing => {
                self.update_yeti(dt);
                self.update_items(dt);
                scoring::update_item_scoring(self, dt);
                spawning::spawn_items(self, dt);
                physics::check_collisions(self);
                self.check_level_completion();
                self.update_pipeline_animation(dt);
                self.update_collision_grace(dt);
                self.update_feedback_message(dt);
                self.update_next_item_feedback();
            }
            GameState::LevelComplete => {
                self.level_complete_timer -= dt;
                if self.level_complete_timer <= 0.0 {
                    self.state = GameState::Playing;
                }
            }
            GameState::GameOver => {
                if self.is_new_high_score && is_key_pressed(KeyCode::Space) {
                    self.state = GameState::NameInput;
                } else if is_key_pressed(KeyCode::Space) {
                    self.reset_game();
                } else if is_key_pressed(KeyCode::L) {
                    self.state = GameState::ViewingLeaderboard;
                }
            }
            GameState::NameInput => {
                self.handle_name_input();
            }
            GameState::ViewingLeaderboard => {
                if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Space) {
                    self.state = GameState::MainMenu;
                }
                self.handle_leaderboard_scroll(dt);
            }
        }
    }

    fn update_yeti(&mut self, dt: f32) {
        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
            self.yeti.jump();
        }

        self.yeti.update(dt);
        self.yeti.update_texture(&self.textures);
    }

    fn update_items(&mut self, dt: f32) {
        for item in &mut self.items {
            item.update(dt, self.level);
        }

        self.items.retain(|item| !item.is_off_screen());
    }

    fn check_level_completion(&mut self) {
        if self.checks_completed >= self.checks_required {
            // Award level completion bonus
            self.score += scoring::calculate_level_score_bonus(self.level);

            self.level += 1;
            self.checks_completed = 0;
            self.checks_required = 5 + (self.level - 1) * 3;
            self.spawn_rate = (INITIAL_SPAWN_RATE - (self.level as f32 * 0.1)).max(MIN_SPAWN_RATE);

            // Show level complete message
            self.level_complete_message = format!("Issue #{} Done!", self.level - 1);
            self.level_complete_submessage = "What else is assigned to me...".to_string();

            self.level_complete_timer = 2.5; // Show for 2.5 seconds
            self.state = GameState::LevelComplete;
        }
    }

    pub fn start_game(&mut self) {
        self.yeti.reset();
        self.items.clear();
        self.score = 0;
        self.level = 1;
        self.checks_completed = 0;
        self.checks_required = 5;
        self.spawn_timer = 0.0;
        self.spawn_rate = INITIAL_SPAWN_RATE;
        self.pipeline_scroll = 0.0;
        self.collision_grace = 0.0;
        self.feedback_message = String::new();
        self.feedback_timer = 0.0;
        self.feedback_color = TEXT_PRIMARY;
        self.level_complete_timer = 0.0;
        self.level_complete_message = String::new();
        self.is_new_high_score = false;
        self.state = GameState::Playing;
    }

    pub fn reset_game(&mut self) {
        self.menu_time = 0.0;
        self.mini_leaderboard_scroll = 0.0;
        self.state = GameState::MainMenu;
    }

    fn update_pipeline_animation(&mut self, dt: f32) {
        let speed = PIPELINE_BASE_SPEED + (self.level as f32 * PIPELINE_SPEED_INCREASE);
        self.pipeline_scroll += speed * dt;

        if self.pipeline_scroll >= PIPELINE_SCROLL_RESET {
            self.pipeline_scroll = 0.0;
        }
    }

    fn update_collision_grace(&mut self, dt: f32) {
        if self.collision_grace > 0.0 {
            self.collision_grace -= dt;
        }
    }

    fn update_feedback_message(&mut self, dt: f32) {
        if self.feedback_timer > 0.0 {
            self.feedback_timer -= dt;
            if self.feedback_timer <= 0.0 {
                self.feedback_message.clear();
            }
        }
    }

    fn update_next_item_feedback(&mut self) {
        let next_item = self
            .items
            .iter()
            .filter(|item| item.x + item.width > self.yeti.x)
            .filter(|item| item.x < SCREEN_WIDTH)
            .min_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        if let Some(item) = next_item {
            let new_message = item.item_type.get_feedback_text().to_string();
            if self.feedback_message != new_message {
                self.feedback_message = new_message;
                self.feedback_timer = FEEDBACK_DISPLAY_TIME;
                self.feedback_color = TEXT_PRIMARY;
            }
        } else {
            self.feedback_message.clear();
        }
    }

    pub fn game_over(&mut self) {
        self.collision_grace = COLLISION_GRACE_TIME;

        // Calculate final score with bonuses
        let final_score = scoring::calculate_total_score_with_bonuses(
            self.score,
            self.level,
            self.checks_completed,
        );
        self.score = final_score;

        // Check if this is a new high score
        self.is_new_high_score = self.leaderboard.is_high_score(self.score);

        self.state = GameState::GameOver;
    }

    fn handle_name_input(&mut self) {
        // Handle character input for name
        if let Some(character) = get_last_key_pressed() {
            match character {
                KeyCode::Enter => {
                    if !self.player_name_input.trim().is_empty() {
                        self.submit_high_score();
                    }
                }
                KeyCode::Backspace => {
                    self.player_name_input.pop();
                }
                KeyCode::Escape => {
                    self.reset_game();
                }
                _ => {
                    // Convert keycode to character if possible
                    if let Some(ch) = self.keycode_to_char(character) {
                        if self.player_name_input.len() < 20 {
                            // Limit name length
                            self.player_name_input.push(ch);
                        }
                    }
                }
            }
        }
    }

    fn keycode_to_char(&self, keycode: KeyCode) -> Option<char> {
        match keycode {
            KeyCode::A => Some('A'),
            KeyCode::B => Some('B'),
            KeyCode::C => Some('C'),
            KeyCode::D => Some('D'),
            KeyCode::E => Some('E'),
            KeyCode::F => Some('F'),
            KeyCode::G => Some('G'),
            KeyCode::H => Some('H'),
            KeyCode::I => Some('I'),
            KeyCode::J => Some('J'),
            KeyCode::K => Some('K'),
            KeyCode::L => Some('L'),
            KeyCode::M => Some('M'),
            KeyCode::N => Some('N'),
            KeyCode::O => Some('O'),
            KeyCode::P => Some('P'),
            KeyCode::Q => Some('Q'),
            KeyCode::R => Some('R'),
            KeyCode::S => Some('S'),
            KeyCode::T => Some('T'),
            KeyCode::U => Some('U'),
            KeyCode::V => Some('V'),
            KeyCode::W => Some('W'),
            KeyCode::X => Some('X'),
            KeyCode::Y => Some('Y'),
            KeyCode::Z => Some('Z'),
            KeyCode::Space => Some(' '),
            _ => None,
        }
    }

    fn submit_high_score(&mut self) {
        let high_score = HighScore::new(
            self.player_name_input.trim().to_string(),
            self.score,
            self.level,
        );

        // Submit to API with local fallback
        self.submit_score_to_api(high_score);

        self.player_name_input.clear();
        self.reset_game();
    }

    fn handle_leaderboard_scroll(&mut self, dt: f32) {
        // Simple scroll handling - could be enhanced with mouse wheel support
        if is_key_down(KeyCode::Up) {
            self.leaderboard_scroll -= 100.0 * dt;
        }
        if is_key_down(KeyCode::Down) {
            self.leaderboard_scroll += 100.0 * dt;
        }

        // Clamp scroll to reasonable bounds
        self.leaderboard_scroll = self.leaderboard_scroll.clamp(0.0, 400.0);
    }

    fn update_mini_leaderboard_scroll(&mut self, dt: f32) {
        // Only scroll if we have more than 3 scores and have been on menu for 3+ seconds
        if self.leaderboard.scores.len() > 3 && self.menu_time > 3.0 {
            // Slow, smooth scroll
            self.mini_leaderboard_scroll += 15.0 * dt;

            // Reset scroll when we've scrolled through all extra entries
            let max_scroll = (self.leaderboard.scores.len() - 3) as f32 * 20.0;
            if self.mini_leaderboard_scroll > max_scroll + 60.0 {
                self.mini_leaderboard_scroll = 0.0;
            }
        }
    }

    // Process messages from async API tasks
    fn process_api_messages(&mut self) {
        while let Ok(message) = self.api_receiver.try_recv() {
            match message {
                ApiMessage::LeaderboardSynced(updated_leaderboard) => {
                    self.leaderboard = updated_leaderboard;
                    self.api_loading = false;
                    println!("Leaderboard synced successfully from API");
                }
                ApiMessage::ScoreSubmitted(success) => {
                    if success {
                        println!("Score submitted successfully to API");
                    } else {
                        println!("Score submission failed, using local fallback");
                    }
                }
            }
        }
    }

    // API sync methods
    pub fn sync_leaderboard_with_api(&mut self) {
        if self.api_loading {
            return;
        }
        
        self.api_loading = true;
        self.last_api_sync = 0.0;
        
        let api_client = self.api_client.clone();
        let sender = self.api_sender.clone();
        let mut leaderboard = self.leaderboard.clone();
        
        // Spawn background thread with its own Tokio runtime
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let success = load_leaderboard_with_fallback(&api_client, &mut leaderboard).await;
                
                if success {
                    // Send updated leaderboard back to main thread
                    let _ = sender.send(ApiMessage::LeaderboardSynced(leaderboard));
                } else {
                    // Signal that loading is complete even if failed
                    let _ = sender.send(ApiMessage::LeaderboardSynced(leaderboard));
                }
            });
        });
    }
    
    pub fn submit_score_to_api(&mut self, high_score: HighScore) {
        // Add to local leaderboard immediately for responsive UI
        self.leaderboard.add_score(high_score.clone());
        
        let api_client = self.api_client.clone();
        let sender = self.api_sender.clone();
        let mut leaderboard = self.leaderboard.clone();
        
        // Spawn background thread with its own Tokio runtime
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let success = submit_score_with_fallback(&api_client, &high_score, &mut leaderboard).await;
                
                // Send result back to main thread
                let _ = sender.send(ApiMessage::ScoreSubmitted(success));
            });
        });
    }
}
