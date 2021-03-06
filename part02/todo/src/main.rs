use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use thiserror::Error;

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
}

impl ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
  let mut entries = vec![];
  entries.push(TodoEntry {
    id: 1,
    text: "First Entry".to_string(),
  });
  entries.push(TodoEntry {
    id: 2,
    text: "Secont Entry".to_string(),
  });
  let html = IndexTemplate { entries };

  let response_body = html.render()?;
  Ok(
    HttpResponse::Ok()
      .content_type("text/html")
      .body(response_body),
  )
}

// actix_webのバージョンが上がってactix_rt使わなくても良くなった模様
#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
  HttpServer::new(|| App::new().service(index))
    .bind("127.0.0.1:8087")?
    .run()
    .await?;
  Ok(())
}
