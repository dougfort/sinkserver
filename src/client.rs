use actix_web::client::Client;

#[actix_web::main]
async fn main() {
    let client = Client::default();

    // Create request builder and send request
    let response = client
        .get("http://localhost:3000/hello")
        .header("User-Agent", "actix-web/3.0")
        .send() // <- Send request
        .await; // <- Wait for response

    println!("Response: {:?}", response);
}
