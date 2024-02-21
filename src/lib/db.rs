use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DbConn, DbErr};
use std::env;

pub async fn db_connection() -> Result<DbConn, DbErr> {
  dotenv().ok().unwrap();

  let db = Database::connect(env::var("DATABASE_URL").unwrap())
    .await
    .unwrap();

  Migrator::up(&db, None)
    .await
    .expect("Failed to run migrations for tests");

  Ok(db)
}
