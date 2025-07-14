use super::{hud, leaderboard, level_complete, menu, name_input};
use crate::colors::*;
use crate::config::*;
use crate::game::{Game, GameState};
use macroquad::prelude::*;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&self, game: &Game) {
        gl_use_default_material();
        self.draw_background(game);
        self.draw_pipeline(game);

        if matches!(game.state, GameState::Playing) {
            self.draw_game_objects(game);
            hud::draw_game_ui(game);
        }

        match game.state {
            GameState::MainMenu => menu::draw_main_menu(game),
            GameState::GameOver => menu::draw_game_over(game),
            GameState::Playing => {
                // hud::draw_instructions(game);
            }
            GameState::LevelComplete => level_complete::draw_level_complete(game),
            GameState::NameInput => name_input::draw_name_input(game),
            GameState::ViewingLeaderboard => leaderboard::draw_leaderboard_view(game),
        }
    }

    fn draw_background(&self, game: &Game) {
        if let Some(bg_texture) = game.textures.get("background") {
            let scale_x = SCREEN_WIDTH / bg_texture.width();
            let scale_y = SCREEN_HEIGHT / bg_texture.height();
            let scale = scale_x.min(scale_y);

            let scaled_width = bg_texture.width() * scale;
            let scaled_height = bg_texture.height() * scale;

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

            if scaled_width < SCREEN_WIDTH || scaled_height < SCREEN_HEIGHT {
                clear_background(ICE_BLUE);
            }
        } else {
            clear_background(ICE_BLUE);
        }
    }

    fn draw_pipeline(&self, game: &Game) {
        if let Some(pipeline_texture) = game.textures.get("pipeline_track") {
            let track_y = GROUND_Y + 20.0;
            let track_width = pipeline_texture.width();

            let num_tracks = ((SCREEN_WIDTH / track_width) as i32) + 2;
            for i in 0..num_tracks {
                let x = (i as f32 * track_width) - game.pipeline_scroll;
                draw_texture(pipeline_texture, x, track_y, WHITE);
            }
        } else {
            draw_line(
                0.0,
                GROUND_Y + 48.0,
                SCREEN_WIDTH,
                GROUND_Y + 48.0,
                4.0,
                METAL,
            );
        }
    }

    fn draw_game_objects(&self, game: &Game) {
        self.draw_yeti(game);
        self.draw_items(game);
    }

    fn draw_yeti(&self, game: &Game) {
        let yeti_tint = if game.collision_grace > 0.0 {
            EMBER
        } else {
            WHITE
        };

        if let Some(texture) = &game.yeti.texture {
            draw_texture_ex(
                texture,
                game.yeti.x,
                game.yeti.y - game.yeti.height,
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
                game.yeti.x,
                game.yeti.y - game.yeti.height,
                game.yeti.width,
                game.yeti.height,
                VIBRANT_BLUE,
            );
        }
    }

    fn draw_items(&self, game: &Game) {
        for item in &game.items {
            if let Some(texture) = &item.texture {
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
                let color = if item.is_good {
                    SUCCESS_GREEN
                } else {
                    ERROR_RED
                };
                draw_rectangle(item.x, item.y - item.height, item.width, item.height, color);
            }
        }
    }
}
