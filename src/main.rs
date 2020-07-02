#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

use rocket::{response::Debug, Data};
use std::{env, io};

#[post("/<path>", data = "<data>")]
fn upload(path: String, data: Data) -> Result<String, Debug<io::Error>> {
    println!("path = {}", path);
    data.stream_to_file(env::temp_dir().join("upload.txt"))
        .map(|n| n.to_string())
        .map_err(Debug)
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
