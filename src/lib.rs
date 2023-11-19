pub mod common {
    use slack_morphism::prelude::*;
    use std::env;

    pub fn get_members_from_teams_string(teams: String) -> Vec<SlackUserId> {
        teams.split(',')
            .filter_map(|team| env::var(&team.to_uppercase()).ok()) // If a team doesn't exist, skip it
            .flat_map(|ids| ids.split(',').map(String::from).collect::<Vec<_>>())
            .map(|id| SlackUserId(id))
            .collect()
    }

    pub fn get_members_from_user_ids_string(user_ids: String) -> Vec<SlackUserId> {
        user_ids.split(',')
            .map(|id| SlackUserId(id.to_string()))
            .collect()
    }
    
    pub async fn create_channel_and_invite_members(channel_name: String, members:Vec<SlackUserId>) -> String {
        let token_value: SlackApiTokenValue = env::var("SLACK_BOT_TOKEN")
            .expect("SLACK_BOT_TOKEN is not set in the environment")
            .into();
        // println!("Token: {}", token_value);
        let token: SlackApiToken = SlackApiToken::new(token_value);
        let client = SlackClient::new(SlackClientHyperConnector::new());
        let session = client.open_session(&token);
    
        let create_channel_response = session
            .conversations_create(&SlackApiConversationsCreateRequest {
                name: channel_name.into(),
                is_private: Some(true),
                user_ds: None,
            })
            .await
            .expect("Failed to create channel");

        let channel_id = create_channel_response.channel.id.clone();

        let _invite_members_response = session
            .conversations_invite(&SlackApiConversationsInviteRequest {
                channel: channel_id,
                users: members,
            })
            .await
            .expect("Failed to invite members");
    
        println!("Channel created and members added successfully.");
    
        String::from("Done")
    }
}

