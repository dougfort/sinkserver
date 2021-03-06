use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use http::header;
use log::debug;
use std::env;
use std::io::Write;

mod hashwriter;
use hashwriter::HashWriter;

async fn hello(req: HttpRequest) -> impl Responder {
    debug!("REQ: {:?}", req);
    "Hello world!"
}

async fn upload(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut h = HashWriter::new();
    let mut size: u64 = 0;
    while let Some(item) = body.next().await {
        size += h.write(&item?)? as u64;
    }
    let digest = h.close();
    let digest_as_slice = digest.as_slice();

    debug!(
        "size = {}; Digest = {:?}",
        size,
        hex::encode(digest_as_slice)
    );
    Ok(HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .body(digest))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "actix_server=info,actix_web=info,sinkserver=debug",
    );
    env_logger::init();

    let address = construct_address();

    HttpServer::new(|| {
        App::new()
            .route("*", web::get().to(hello))
            .route("*", web::post().to(upload))
    })
    .bind(address)?
    .run()
    .await
}

fn construct_address() -> String {
    let host = match env::var_os("SINKSERVER_HOST") {
        Some(val) => val.into_string().unwrap(),
        None => "0.0.0.0".to_string(),
    };
    let port = match env::var_os("SINKSERVER_PORT") {
        Some(val) => val.into_string().unwrap(),
        None => "3000".to_string(),
    };
    format!("{}:{}", host, port)
}
