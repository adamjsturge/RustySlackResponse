use slack_morphism::prelude::*;
use std::collections::HashSet;
use dotenv::dotenv;
use std::env;


fn main() {
    dotenv().ok();
    let members_string = create_members_string("security,legal".into());
    println!("Member String {}", members_string);
}

fn create_members_string(teams: String) -> String {
    let groups: Vec<String> = teams.split(',').map(|s| s.to_string()).collect();
    let mut members_string = String::new();
    for group in groups {
        let group_members = env::var(&group.to_uppercase()).expect("Group not found in env");
        members_string.push_str(&group_members);
        members_string.push_str(",");
    }
    members_string
}

async fn create_channel_and_invite_members(channel_name: String, members_string: String) -> String {
    let token_value: SlackApiTokenValue = env::var("SLACK_BOT_TOKEN")
        .expect("SLACK_BOT_TOKEN is not set in the environment")
        .into();
    let token: SlackApiToken = SlackApiToken::new(token_value);
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let session = client.open_session(&token);

    let members: HashSet<SlackUserId> = members_string.split(',')
    .filter(|id_str| !id_str.is_empty())
    .map(|id_str| SlackUserId::from(id_str))
    .collect();

    let _create_channel_response = session
        .conversations_create(&SlackApiConversationsCreateRequest {
            name: channel_name.into(),
            is_private: Some(true),
            user_ds: Some(members),
        })
        .await
        .expect("Failed to create channel");

    println!("Channel created and members added successfully.");

    String::from("Done")
}