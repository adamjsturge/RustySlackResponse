use slack_morphism::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use clap::{Arg, ArgAction, Command, ArgGroup};
use std::iter::FromIterator;

fn main() {
    let matches = Command::new("rusty_slack")
        .version("1.0")
        .author("Your Name")
        .about("Manages Slack channels")
        .subcommand(
            Command::new("create:channel")
                .about("Creates a new channel")
                .arg(
                    Arg::new("teams")
                        .long("teams")
                        .value_name("TEAM")
                        .help("Lists the teams to be invited")
                )
                .group(ArgGroup::new("teams_group")
                .args(["flag", "color"])
                .multiple(true))
        )
        .get_matches();

    // Handle the create:channel subcommand
    if let Some(matches) = matches.subcommand_matches("create:channel") {
        if let Some(teams) = matches.get_many("teams") {
            let teams_vec: Vec<String> = teams.map(|t| t.to_string()).collect();
        }
    }
    
}

fn create_members_string(teams: String) -> String {
    let groups: Vec<String> = Values::from_iter(teams).map(|s| s.to_string()).collect();
    let mut members_string = String::new();
    for group in groups {
        let group_members = env::var(&group).expect("Group not found in env");
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

    let create_channel_response = session
        .conversations_create(&SlackApiConversationsCreateRequest {
            name: channel_name.into(),
            is_private: Some(true),
            user_ds: Some(members),
        })
        .await
        .expect("Failed to create channel");

    // let channel_id = create_channel_response
    //     .channel
    //     .unwrap()
    //     .id;


    println!("Channel created and members added successfully.");

    String::from("Done")
}

// fn create_members_string(groups: Vec<String>) -> String {
//     let mut members_string = String::new();
//     for group in groups {
//         let group_members = env::var(group).expect("Group not found in env");
//         members_string.push_str(&group_members);
//         members_string.push_str(",");
//     }
//     members_string
// }