mod controllers;
mod lib;
mod routes;

use crate::routes::tasks_routes::init_tasks_routes;
use axum::Router;

#[tokio::main]
async fn main() {
  // inialize tracing
  tracing_subscriber::fmt::init();

  // create a new router
  let app = Router::new().nest("/api/tasks", init_tasks_routes());

  // create a new listener
  println!("Server running on port {}", 3000);
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
