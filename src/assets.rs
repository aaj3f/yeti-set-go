use macroquad::prelude::*;
use std::collections::HashMap;
use crate::design::GameFonts;

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
    match load_ttf_font("assets/Gotham-Medium.otf").await {
        Ok(font) => {
            println!("Successfully loaded Gotham-Medium font");
            fonts.primary = Some(font);
        }
        Err(e) => {
            println!("Failed to load primary font: {}", e);
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
        ("yeti_run_1", "generated_assets/yeti_run_frame1_left_foot_forward_no_bg.png"),
        ("yeti_run_2", "generated_assets/yeti_run_frame3_both_feet_contact_no_bg.png"),
        ("yeti_jump", "generated_assets/yeti_jump_no_bg.png"),
        ("yeti_cheer", "generated_assets/yeti_cheer_no_bg.png"),
        ("yeti_stumble", "generated_assets/yeti_stumble_no_bg.png"),
        ("item_pr_merged", "generated_assets/item_pr_merged.png"),
        ("item_ci_pass", "generated_assets/item_ci_pass.png"),
        ("item_deploy_success", "generated_assets/item_deploy_success.png"),
        ("item_code_review", "generated_assets/item_code_review.png"),
        ("item_tests_pass", "generated_assets/item_tests_pass.png"),
        ("item_test_fail", "generated_assets/item_test_fail.png"),
        ("item_merge_conflict", "generated_assets/item_merge_conflict.png"),
        ("item_ci_fail", "generated_assets/item_ci_fail.png"),
        ("item_security_vuln", "generated_assets/item_security_vuln.png"),
        ("pipeline_track", "generated_assets/pipeline_track.png"),
        ("background", "generated_assets/background.png"),
        ("ui_frame", "generated_assets/ui_frame.png"),
    ];

    let mut textures = HashMap::new();

    for (name, path) in texture_files {
        match load_texture(path).await {
            Ok(texture) => {
                texture.set_filter(FilterMode::Nearest);
                textures.insert(name.to_string(), texture);
            }
            Err(e) => {
                println!("Failed to load texture {}: {}", path, e);
            }
        }
    }

    textures
}