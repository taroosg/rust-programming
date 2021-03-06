use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {}

impl ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
  let response_body = "Hello World!";
  Ok(HttpResponse::Ok().body(response_body))
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
