use crate::controllers::tasks_controller::{create_task, get_tasks};
use axum::routing::{get, post};
use axum::Router;

pub fn init_tasks_routes() -> Router {
  Router::new()
    .route("/", get(get_tasks))
    .route("/", post(create_task))
}
