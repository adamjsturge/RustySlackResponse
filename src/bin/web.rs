use axum::{extract::Json, extract::Path, routing::get, routing::post, Router};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use serde::Deserialize;

use rusty_slack::common::{get_invitees_hashset, create_channel_and_invite_members};

#[derive(Deserialize, Debug)]
struct BodyData {
    teams: String,
    channel_name: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/webhook/:id", post(webhook_handler));

    // Address that server will bind to.
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler() -> &'static str {
    "Hello, World!"
}

async fn webhook_handler(Path(id): Path<String>, Json(body): Json<BodyData>) -> String {
    if env::var("HASH").unwrap() == id {
        process_webhook(body).await
    } else {
        format!("Unauthorized")
    }
}

async fn process_webhook(body: BodyData) -> String {
    let invitees = get_invitees_hashset(body.teams);
    let channel_name = body.channel_name;
    let result = create_channel_and_invite_members(channel_name, invitees);
    result.await
}