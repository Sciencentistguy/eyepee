use axum::{routing::get, Router};
use once_cell::sync::Lazy;
use reqwest::Client;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new().route("/", get(|| async { get_ip().await.unwrap() }));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_ip() -> Result<String, reqwest::Error> {
    static CLIENT: Lazy<Client> = Lazy::new(Client::new);
    let response = CLIENT.get("https://ipecho.net/plain").send().await?;
    let text = response.text().await?;
    Ok(text)
}
