use rusty_slack_response::common::{create_channel_and_invite_members, get_members_from_teams_string};
use dotenv::dotenv;

#[tokio::main]
async fn main(){
    dotenv().ok();
    println!("Starting");
    let members = get_members_from_teams_string("security".to_string());
    let channel_name = "privatetest4".to_string();
    let _result = create_channel_and_invite_members(channel_name, members).await;    
}