use axum::Json;
use entity::post;
use sea_orm::{ActiveModelTrait, Set};

use crate::lib::db::db_connection;

pub async fn get_tasks() -> Json<&'static str> {
  let db = db_connection().await.unwrap();

  let post = post::ActiveModel {
    title: Set(String::from("Amazing title 1")),
    text: Set(String::from("Lorem ipsum dolor sit amet.")),
    ..Default::default()
  };

  let post: post::Model = post.insert(&db).await.unwrap();
  println!("Post created with ID: {}, TITLE: {}", post.id, post.title);
  Json("Hello, World! 🦀")
}

pub async fn create_task() -> Json<&'static str> {
  Json("Create a new task! 🦀")
}
