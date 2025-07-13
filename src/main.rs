use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::collections::HashMap;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 270.0;
const GROUND_Y: f32 = 210.0;

#[derive(Debug, Clone)]
struct Yeti {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    velocity_y: f32,
    is_jumping: bool,
    texture: Option<Texture2D>,
}

#[derive(Debug, Clone)]
struct Item {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_good: bool,
    texture: Option<Texture2D>,
    item_type: ItemType,
}

#[derive(Debug, Clone)]
enum ItemType {
    PrMerged,
    CiPass,
    DeploySuccess,
    CodeReview,
    TestsPass,
    TestFail,
    MergeConflict,
    CiFail,
    SecurityVuln,
}

#[derive(Debug)]
enum GameState {
    MainMenu,
    Playing,
    GameOver,
}

struct Game {
    yeti: Yeti,
    items: Vec<Item>,
    score: u32,
    level: u32,
    checks_completed: u32,
    checks_required: u32,
    spawn_timer: f32,
    spawn_rate: f32,
    textures: HashMap<String, Texture2D>,
    state: GameState,
    high_score: u32,
    pipeline_scroll: f32,
    collision_grace: f32,
    feedback_message: String,
    feedback_timer: f32,
    feedback_color: Color,
}

impl Game {
    fn new() -> Self {
        let yeti = Yeti {
            x: 100.0,
            y: GROUND_Y,
            width: 48.0,
            height: 48.0,
            velocity_y: 0.0,
            is_jumping: false,
            texture: None,
        };

        Self {
            yeti,
            items: Vec::new(),
            score: 0,
            level: 1,
            checks_completed: 0,
            checks_required: 5,
            spawn_timer: 0.0,
            spawn_rate: 2.0,
            textures: HashMap::new(),
            state: GameState::MainMenu,
            high_score: 0,
            pipeline_scroll: 0.0,
            collision_grace: 0.0,
            feedback_message: String::new(),
            feedback_timer: 0.0,
            feedback_color: WHITE,
        }
    }

    async fn load_textures(&mut self) {
        let texture_files = vec![
            (
                "yeti_run_1",
                "generated_assets/yeti_run_frame1_left_foot_forward_no_bg.png",
            ),
            (
                "yeti_run_2",
                "generated_assets/yeti_run_frame3_both_feet_contact_no_bg.png",
            ),
            ("yeti_jump", "generated_assets/yeti_jump_no_bg.png"),
            ("yeti_cheer", "generated_assets/yeti_cheer_no_bg.png"),
            ("yeti_stumble", "generated_assets/yeti_stumble_no_bg.png"),
            ("item_pr_merged", "generated_assets/item_pr_merged.png"),
            ("item_ci_pass", "generated_assets/item_ci_pass.png"),
            (
                "item_deploy_success",
                "generated_assets/item_deploy_success.png",
            ),
            ("item_code_review", "generated_assets/item_code_review.png"),
            ("item_tests_pass", "generated_assets/item_tests_pass.png"),
            ("item_test_fail", "generated_assets/item_test_fail.png"),
            (
                "item_merge_conflict",
                "generated_assets/item_merge_conflict.png",
            ),
            ("item_ci_fail", "generated_assets/item_ci_fail.png"),
            (
                "item_security_vuln",
                "generated_assets/item_security_vuln.png",
            ),
            ("pipeline_track", "generated_assets/pipeline_track.png"),
            ("background", "generated_assets/background.png"),
            ("ui_frame", "generated_assets/ui_frame.png"),
        ];

        for (name, path) in texture_files {
            match load_texture(path).await {
                Ok(texture) => {
                    // Ensure texture supports transparency properly
                    texture.set_filter(FilterMode::Nearest);
                    self.textures.insert(name.to_string(), texture);
                }
                Err(e) => {
                    println!("Failed to load texture {}: {}", path, e);
                }
            }
        }

        self.yeti.texture = self.textures.get("yeti_run_1").cloned();
    }

    fn update(&mut self, dt: f32) {
        match self.state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    self.start_game();
                }
            }
            GameState::Playing => {
                self.update_yeti(dt);
                self.update_items(dt);
                self.spawn_items(dt);
                self.check_collisions();
                self.check_level_completion();
                self.update_pipeline_animation(dt);
                self.update_collision_grace(dt);
                self.update_feedback_message(dt);
                self.update_next_item_feedback();
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    self.reset_game();
                }
            }
        }
    }

    fn update_yeti(&mut self, dt: f32) {
        if (is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left))
            && !self.yeti.is_jumping
        {
            self.yeti.velocity_y = -350.0;
            self.yeti.is_jumping = true;
        }

        if self.yeti.is_jumping {
            self.yeti.velocity_y += 800.0 * dt;
            self.yeti.y += self.yeti.velocity_y * dt;

            if self.yeti.y >= GROUND_Y {
                self.yeti.y = GROUND_Y;
                self.yeti.velocity_y = 0.0;
                self.yeti.is_jumping = false;
            }
        }

        if self.yeti.is_jumping {
            self.yeti.texture = self.textures.get("yeti_jump").cloned();
        } else {
            let run_frame = if (get_time() * 8.0) as i32 % 2 == 0 {
                "yeti_run_1"
            } else {
                "yeti_run_2"
            };
            self.yeti.texture = self.textures.get(run_frame).cloned();
        }
    }

    fn update_items(&mut self, dt: f32) {
        let speed = 200.0 + (self.level as f32 * 20.0);

        for item in &mut self.items {
            item.x -= speed * dt;
        }

        self.items.retain(|item| item.x > -item.width);
    }

    fn spawn_items(&mut self, dt: f32) {
        self.spawn_timer += dt;

        if self.spawn_timer >= self.spawn_rate {
            self.spawn_timer = 0.0;
            self.spawn_random_item();
        }
    }

    fn spawn_random_item(&mut self) {
        let mut rng = thread_rng();
        let is_good = rng.gen_bool(0.65);

        let item_type = if is_good {
            match rng.gen_range(0..5) {
                0 => ItemType::PrMerged,
                1 => ItemType::CiPass,
                2 => ItemType::DeploySuccess,
                3 => ItemType::CodeReview,
                _ => ItemType::TestsPass,
            }
        } else {
            match rng.gen_range(0..4) {
                0 => ItemType::TestFail,
                1 => ItemType::MergeConflict,
                2 => ItemType::CiFail,
                _ => ItemType::SecurityVuln,
            }
        };

        let texture_name = match item_type {
            ItemType::PrMerged => "item_pr_merged",
            ItemType::CiPass => "item_ci_pass",
            ItemType::DeploySuccess => "item_deploy_success",
            ItemType::CodeReview => "item_code_review",
            ItemType::TestsPass => "item_tests_pass",
            ItemType::TestFail => "item_test_fail",
            ItemType::MergeConflict => "item_merge_conflict",
            ItemType::CiFail => "item_ci_fail",
            ItemType::SecurityVuln => "item_security_vuln",
        };

        let item = Item {
            x: SCREEN_WIDTH,
            y: GROUND_Y,
            width: 32.0,
            height: 32.0,
            is_good,
            texture: self.textures.get(texture_name).cloned(),
            item_type,
        };

        self.items.push(item);
    }

    fn check_collisions(&mut self) {
        let mut items_to_remove = Vec::new();

        for (i, item) in self.items.iter().enumerate() {
            // Add collision grace - reduce effective collision box by 8 pixels on all sides
            let grace_margin = 8.0;
            let yeti_collision_x = self.yeti.x + grace_margin;
            let yeti_collision_y = self.yeti.y + grace_margin;
            let yeti_collision_width = self.yeti.width - (grace_margin * 2.0);
            let yeti_collision_height = self.yeti.height - (grace_margin * 2.0);

            let item_collision_x = item.x + grace_margin;
            let item_collision_y = item.y + grace_margin;
            let item_collision_width = item.width - (grace_margin * 2.0);
            let item_collision_height = item.height - (grace_margin * 2.0);

            if yeti_collision_x < item_collision_x + item_collision_width
                && yeti_collision_x + yeti_collision_width > item_collision_x
                && yeti_collision_y < item_collision_y + item_collision_height
                && yeti_collision_y + yeti_collision_height > item_collision_y
            {
                if item.is_good {
                    self.score += 10;
                    self.checks_completed += 1;
                } else {
                    // Set collision grace period for visual feedback
                    self.collision_grace = 0.5;
                    self.state = GameState::GameOver;
                    if self.score > self.high_score {
                        self.high_score = self.score;
                    }
                }

                items_to_remove.push(i);
            }
        }

        for &i in items_to_remove.iter().rev() {
            self.items.remove(i);
        }
    }

    fn check_level_completion(&mut self) {
        if self.checks_completed >= self.checks_required {
            self.level += 1;
            self.checks_completed = 0;
            self.checks_required = 5 + (self.level - 1) * 3;
            self.spawn_rate = (2.0 - (self.level as f32 * 0.1)).max(0.5);
        }
    }

    fn start_game(&mut self) {
        self.yeti.x = 100.0;
        self.yeti.y = GROUND_Y;
        self.yeti.velocity_y = 0.0;
        self.yeti.is_jumping = false;
        self.items.clear();
        self.score = 0;
        self.level = 1;
        self.checks_completed = 0;
        self.checks_required = 5;
        self.spawn_timer = 0.0;
        self.spawn_rate = 2.0;
        self.pipeline_scroll = 0.0;
        self.collision_grace = 0.0;
        self.feedback_message = String::new();
        self.feedback_timer = 0.0;
        self.feedback_color = WHITE;
        self.state = GameState::Playing;
    }

    fn reset_game(&mut self) {
        self.state = GameState::MainMenu;
    }

    fn update_pipeline_animation(&mut self, dt: f32) {
        // Scroll pipeline based on level speed
        let speed = 100.0 + (self.level as f32 * 10.0);
        self.pipeline_scroll += speed * dt;

        // Reset scroll when it goes beyond track width to create seamless loop
        if self.pipeline_scroll >= 128.0 {
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
        // Find the next item that the yeti hasn't passed yet
        let next_item = self
            .items
            .iter()
            .filter(|item| item.x + item.width > self.yeti.x) // Item hasn't been passed by yeti
            .filter(|item| item.x < SCREEN_WIDTH) // Item is on screen
            .min_by(|a, b| a.x.partial_cmp(&b.x).unwrap()); // Get the closest one

        if let Some(item) = next_item {
            // Only update if this is a different item than what we're currently showing
            let new_message = self.get_feedback_text(&item.item_type);
            if self.feedback_message != new_message {
                self.feedback_message = new_message;
                self.feedback_timer = 10.0; // Show for a long time since we update it manually
                self.feedback_color = BLACK;
            }
        } else {
            // No next item, clear the message
            self.feedback_message.clear();
        }
    }

    fn get_feedback_text(&self, item_type: &ItemType) -> String {
        let item_text = match item_type {
            ItemType::PrMerged => "Someone finally approved my PR! Let's merge it!",
            ItemType::CiPass => "Phew! The CI pipeline checks all passed!",
            ItemType::DeploySuccess => "Deployment succeeded--my code is live!",
            ItemType::CodeReview => "Their code looks great! Let's approve it!",
            ItemType::TestsPass => "Thank god! All the tests are finally passing!",
            ItemType::TestFail => "Ah, shark farts... some tests are failing...",
            ItemType::MergeConflict => "Of course there's a merge conflict...",
            ItemType::CiFail => "Wait what? The CI pipeline failed? Why??",
            ItemType::SecurityVuln => "Um... do I have to worry about this security vulnerability?",
        };

        item_text.to_string()
    }

    fn draw(&self) {
        // Ensure proper alpha blending for transparency
        gl_use_default_material();
        // Draw background
        if let Some(bg_texture) = self.textures.get("background") {
            // Scale background to fit screen while maintaining aspect ratio
            let scale_x = SCREEN_WIDTH / bg_texture.width();
            let scale_y = SCREEN_HEIGHT / bg_texture.height();
            let scale = scale_x.min(scale_y); // Use uniform scaling to maintain aspect ratio

            let scaled_width = bg_texture.width() * scale;
            let scaled_height = bg_texture.height() * scale;

            // Center the background if it doesn't fill the entire screen
            let offset_x = (SCREEN_WIDTH - scaled_width) / 2.0;
            let offset_y = (SCREEN_HEIGHT - scaled_height) / 2.0;

            draw_texture_ex(
                bg_texture,
                offset_x,
                offset_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(scaled_width, scaled_height)),
                    ..Default::default()
                },
            );

            // Fill any remaining space with a background color
            if scaled_width < SCREEN_WIDTH || scaled_height < SCREEN_HEIGHT {
                clear_background(Color::new(0.8, 0.9, 1.0, 1.0));
            }
        } else {
            clear_background(Color::new(0.8, 0.9, 1.0, 1.0));
        }

        // Draw animated pipeline track
        if let Some(pipeline_texture) = self.textures.get("pipeline_track") {
            let track_y = GROUND_Y + 20.0;
            let track_width = pipeline_texture.width();

            // Draw multiple copies of the track to fill the screen width
            let num_tracks = ((SCREEN_WIDTH / track_width) as i32) + 2;
            for i in 0..num_tracks {
                let x = (i as f32 * track_width) - self.pipeline_scroll;
                draw_texture(pipeline_texture, x, track_y, WHITE);
            }
        } else {
            // Fallback pipeline line
            draw_line(
                0.0,
                GROUND_Y + 48.0,
                SCREEN_WIDTH,
                GROUND_Y + 48.0,
                4.0,
                DARKGRAY,
            );
        }

        // Only draw game objects when playing
        if matches!(self.state, GameState::Playing) {
            // Draw yeti with potential collision grace visual effect
            let yeti_tint = if self.collision_grace > 0.0 {
                Color::new(1.0, 0.5, 0.5, 0.8) // Red tint when hit
            } else {
                WHITE
            };

            if let Some(texture) = &self.yeti.texture {
                // Draw texture with proper alpha blending for transparency
                draw_texture_ex(
                    texture,
                    self.yeti.x,
                    self.yeti.y - self.yeti.height,
                    yeti_tint,
                    DrawTextureParams {
                        dest_size: None,
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    },
                );
            } else {
                draw_rectangle(
                    self.yeti.x,
                    self.yeti.y - self.yeti.height,
                    self.yeti.width,
                    self.yeti.height,
                    BLUE,
                );
            }
        }

        // Only draw items when playing
        if matches!(self.state, GameState::Playing) {
            for item in &self.items {
                if let Some(texture) = &item.texture {
                    // Draw texture with proper alpha blending for transparency
                    draw_texture_ex(
                        texture,
                        item.x,
                        item.y - item.height,
                        WHITE,
                        DrawTextureParams {
                            dest_size: None,
                            source: None,
                            rotation: 0.0,
                            flip_x: false,
                            flip_y: false,
                            pivot: None,
                        },
                    );
                } else {
                    let color = if item.is_good { GREEN } else { RED };
                    draw_rectangle(item.x, item.y - item.height, item.width, item.height, color);
                }
            }
        }

        // Only draw game UI when playing
        if matches!(self.state, GameState::Playing) {
            // Draw UI frame for progress counter if available
            // if let Some(ui_frame) = self.textures.get("ui_frame") {
            //     draw_texture_ex(
            //         ui_frame,
            //         10.0,
            //         10.0,
            //         WHITE,
            //         DrawTextureParams {
            //             dest_size: Some(Vec2::new(200.0, 40.0)),
            //             ..Default::default()
            //         },
            //     );
            // }

            let progress_text = format!(
                "{} / {} Passing Checks",
                self.checks_completed, self.checks_required
            );
            draw_text(&progress_text, 15.0, 30.0, 20.0, BLACK);

            let level_text = format!("Level: {}", self.level);
            draw_text(&level_text, 15.0, 60.0, 18.0, BLACK);

            let score_text = format!("Score: {}", self.score);
            draw_text(&score_text, 15.0, 80.0, 18.0, BLACK);

            // Draw feedback message with GameBoy-style border
            if !self.feedback_message.is_empty() && self.feedback_timer > 0.0 {
                // Fixed-size message box (GameBoy style)
                let box_width = 300.0;
                let box_height = 60.0;
                let box_x = SCREEN_WIDTH - box_width - 20.0;
                let box_y = 20.0;

                // Draw black border
                draw_rectangle(
                    box_x - 3.0,
                    box_y - 3.0,
                    box_width + 6.0,
                    box_height + 6.0,
                    BLACK,
                );

                // Draw white background
                draw_rectangle(box_x, box_y, box_width, box_height, WHITE);

                // Draw text left-aligned with wrapping
                let text_x = box_x + 10.0;
                let text_y = box_y + 20.0;
                let line_height = 16.0;

                // Simple word wrapping
                let words: Vec<&str> = self.feedback_message.split_whitespace().collect();
                let mut current_line = String::new();
                let mut y_offset = 0.0;

                for word in words {
                    let test_line = if current_line.is_empty() {
                        word.to_string()
                    } else {
                        format!("{} {}", current_line, word)
                    };

                    let test_width = measure_text(&test_line, None, 20, 1.0).width;

                    if test_width <= box_width - 20.0 {
                        current_line = test_line;
                    } else {
                        // Draw current line and start new one
                        if !current_line.is_empty() {
                            draw_text(&current_line, text_x, text_y + y_offset, 20.0, BLACK);
                            y_offset += line_height;
                        }
                        current_line = word.to_string();
                    }
                }

                // Draw the last line
                if !current_line.is_empty() {
                    draw_text(&current_line, text_x, text_y + y_offset, 20.0, BLACK);
                }
            }
        }

        match self.state {
            GameState::MainMenu => {
                // Draw semi-transparent overlay
                draw_rectangle(
                    0.0,
                    0.0,
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                    Color::new(0.0, 0.0, 0.0, 0.7),
                );

                // Game title
                let title_text = "Yeti, Set, Go!";
                let title_size = measure_text(title_text, None, 35, 1.0);
                draw_text(
                    title_text,
                    SCREEN_WIDTH / 2.0 - title_size.width / 2.0,
                    SCREEN_HEIGHT / 2.0 - 100.0,
                    35.0,
                    WHITE,
                );

                // Instructions
                let instructions = vec![
                    "A CI/CD Pipeline Runner for Impatient Devs",
                    "",
                    "• SPACE or Click to Jump",
                    "• Collect GOOD statuses // Avoid BAD ones",
                    "• Complete checks to advance levels",
                    "",
                    "Press SPACE to Start",
                ];

                let mut y_offset = SCREEN_HEIGHT / 2.0 - 50.0;
                for instruction in instructions {
                    if !instruction.is_empty() {
                        let text_size = measure_text(instruction, None, 18, 1.0);
                        draw_text(
                            instruction,
                            SCREEN_WIDTH / 2.0 - text_size.width / 2.0,
                            y_offset,
                            18.0,
                            WHITE,
                        );
                    }
                    y_offset += 22.0;
                }

                // High score
                if self.high_score > 0 {
                    let high_score_text = format!("High Score: {}", self.high_score);
                    let score_size = measure_text(&high_score_text, None, 16, 1.0);
                    draw_text(
                        &high_score_text,
                        SCREEN_WIDTH / 2.0 - score_size.width / 2.0,
                        SCREEN_HEIGHT - 30.0,
                        16.0,
                        YELLOW,
                    );
                }
            }
            GameState::GameOver => {
                draw_rectangle(
                    0.0,
                    0.0,
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                    Color::new(0.0, 0.0, 0.0, 0.5),
                );

                let game_over_text = "GAME OVER!";
                let text_size = measure_text(game_over_text, None, 48, 1.0);
                draw_text(
                    game_over_text,
                    SCREEN_WIDTH / 2.0 - text_size.width / 2.0,
                    SCREEN_HEIGHT / 2.0 - 70.0,
                    48.0,
                    WHITE,
                );

                let final_score_text = format!("Final Score: {}", self.score);
                let final_size = measure_text(&final_score_text, None, 24, 1.0);
                draw_text(
                    &final_score_text,
                    SCREEN_WIDTH / 2.0 - final_size.width / 2.0,
                    SCREEN_HEIGHT / 2.0 - 20.0,
                    24.0,
                    WHITE,
                );

                if self.score == self.high_score && self.high_score > 0 {
                    let new_record_text = "NEW HIGH SCORE!";
                    let record_size = measure_text(new_record_text, None, 20, 1.0);
                    draw_text(
                        new_record_text,
                        SCREEN_WIDTH / 2.0 - record_size.width / 2.0,
                        SCREEN_HEIGHT / 2.0 + 10.0,
                        20.0,
                        YELLOW,
                    );
                }

                let restart_text = "Press SPACE to return to menu";
                let restart_size = measure_text(restart_text, None, 18, 1.0);
                draw_text(
                    restart_text,
                    SCREEN_WIDTH / 2.0 - restart_size.width / 2.0,
                    SCREEN_HEIGHT / 2.0 + 50.0,
                    18.0,
                    WHITE,
                );
            }
            GameState::Playing => {
                let instructions = "SPACE or Click to Jump | Collect Good Items | Avoid Bad Items";
                draw_text(instructions, 10.0, SCREEN_HEIGHT - 20.0, 14.0, DARKGRAY);
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Yeti, Set, Go!".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    game.load_textures().await;

    loop {
        let dt = get_frame_time();
        game.update(dt);
        game.draw();
        next_frame().await;
    }
}
