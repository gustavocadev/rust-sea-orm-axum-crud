use crate::controllers::tasks_controller::{
  create_task, delete_taks, get_task, get_tasks, update_task,
};
use axum::routing::{get, patch};
use axum::Router;

pub fn init_tasks_routes() -> Router {
  Router::new()
    .route("/", get(get_tasks).post(create_task))
    .route("/:id", patch(update_task).delete(delete_taks).get(get_task))
}
