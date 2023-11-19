use axum::{extract::Json, extract::Path, routing::get, routing::post, Router};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use serde::Deserialize;

use rusty_slack::common::{get_members_from_teams_string, create_channel_and_invite_members, get_members_from_user_ids_string};

#[derive(Deserialize, Debug)]
struct CreateChannelBodyData {
    channel_name: String,
    teams: Option<String>,
    user_ids: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/create_channel/:id", post(create_channel));

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

async fn create_channel(Path(id): Path<String>, Json(body): Json<CreateChannelBodyData>) -> String {
    if env::var("HASH").unwrap() != id {
        return format!("Unauthorized");
    }

    let invitees = match body.user_ids {
        Some(user_ids) => get_members_from_user_ids_string(user_ids),
        None => Vec::new(),
    };
    let teams = match body.teams {
        Some(teams) => get_members_from_teams_string(teams),
        None => Vec::new(),
    };

    // Combine invitees and teams into a single vector
    let members = invitees.into_iter().chain(teams.into_iter()).collect::<Vec<_>>();

    let channel_name = body.channel_name;
    let result = create_channel_and_invite_members(channel_name, members);
    result.await
}