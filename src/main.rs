use slack_morphism::prelude::*;
use std::collections::HashSet;
use dotenv::dotenv;
use std::env;


fn main() {
    dotenv().ok();
    let _members = get_invitees_hashset("security,legal".into());
}

fn get_invitees_hashset(teams: String) -> HashSet<SlackUserId> {
    teams.split(',')
        .filter_map(|team| env::var(&team.to_uppercase()).ok()) // If a team doesn't exist, skip it
        .flat_map(|ids| ids.split(',').map(String::from).collect::<Vec<_>>())
        .map(|id| SlackUserId(id))
        .collect()
}

async fn create_channel_and_invite_members(channel_name: String, members:HashSet<SlackUserId>) -> String {
    let token_value: SlackApiTokenValue = env::var("SLACK_BOT_TOKEN")
        .expect("SLACK_BOT_TOKEN is not set in the environment")
        .into();
    let token: SlackApiToken = SlackApiToken::new(token_value);
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let session = client.open_session(&token);

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

#[cfg(test)]
mod tests {
    use super::*;
    // use mockito::{mock, server_url};
    use std::env;
    use std::collections::HashSet;

    #[test]
    fn test_get_invitees_hashset() {
        // Set up the environment variables for the teams
        env::set_var("TEAM1", "user1,user2,user3");
        env::set_var("TEAM2", "user4,user5");

        // Call the function with a string of team names
        let teams = "team1,team2";
        let result = get_invitees_hashset(teams.to_string());

        // Create the expected HashSet
        let mut expected = HashSet::new();
        expected.insert(SlackUserId("user1".to_string()));
        expected.insert(SlackUserId("user2".to_string()));
        expected.insert(SlackUserId("user3".to_string()));
        expected.insert(SlackUserId("user4".to_string()));
        expected.insert(SlackUserId("user5".to_string()));

        // Assert that the function's output matches the expected output
        assert_eq!(result, expected);

        // Clean up the environment variables
        env::remove_var("TEAM1");
        env::remove_var("TEAM2");
    }

    #[test]
    fn test_get_invitees_hashset_with_missing_team() {
        // Set up the environment variables for the teams
        env::set_var("TEAM1", "user1,user2,user3");
        env::set_var("TEAM3", "user4,user5");

        // Call the function with a string of team names
        let teams = "team1,team2,team3";
        let result = get_invitees_hashset(teams.to_string());

        // Create the expected HashSet
        let mut expected = HashSet::new();
        expected.insert(SlackUserId("user1".to_string()));
        expected.insert(SlackUserId("user2".to_string()));
        expected.insert(SlackUserId("user3".to_string()));
        expected.insert(SlackUserId("user4".to_string()));
        expected.insert(SlackUserId("user5".to_string()));

        // Assert that the function's output matches the expected output
        assert_eq!(result, expected);

        // Clean up the environment variables
        env::remove_var("TEAM1");
        env::remove_var("TEAM3");
    }

    // #[tokio::test]
    // async fn test_create_channel_and_invite_members() {
    //     let _m = mock("POST", "/api/conversations.create")
    //         .with_status(200)
    //         .with_body(r#"{"ok": true, "channel": {"id": "C024BE91L"}}"#)
    //         .create();

    //     let _n = mock("POST", "/api/conversations.invite")
    //         .with_status(200)
    //         .with_body(r#"{"ok": true}"#)
    //         .create();

    //     env::set_var("SLACK_BOT_TOKEN", "test-token");

    //     let result = create_channel_and_invite_members("test-channel".into(), "user1,user2".into()).await;
    //     assert_eq!(result, "C024BE91L");
    // }
}