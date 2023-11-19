fn main() {
}

#[cfg(test)]
mod tests {
    use rusty_slack::common::get_invitees_hashset;
    use slack_morphism::SlackUserId;
    // use super::*;
    // use mockito::{mock, server_url};
    use std::env;
    use std::collections::HashSet;

    #[test]
    fn test_get_invitees_hashset() {
        env::set_var("TEAM1", "user1,user2,user3");
        env::set_var("TEAM2", "user4,user5");

        let teams = "team1,team2";
        let result = get_invitees_hashset(teams.to_string());

        let mut expected = HashSet::new();
        expected.insert(SlackUserId("user1".to_string()));
        expected.insert(SlackUserId("user2".to_string()));
        expected.insert(SlackUserId("user3".to_string()));
        expected.insert(SlackUserId("user4".to_string()));
        expected.insert(SlackUserId("user5".to_string()));

        assert_eq!(result, expected);

        env::remove_var("TEAM1");
        env::remove_var("TEAM2");
    }

    #[test]
    fn test_get_invitees_hashset_with_missing_team() {
        env::set_var("TEAM1", "user1,user2,user3");
        env::set_var("TEAM3", "user4,user5");

        let teams = "team1,team2,team3";
        let result = get_invitees_hashset(teams.to_string());

        let mut expected = HashSet::new();
        expected.insert(SlackUserId("user1".to_string()));
        expected.insert(SlackUserId("user2".to_string()));
        expected.insert(SlackUserId("user3".to_string()));
        expected.insert(SlackUserId("user4".to_string()));
        expected.insert(SlackUserId("user5".to_string()));

        assert_eq!(result, expected);

        env::remove_var("TEAM1");
        env::remove_var("TEAM3");
    }
}