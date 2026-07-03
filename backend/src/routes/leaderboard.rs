//! Scan leaderboard persistence and endpoints.

use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

use crate::error::AppError;
use crate::state::AppState;

const MAX_PLAYER_NAME_CHARS: usize = 3;
const MAX_LEADERBOARD_ENTRIES: usize = 10;
const ANONYMOUS_NAME: &str = "AAA";

/// One row in the leaderboard.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeaderboardEntry {
    /// Player-chosen display name; sanitised server-side.
    pub name: String,
    /// Time elapsed in tenths of a second, lower = better.
    pub score: u32,
    /// Rfc3339 timestamp taken when the score was submitted.
    pub date: String,
}

#[derive(Deserialize)]
pub struct GetParams {
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubmitRequest {
    pub name: String,
    pub score: u32,
    pub category: String,
}

pub type LeaderboardData = HashMap<String, Vec<LeaderboardEntry>>;

async fn read_leaderboard(path: &Path) -> LeaderboardData {
    match fs::read_to_string(path).await {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}

#[must_use]
pub fn sanitize_player_name(raw: &str) -> String {
    let trimmed = raw.trim().to_ascii_uppercase();
    let chars: String = trimmed
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();
    if chars.is_empty() {
        return ANONYMOUS_NAME.to_string();
    }
    if chars.len() <= MAX_PLAYER_NAME_CHARS {
        chars
    } else {
        chars.chars().take(MAX_PLAYER_NAME_CHARS).collect()
    }
}

async fn atomic_write(path: &Path, content: &[u8]) -> Result<(), AppError> {
    let parent = path
        .parent()
        .ok_or_else(|| AppError::internal("leaderboard path has no parent"))?;
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| AppError::internal("leaderboard filename invalid"))?;
    let tmp = parent.join(format!(".{file_name}.tmp"));
    fs::create_dir_all(parent).await?;
    fs::write(&tmp, content).await?;
    if let Err(e) = fs::rename(&tmp, path).await {
        let _ = fs::remove_file(&tmp).await;
        return Err(AppError::Io(e));
    }
    Ok(())
}

pub async fn get_leaderboard(
    State(state): State<AppState>,
    Query(params): Query<GetParams>,
) -> Response {
    let path = state.leaderboard_file.clone();
    let category = params.category.unwrap_or_else(|| "Alpha".to_string());
    let data = read_leaderboard(&path).await;
    let list = data.get(&category).cloned().unwrap_or_default();
    (StatusCode::OK, Json(list)).into_response()
}

pub async fn read_leaderboard_count(state: &AppState) -> u64 {
    let data = read_leaderboard(&state.leaderboard_file).await;
    data.values().map(Vec::len).sum::<usize>() as u64
}

pub async fn submit_score(
    State(state): State<AppState>,
    Json(req): Json<SubmitRequest>,
) -> Result<Response, AppError> {
    let path = state.leaderboard_file.clone();
    let name = sanitize_player_name(&req.name);
    let date = Utc::now().to_rfc3339();

    let entry = LeaderboardEntry {
        name,
        score: req.score,
        date,
    };

    let _guard = state.leaderboard_lock.lock().await;

    let mut data = read_leaderboard(&path).await;
    let list = data.entry(req.category.clone()).or_insert_with(Vec::new);
    list.push(entry);

    // Sort ascending: lower score (time) is better!
    list.sort_by_key(|e| e.score);
    list.truncate(MAX_LEADERBOARD_ENTRIES);

    let content = serde_json::to_string_pretty(&data)?;
    atomic_write(&path, content.as_bytes()).await?;

    let updated_list = data.get(&req.category).cloned().unwrap_or_default();
    Ok((StatusCode::OK, Json(updated_list)).into_response())
}
