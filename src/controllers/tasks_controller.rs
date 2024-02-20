use axum::Json;

pub async fn get_tasks() -> Json<&'static str> {
  Json("Hello, World! This is the tasks route!")
}
