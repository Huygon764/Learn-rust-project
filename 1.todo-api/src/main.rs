use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    handlers::{create_todo, delete_todo, get_todo, get_todos, update_todo},
    state::create_store,
};

mod errors;
mod handlers;
mod models;
mod state;

#[tokio::main]
async fn main() {
    let store = create_store();

    let app = Router::new()
        .route("/todos", get(get_todos))
        .route("/todos", post(create_todo))
        .route("/todos/:id", get(get_todo))
        .route("/todos/:id", put(update_todo))
        .route("/todos/:id", delete(delete_todo))
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Server running on http://localhost:3000");
    println!("📋 Available endpoints:");
    println!("  GET    /todos       - Get all todos");
    println!("  POST   /todos       - Create new todo");
    println!("  GET    /todos/:id   - Get todo by ID");
    println!("  PUT    /todos/:id   - Update todo");
    println!("  DELETE /todos/:id   - Delete todo");

    axum::serve(listener, app).await.unwrap();
}
