
pub mod common {
    use slack_morphism::prelude::*;
    use std::collections::HashSet;
    use std::env;


    pub fn get_invitees_hashset(teams: String) -> HashSet<SlackUserId> {
        teams.split(',')
            .filter_map(|team| env::var(&team.to_uppercase()).ok()) // If a team doesn't exist, skip it
            .flat_map(|ids| ids.split(',').map(String::from).collect::<Vec<_>>())
            .map(|id| SlackUserId(id))
            .collect()
    }
    
    pub async fn create_channel_and_invite_members(channel_name: String, members:HashSet<SlackUserId>) -> String {
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
}

