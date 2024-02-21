mod controllers;
mod lib;
mod routes;

use crate::routes::tasks_routes::tasks_api;
use axum::Router;

#[tokio::main]
async fn main() {
  // inialize tracing
  tracing_subscriber::fmt::init();

  // create a new router
  let app = Router::new().nest("/api/tasks", tasks_api());

  // create a new listener
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
