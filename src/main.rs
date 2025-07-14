mod api;
mod assets;
mod colors;
mod config;
mod design;
mod dev_mode;
mod entities;
mod game;
mod highscores;
mod ui;

use assets::load_assets;
use config::window_conf;
use dev_mode::DevMode;
use game::Game;
use macroquad::prelude::*;
use ui::Renderer;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    let assets = load_assets().await;
    game.textures = assets.textures;
    game.fonts = assets.fonts;

    let mut dev_mode = DevMode::new();
    dev_mode.mock_game.textures = game.textures.clone();
    dev_mode.mock_game.fonts = game.fonts.clone();
    
    let renderer = Renderer::new();

    loop {
        let dt = get_frame_time();
        
        // Check for dev mode toggle (D key) - only if dev mode is enabled in config
        if config::DEV_MODE_ENABLED && is_key_pressed(KeyCode::D) {
            dev_mode.toggle();
        }
        
        if dev_mode.enabled {
            // Handle dev mode input
            dev_mode.handle_input();
            
            // Override game state for dev mode
            dev_mode.mock_game.state = dev_mode.get_current_game_state();
            
            // Draw the mock game or custom screens
            if matches!(dev_mode.current_screen, dev_mode::DevScreen::TypographyShowcase | dev_mode::DevScreen::ColorShowcase) {
                dev_mode.draw_custom_screen(&game.fonts);
            } else {
                renderer.draw(&dev_mode.mock_game);
            }
            
            // Draw dev mode overlay
            dev_mode.draw_dev_overlay(&game.fonts);
        } else {
            // Normal game loop
            game.update(dt);
            renderer.draw(&game);
        }
        
        next_frame().await;
    }
}
