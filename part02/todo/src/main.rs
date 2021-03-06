use actix_web::{get, http::header, post, web, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Deserialize;
use thiserror::Error;

// データ追加用の構造体
#[derive(Deserialize)]
struct AddParams {
  text: String,
}

// データ削除用の構造体
#[derive(Deserialize)]
struct DeleteParams {
  id: u32,
}

// 取得したデータの構造体
struct TodoEntry {
  id: u32,
  text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
  entries: Vec<TodoEntry>,
}

#[derive(Error, Debug)]
enum MyError {
  #[error("Failed to render HTML")]
  AskamaError(#[from] askama::Error),

  #[error("Failed to get connection.")]
  ConnectionError(#[from] r2d2::Error),

  #[error("Failed to SQL ececution")]
  SQLiteError(#[from] rusqlite::Error),
}

impl ResponseError for MyError {}

#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
  let conn = db.get()?;
  let mut statement = conn.prepare("SELECT id, text FROM todo")?;
  let rows = statement.query_map(params![], |row| {
    let id = row.get(0)?;
    let text = row.get(1)?;
    Ok(TodoEntry { id, text })
  })?;

  let mut entries = vec![];
  for row in rows {
    entries.push(row?);
  }
  let html = IndexTemplate { entries };

  let response_body = html.render()?;
  Ok(
    HttpResponse::Ok()
      .content_type("text/html")
      .body(response_body),
  )
}

#[post("/add")]
async fn add_todo(
  params: web::Form<AddParams>,
  db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
  let conn = db.get()?;
  conn.execute("INSERT INTO todo (text) VALUES (?)", &[&params.text])?;
  Ok(
    HttpResponse::SeeOther()
      .header(header::LOCATION, "/")
      .finish(),
  )
}

#[post("/delete")]
async fn delete_todo(
  params: web::Form<DeleteParams>,
  db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
  let conn = db.get()?;
  conn.execute("DELETE FROM todo WHERE id=?", &[&params.id])?;
  Ok(
    HttpResponse::SeeOther()
      .header(header::LOCATION, "/")
      .finish(),
  )
}

// actix_webのバージョンが上がってactix_rt使わなくても良くなった模様
#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
  // コネクションプールを作成
  let manager = SqliteConnectionManager::file("todo.db");
  let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
  // DB接続
  let conn = pool
    .get()
    .expect("Failed to get the connection from the pool.");
  // SQL実行
  conn
    .execute(
      "CREATE TABLE IF NOT EXISTS todo (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        text TEXT NOT NULL
      )",
      params![],
    )
    .expect("Failed to create a table `todo`.");

  HttpServer::new(move || {
    App::new()
      .service(index)
      .service(add_todo)
      .service(delete_todo)
      .data(pool.clone())
  })
  // ここは0.0.0.0にしないと死ぬ
  .bind("0.0.0.0:8087")?
  .run()
  .await?;
  Ok(())
}
