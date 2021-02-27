use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let listener = TcpListener::bind("localhost:8080").expect("Failed to bind random port");
  zero2prod::run(listener)?.await
}