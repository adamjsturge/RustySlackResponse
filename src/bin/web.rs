use axum::{extract::Json, extract::Path, routing::get, routing::post, Router};
use dotenv::dotenv;
use std::{env, string};
use std::net::SocketAddr;
use serde::Deserialize;

use rusty_slack::common::{get_invitees_hashset, create_channel_and_invite_members, string_to_hashset};

#[derive(Deserialize, Debug)]
struct BodyV1Data {
    teams: String,
    channel_name: String,
}

struct BodyV2Data {
    user_ids: String,
    channel_name: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/v1/create_channel/:id", post(create_channel_v1))
        .route("/v2/create_channel/:id", post(create_channel_v2));

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

async fn create_channel_v1(Path(id): Path<String>, Json(body): Json<BodyV1Data>) -> String {
    if env::var("HASH").unwrap() != id {
        return format!("Unauthorized");
    }
    let invitees = get_invitees_hashset(body.teams);
    let channel_name = body.channel_name;
    let result = create_channel_and_invite_members(channel_name, invitees);
    result.await
}

async fn create_channel_v2(Path(id): Path<String>, Json(body): Json<BodyV2Data>) -> String {
    if env::var("HASH").unwrap() != id {
        return format!("Unauthorized");
    }
    let invitees = string_to_hashset(body.user_ids);
    let channel_name = body.channel_name;
    let result = create_channel_and_invite_members(channel_name, invitees);
    result.await
}
