use macroquad::prelude::*;
use crate::config::*;

#[derive(Debug, Clone)]
pub struct Yeti {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub velocity_y: f32,
    pub is_jumping: bool,
    pub texture: Option<Texture2D>,
}

impl Yeti {
    pub fn new() -> Self {
        Self {
            x: 100.0,
            y: GROUND_Y,
            width: YETI_WIDTH,
            height: YETI_HEIGHT,
            velocity_y: 0.0,
            is_jumping: false,
            texture: None,
        }
    }

    pub fn reset(&mut self) {
        self.x = 100.0;
        self.y = GROUND_Y;
        self.velocity_y = 0.0;
        self.is_jumping = false;
    }

    pub fn jump(&mut self) {
        if !self.is_jumping {
            self.velocity_y = JUMP_VELOCITY;
            self.is_jumping = true;
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_jumping {
            self.velocity_y += GRAVITY * dt;
            self.y += self.velocity_y * dt;

            if self.y >= GROUND_Y {
                self.y = GROUND_Y;
                self.velocity_y = 0.0;
                self.is_jumping = false;
            }
        }
    }

    pub fn update_texture(&mut self, textures: &std::collections::HashMap<String, Texture2D>) {
        if self.is_jumping {
            self.texture = textures.get("yeti_jump").cloned();
        } else {
            let run_frame = if (get_time() * 8.0) as i32 % 2 == 0 {
                "yeti_run_1"
            } else {
                "yeti_run_2"
            };
            self.texture = textures.get(run_frame).cloned();
        }
    }

    pub fn get_collision_rect(&self) -> (f32, f32, f32, f32) {
        let margin = COLLISION_GRACE_MARGIN;
        (
            self.x + margin,
            self.y + margin,
            self.width - (margin * 2.0),
            self.height - (margin * 2.0),
        )
    }
}