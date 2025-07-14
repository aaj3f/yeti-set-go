use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighScore {
    pub name: String,
    pub score: u32,
    pub level: u32,
    pub timestamp: DateTime<Utc>,
}

impl HighScore {
    pub fn new(name: String, score: u32, level: u32) -> Self {
        Self {
            name,
            score,
            level,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leaderboard {
    pub scores: Vec<HighScore>,
    pub local_best: Option<HighScore>,
}

impl Leaderboard {
    pub fn new() -> Self {
        Self {
            scores: Vec::new(),
            local_best: None,
        }
    }

    pub fn add_score(&mut self, high_score: HighScore) {
        // Update local best if this is better
        if self.local_best.is_none() || high_score.score > self.local_best.as_ref().unwrap().score {
            self.local_best = Some(high_score.clone());
        }

        // Add to scores and sort
        self.scores.push(high_score);
        self.scores.sort_by(|a, b| b.score.cmp(&a.score));

        // Keep only top 25
        if self.scores.len() > 25 {
            self.scores.truncate(25);
        }
    }

    pub fn is_high_score(&self, score: u32) -> bool {
        if self.scores.len() < 25 {
            return true;
        }
        score > self.scores.last().unwrap().score
    }

    pub fn get_top_3(&self) -> Vec<&HighScore> {
        self.scores.iter().take(3).collect()
    }

    pub fn get_rank(&self, score: u32) -> Option<usize> {
        for (i, high_score) in self.scores.iter().enumerate() {
            if score >= high_score.score {
                return Some(i + 1);
            }
        }
        if self.scores.len() < 25 {
            Some(self.scores.len() + 1)
        } else {
            None
        }
    }

    pub fn merge_remote_scores(&mut self, remote_scores: Vec<HighScore>) {
        // Merge remote scores with local scores
        let mut all_scores = self.scores.clone();
        all_scores.extend(remote_scores);

        // Remove duplicates based on name and score (in case of sync issues)
        let mut seen = HashMap::new();
        all_scores.retain(|score| {
            let key = (score.name.clone(), score.score);
            seen.insert(key, ()).is_none()
        });

        // Sort by score and keep top 25
        all_scores.sort_by(|a, b| b.score.cmp(&a.score));
        all_scores.truncate(25);

        self.scores = all_scores;
    }

    pub fn get_local_best_score(&self) -> u32 {
        self.local_best.as_ref().map_or(0, |score| score.score)
    }
}
