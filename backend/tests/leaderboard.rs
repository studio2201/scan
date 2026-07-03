//! Black-box integration tests for the scan backend's leaderboard endpoints.

mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;

use common::{build_test_app, get, send, with_connect_info};

#[tokio::test]
async fn leaderboard_get_returns_empty_when_no_file() {
    let (_tmp, _state, router) = build_test_app(None).await;
    let (status, body, _) = send(
        &router,
        with_connect_info(get("/api/leaderboard?category=Alpha")),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn leaderboard_post_persists_and_get_returns_same() {
    let (_tmp, _state, router) = build_test_app(None).await;
    let entry = json!({
        "name": "ALI",
        "score": 100,
        "category": "Alpha",
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/leaderboard")
        .header("content-type", "application/json")
        .header("origin", "http://localhost:4401")
        .body(Body::from(entry.to_string()))
        .expect("req");
    let (status, body, _) = send(&router, with_connect_info(req)).await;
    assert_eq!(status, StatusCode::OK);
    let arr = body.as_array().expect("array");
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["name"], "ALI");
    assert_eq!(arr[0]["score"], 100);

    let (status, body, _) = send(
        &router,
        with_connect_info(get("/api/leaderboard?category=Alpha")),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let arr = body.as_array().expect("array");
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["name"], "ALI");
    assert_eq!(arr[0]["score"], 100);
}

#[tokio::test]
async fn leaderboard_post_sanitizes_player_name() {
    let (_tmp, _state, router) = build_test_app(None).await;
    let entry = json!({ "name": "  xyz123  ", "score": 5, "category": "Alpha" });
    let req = Request::builder()
        .method("POST")
        .uri("/api/leaderboard")
        .header("content-type", "application/json")
        .header("origin", "http://localhost:4401")
        .body(Body::from(entry.to_string()))
        .expect("req");
    let (_, body, _) = send(&router, with_connect_info(req)).await;
    let arr = body.as_array().expect("array");
    assert_eq!(arr.len(), 1);
    let stored = arr[0]["name"].as_str().expect("str");
    assert_eq!(stored, "XYZ");
}

#[tokio::test]
async fn leaderboard_get_returns_401_when_pin_set_no_cookie() {
    let (_tmp, _state, router) = build_test_app(Some("1234")).await;
    let (status, body, _) = send(
        &router,
        with_connect_info(get("/api/leaderboard?category=Alpha")),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "Unauthorized");
}

#[tokio::test]
async fn leaderboard_get_returns_200_with_valid_session_cookie() {
    let (_tmp, state, router) = build_test_app(Some("1234")).await;
    let session_id = "0123456789abcdef0123456789abcdef";
    state.register_session(session_id.to_string()).await;
    let req = Request::builder()
        .uri("/api/leaderboard?category=Alpha")
        .header("cookie", format!("SCAN_PIN={session_id}"))
        .body(Body::empty())
        .expect("req");
    let (status, body, _) = send(&router, with_connect_info(req)).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.is_array());
}
