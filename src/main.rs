use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use log::info;
use std::io::Write;

mod hashwriter;
use hashwriter::HashWriter;

async fn hello(req: HttpRequest) -> impl Responder {
    info!("REQ: {:?}", req);
    "Hello world!"
}

async fn upload(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut h = HashWriter::new();
    while let Some(item) = body.next().await {
        h.write_all(&item?)?;
    }
    let digest = h.close();

    info!("Digest = {:x?}", digest.as_slice());
    Ok(HttpResponse::Ok().body(digest))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "actix_server=info,actix_web=info,sinkserver=info",
    );
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("*", web::get().to(hello))
            .route("*", web::post().to(upload))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
