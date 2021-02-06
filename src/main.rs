use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, web};
use actix_files as fs;
use actix_web_actors::ws;
use ws_sansad::WsSansad;

mod config;
mod messages;
mod ws_sansad;
mod chat_pinnd;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::new();
    let static_path = config.static_path;
    HttpServer::new(move || {
        App::new()
        .service(web::resource("/ws/").route(web::get().to(ws_index)))
        .service(fs::Files::new("/", &static_path).index_file("index.html"))
    })
    .bind(config.bind_address)?
    .run()
    .await
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WsSansad::new(), &req, stream)
}
