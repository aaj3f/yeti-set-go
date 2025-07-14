use crate::config::*;
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub enum ItemType {
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

impl ItemType {
    pub fn get_texture_name(&self) -> &'static str {
        match self {
            ItemType::PrMerged => "item_pr_merged",
            ItemType::CiPass => "item_ci_pass",
            ItemType::DeploySuccess => "item_deploy_success",
            ItemType::CodeReview => "item_code_review",
            ItemType::TestsPass => "item_tests_pass",
            ItemType::TestFail => "item_test_fail",
            ItemType::MergeConflict => "item_merge_conflict",
            ItemType::CiFail => "item_ci_fail",
            ItemType::SecurityVuln => "item_security_vuln",
        }
    }

    pub fn get_feedback_text(&self) -> &'static str {
        match self {
            ItemType::PrMerged => "Someone finally approved my PR! Let's merge it!",
            ItemType::CiPass => "Phew! The CI pipeline checks all passed!",
            ItemType::DeploySuccess => "Deployment succeeded--my code is live!",
            ItemType::CodeReview => "Their code looks great! Let's approve it!",
            ItemType::TestsPass => "Thank god! All the tests are finally passing!",
            ItemType::TestFail => "Ah, shark farts... some tests are failing...",
            ItemType::MergeConflict => "Of course there's a merge conflict...",
            ItemType::CiFail => "Wait what? The CI pipeline failed? Why??",
            ItemType::SecurityVuln => "Um... do I have to worry about this security vulnerability?",
        }
    }

    pub fn random_good() -> Self {
        let mut rng = thread_rng();
        match rng.gen_range(0..5) {
            0 => ItemType::PrMerged,
            1 => ItemType::CiPass,
            2 => ItemType::DeploySuccess,
            3 => ItemType::CodeReview,
            _ => ItemType::TestsPass,
        }
    }

    pub fn random_bad() -> Self {
        let mut rng = thread_rng();
        match rng.gen_range(0..4) {
            0 => ItemType::TestFail,
            1 => ItemType::MergeConflict,
            2 => ItemType::CiFail,
            _ => ItemType::SecurityVuln,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub is_good: bool,
    pub texture: Option<Texture2D>,
    pub item_type: ItemType,
    pub was_passed: bool,
}

impl Item {
    pub fn new(
        item_type: ItemType,
        is_good: bool,
        textures: &std::collections::HashMap<String, Texture2D>,
    ) -> Self {
        Self {
            x: SCREEN_WIDTH,
            y: GROUND_Y,
            width: ITEM_WIDTH,
            height: ITEM_HEIGHT,
            is_good,
            texture: textures.get(item_type.get_texture_name()).cloned(),
            item_type,
            was_passed: false,
        }
    }

    pub fn random(textures: &std::collections::HashMap<String, Texture2D>) -> Self {
        let mut rng = thread_rng();
        let is_good = rng.gen_bool(GOOD_ITEM_PROBABILITY as f64);

        let item_type = if is_good {
            ItemType::random_good()
        } else {
            ItemType::random_bad()
        };

        Self::new(item_type, is_good, textures)
    }

    pub fn update(&mut self, dt: f32, level: u32) {
        let speed = BASE_ITEM_SPEED + (level as f32 * SPEED_INCREASE_PER_LEVEL);
        self.x -= speed * dt;
    }

    pub fn is_off_screen(&self) -> bool {
        self.x < -self.width
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
