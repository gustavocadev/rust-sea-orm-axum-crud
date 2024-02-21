use crate::lib::db::{self, db_connection};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use entity::task::{self, Entity as Task};
use sea_orm::{ActiveModelTrait, DeleteResult, EntityTrait, ModelTrait, Set};
use serde::Deserialize;
use serde_json::json;

pub async fn get_tasks() -> impl IntoResponse {
  let db = db_connection().await.unwrap();

  let tasks = Task::find().all(&db).await.unwrap();

  // Convert tasks to a Vec<serde_json::Value>
  let tasks = tasks
    .into_iter()
    .map(|task| {
      serde_json::json!({
        "id": task.id,
        "title": task.title,
        "description": task.description
      })
    })
    .collect::<Vec<serde_json::Value>>();

  Json(serde_json::json!(tasks))
}

pub async fn get_task(Path(id): Path<u16>) -> impl IntoResponse {
  let db = db_connection().await.unwrap();

  let task = Task::find_by_id(id).one(&db).await.unwrap();

  let task = task
    .into_iter()
    .map(|task| {
      serde_json::json!({
        "id": task.id,
        "title": task.title,
        "description": task.description
      })
    })
    .collect::<Vec<serde_json::Value>>();

  Json(serde_json::json!(task))
}

#[derive(Debug, Deserialize)]
pub struct CreateTask {
  title: String,
  description: String,
}

pub async fn create_task(Json(body): Json<CreateTask>) -> impl IntoResponse {
  let db = db_connection().await.unwrap();

  let task = task::ActiveModel {
    title: Set(String::from(body.title)),
    description: Set(String::from(body.description)),
    ..Default::default() // all other attributes are `NotSet`
  };

  let task: task::Model = task.insert(&db).await.unwrap();

  (
    StatusCode::CREATED,
    Json(serde_json::json!({
      "id": task.id,
      "title": task.title,
    })),
  )
}

#[derive(Debug, Deserialize)]
pub struct UpdateTask {
  title: String,
  description: String,
}

pub async fn update_task(
  Path(id): Path<u16>,
  Json(body): Json<UpdateTask>,
) -> Result<impl IntoResponse, Json<serde_json::Value>> {
  println!("ID: {}", id);

  println!("Body: {:?}", body);

  let db = db_connection().await.unwrap();

  // UPDATE title of Post by ID
  let task: Option<task::Model> = Task::find_by_id(id).one(&db).await.unwrap();

  // transform Option<task::Model> to task::ActiveModel
  let mut task: task::ActiveModel = task.unwrap().into();

  task.title = Set(body.title.to_owned());
  task.description = Set(body.description.to_owned());

  let task = task.update(&db).await.unwrap();

  println!("Post updated with ID: {}, TITLE: {}", task.id, task.title);

  Ok(Json(json!({ "message": "Task updated!" })))
}

pub async fn delete_taks(Path(id): Path<u16>) -> Result<impl IntoResponse, StatusCode> {
  let db = db_connection().await.unwrap();

  // DELETE Post by ID
  let task = Task::find_by_id(id).one(&db).await.unwrap();
  let task = task.unwrap();

  let res: DeleteResult = task.delete(&db).await.unwrap();
  assert_eq!(res.rows_affected, 1);

  println!("Post deleted: {:?}", res);

  Ok(Json(json! ({
    "msg": "Task deleted! 🦀",
  })))
}
