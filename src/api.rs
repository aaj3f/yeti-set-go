use crate::highscores::{HighScore, Leaderboard};
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use std::env;
use std::time::Duration;

const API_BASE_URL: &str = "https://data.flur.ee/fluree";
const TIMEOUT_SECONDS: u64 = 5;

fn get_api_key() -> Result<String, ApiError> {
    // Try compile-time embedded key first
    if let Some(key) = option_env!("FLUREE_API_KEY") {
        if !key.is_empty() {
            return Ok(key.to_string());
        }
    }

    // Fall back to runtime environment variable
    env::var("FLUREE_API_KEY").map_err(|_| ApiError::MissingApiKey)
}

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    enabled: bool,
}

impl ApiClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("YetiSetGo/1.0")
            .timeout(Duration::from_secs(TIMEOUT_SECONDS))
            .build()
            .unwrap_or_else(|_| Client::new());

        // Check if API key is available on startup
        let enabled = match get_api_key() {
            Ok(_) => {
                println!("✓ Fluree API key loaded successfully");
                true
            }
            Err(_) => {
                println!("⚠ Warning: FLUREE_API_KEY environment variable not set. API features disabled.");
                false
            }
        };

        Self { client, enabled }
    }

    pub async fn fetch_leaderboard(&self) -> Result<Vec<HighScore>, ApiError> {
        if !self.enabled {
            return Err(ApiError::Disabled);
        }

        let api_key = get_api_key()?;

        let query = serde_json::json!({
            "from": "ajohnson/yeti-set-go",
            "where": [
                {
                    "@id": "?s",
                    "score": "?score",
                }
            ],
            "select": { "?s": ["*"] },
            "orderBy": "(desc ?score)",
            "limit": 20
        });

        let url = format!("{}/query", API_BASE_URL);

        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .header(CONTENT_TYPE, "application/json")
            .json(&query)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ApiError::ServerError(response.status().as_u16()));
        }

        let scores: Vec<HighScore> = response
            .json()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))?;

        Ok(scores)
    }

    pub async fn submit_score(&self, high_score: &HighScore) -> Result<(), ApiError> {
        if !self.enabled {
            return Err(ApiError::Disabled);
        }

        let api_key = get_api_key()?;

        let transaction = serde_json::json!({
            "ledger": "ajohnson/yeti-set-go",
            "insert": {
                "score": high_score.score,
                "name": high_score.name,
                "timestamp": high_score.timestamp,
                "level": high_score.level,
            }
        });

        let url = format!("{}/transact", API_BASE_URL);

        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .header(CONTENT_TYPE, "application/json")
            .json(&transaction)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ApiError::ServerError(response.status().as_u16()));
        }

        Ok(())
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

#[derive(Debug, Clone)]
pub enum ApiError {
    NetworkError(String),
    ServerError(u16),
    ParseError(String),
    Disabled,
    MissingApiKey,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ApiError::ServerError(code) => write!(f, "Server error: {}", code),
            ApiError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ApiError::Disabled => write!(f, "API disabled"),
            ApiError::MissingApiKey => write!(
                f,
                "Missing API key: set FLUREE_API_KEY environment variable"
            ),
        }
    }
}

impl std::error::Error for ApiError {}

// High-level API functions with fallback behavior
pub async fn load_leaderboard_with_fallback(
    api_client: &ApiClient,
    local_leaderboard: &mut Leaderboard,
) -> bool {
    match api_client.fetch_leaderboard().await {
        Ok(remote_scores) => {
            local_leaderboard.merge_remote_scores(remote_scores);
            true // Successfully loaded from API
        }
        Err(e) => {
            println!("Failed to load remote leaderboard: {}", e);
            false // Using local fallback
        }
    }
}

pub async fn submit_score_with_fallback(
    api_client: &ApiClient,
    high_score: &HighScore,
    local_leaderboard: &mut Leaderboard,
) -> bool {
    // Always add to local leaderboard first
    local_leaderboard.add_score(high_score.clone());

    // Try to submit to remote API
    match api_client.submit_score(high_score).await {
        Ok(()) => {
            println!("Score submitted successfully to remote API");

            // Re-query the leaderboard to get updated state from API
            match api_client.fetch_leaderboard().await {
                Ok(remote_scores) => {
                    local_leaderboard.merge_remote_scores(remote_scores);
                    println!("Leaderboard updated after score submission");
                }
                Err(e) => {
                    println!("Failed to update leaderboard after submission: {}", e);
                }
            }

            true
        }
        Err(e) => {
            println!("Failed to submit score to remote API: {}", e);
            false // Score saved locally as fallback
        }
    }
}
