use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, web};

async fn health_check() -> impl Responder {
  HttpResponse::Ok().finish()
}


pub fn run() -> Result<Server, std::io::Error> {
  let server = HttpServer::new(|| {
    App::new()
      .route("/health_check", web::get().to(health_check))
  })
  .bind(("127.0.0.1", 8080))?
  .run();

  Ok(server)
}