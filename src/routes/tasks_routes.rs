use crate::controllers::tasks_controller::get_tasks;
use axum::routing::get;
use axum::Router;

pub fn init_tasks_routes() -> Router {
  Router::new().route("/", get(get_tasks))
}
