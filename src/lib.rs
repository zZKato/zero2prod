use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;

async fn health_check() -> HttpResponse {
  HttpResponse::Ok().finish()
}

async fn subscribe() -> HttpResponse {
  HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server = HttpServer::new(|| {
    App::new()
      .route("/health_check", web::get().to(health_check))
      .route("/subscriptions", web::post().to(subscribe))
  })
  .listen(listener)?
  .run();

  Ok(server)
}