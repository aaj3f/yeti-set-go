use crate::design::GameFonts;
use macroquad::prelude::*;
use rust_embed::RustEmbed;
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct AssetFiles;

#[derive(RustEmbed)]
#[folder = "generated_assets/"]
struct GeneratedAssets;

pub struct GameAssets {
    pub textures: HashMap<String, Texture2D>,
    pub fonts: GameFonts,
}

impl GameAssets {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            fonts: GameFonts::new(),
        }
    }
}

pub async fn load_assets() -> GameAssets {
    let mut assets = GameAssets::new();
    assets.textures = load_textures().await;
    assets.fonts = load_fonts().await;
    assets
}

async fn load_fonts() -> GameFonts {
    let mut fonts = GameFonts::new();
    
    // Load primary font (Gotham-Medium)
    match AssetFiles::get("Gotham-Medium.otf") {
        Some(font_data) => {
            match load_ttf_font_from_bytes(&font_data.data) {
                Ok(font) => {
                    println!("Successfully loaded Gotham-Medium font");
                    fonts.primary = Some(font);
                }
                Err(e) => {
                    println!("Failed to load primary font: {}", e);
                }
            }
        }
        None => {
            println!("Gotham-Medium.otf not found in embedded assets");
        }
    }
    
    // For monospace, we'll use the default system monospace font
    // macroquad doesn't provide direct access to system fonts, so we'll use None
    // and the typography system will fall back to the default font for monospace content
    fonts.monospace = None;
    
    fonts
}

async fn load_textures() -> HashMap<String, Texture2D> {
    let texture_files = vec![
        ("yeti_run_1", "yeti_run_frame1_left_foot_forward_no_bg.png"),
        ("yeti_run_2", "yeti_run_frame3_both_feet_contact_no_bg.png"),
        ("yeti_jump", "yeti_jump_no_bg.png"),
        ("yeti_cheer", "yeti_cheer_no_bg.png"),
        ("yeti_stumble", "yeti_stumble_no_bg.png"),
        ("item_pr_merged", "item_pr_merged.png"),
        ("item_ci_pass", "item_ci_pass.png"),
        ("item_deploy_success", "item_deploy_success.png"),
        ("item_code_review", "item_code_review.png"),
        ("item_tests_pass", "item_tests_pass.png"),
        ("item_test_fail", "item_test_fail.png"),
        ("item_merge_conflict", "item_merge_conflict.png"),
        ("item_ci_fail", "item_ci_fail.png"),
        ("item_security_vuln", "item_security_vuln.png"),
        ("pipeline_track", "pipeline_track.png"),
        ("background", "background.png"),
        ("ui_frame", "ui_frame.png"),
    ];

    let mut textures = HashMap::new();

    for (name, filename) in texture_files {
        match GeneratedAssets::get(filename) {
            Some(texture_data) => {
                match image::load_from_memory(&texture_data.data) {
                    Ok(img) => {
                        let image_data = img.to_rgba8();
                        let width = img.width() as u16;
                        let height = img.height() as u16;
                        
                        let image = Image {
                            bytes: image_data.into_raw(),
                            width,
                            height,
                        };
                        
                        let texture = Texture2D::from_image(&image);
                        texture.set_filter(FilterMode::Nearest);
                        textures.insert(name.to_string(), texture);
                        println!("Successfully loaded texture: {}", filename);
                    }
                    Err(e) => {
                        println!("Failed to load texture {}: {}", filename, e);
                    }
                }
            }
            None => {
                println!("Texture file {} not found in embedded assets", filename);
            }
        }
    }

    textures
}