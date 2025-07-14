use crate::game::state::Game;
use crate::colors::*;

pub fn update_item_scoring(game: &mut Game, _dt: f32) {
    
    for item in &mut game.items {
        // Check if item has passed the yeti (item's right edge is behind yeti's left edge)
        if item.x + item.width < game.yeti.x && !item.was_passed {
            item.was_passed = true;
            
            // If it's a bad item that we successfully avoided, award points
            if !item.is_good {
                game.score += 5; // Less than collision bonus, but still rewarding
                game.checks_completed += 1;
                
                // Show feedback for successful avoidance
                game.feedback_message = "Nice dodge! Avoided a problem!".to_string();
                game.feedback_timer = 2.0;
                game.feedback_color = SUCCESS_GREEN;
            }
        }
    }
}

pub fn calculate_level_score_bonus(level: u32) -> u32 {
    // Bonus points for completing a level
    50 + (level * 25)
}

pub fn calculate_total_score_with_bonuses(base_score: u32, level: u32, checks_completed: u32) -> u32 {
    let level_bonus = if level > 1 { 
        (1..level).map(calculate_level_score_bonus).sum::<u32>()
    } else { 
        0 
    };
    
    let completion_bonus = checks_completed * 2; // Small bonus for each check completed
    
    base_score + level_bonus + completion_bonus
}