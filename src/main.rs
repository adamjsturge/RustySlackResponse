use slack_morphism::prelude::*;
use slack_morphism_hyper::*;
use std::env;

#[tokio::main]
async fn main() {
    let client = SlackHyperClient::new(SlackClientEnvironment::new());
    // // Set your Slack OAuth token as an environment variable
    // let token = env::var("SLACK_OAUTH_TOKEN").expect("Missing SLACK_OAUTH_TOKEN");

    // // Initialize the Slack client with the OAuth token
    // let client = SlackHyperClient::new(
    //     SlackClientEnvironment::new().with_token(SlackApiToken::new(token))
    // );

    // let client = SlackClient::new(SlackClientHyperConnector::new());

    // let token_value: SlackApiTokenValue = get_from_env("SLACK_BOT_TOKEN").unwrap().into();
    // let token: SlackApiToken = SlackApiToken::new(token_value);
    // let session = client.open_session(&token);

    // Create a private channel
    let create_channel_response = client
        .conversations_create(&SlackApiConversationsCreateRequest {
            name: "your_private_channel_name".into(),
            is_private: Some(true),
            ..Default::default()
        })
        .await
        .expect("Failed to create channel");

    let channel_id = create_channel_response
        .channel
        .unwrap()
        .id;

    // Add members to the channel
    let users_to_add = vec!["user_id1", "user_id2"]; // Replace with actual user IDs

    for user_id in users_to_add {
        client
            .conversations_invite(&SlackApiConversationsInviteRequest {
                channel: channel_id.clone(),
                users: vec![user_id.to_string()],
            })
            .await
            .expect("Failed to add member to the channel");
    }

    println!("Channel created and members added successfully.");
}
