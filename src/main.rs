#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

mod hashwriter;
use hashwriter::{HashWriter};

use rocket::{response::Debug, Data};
use std::io;

#[post("/<path>", data = "<data>")]
fn upload(path: String, data: Data) -> Result<Vec<u8>, Debug<io::Error>> {
    println!("path = {}", path);
    let mut h = HashWriter::new();
    data.stream_to(&mut h).map_err(Debug)?;
    Ok(h.close())
}

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, upload])
}

fn main() {
    rocket().launch();
}
