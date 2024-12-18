//! This example demonstrates a basic Axum server with two endpoints:
//! - A GET endpoint that returns a Simple greeting
//! - A POST endpoint that accepts JSON data and returns a user object
//!
//! ## GET request
//!
//! ```
//! curl http://localhost:8000
//! ```
//! Expected Response:
//! ```
//! Hello World!
//! ```
//!
//! ## POST request
//!
//! ```
//! curl -X POST http://localhost:8000 \
//!      -H "Content-Type: application/json" \
//!      -d '{"username": "alice}"'
//! ```
//!
//! Expected Response:
//! ```
//! {"id":1, "username": "ritzier"}
//! ```

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build application
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    // Run app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    // Simulate user creation
    let user = User {
        id: 1,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

// Reqeust payload for creating a user
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// Response payload for a user
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
