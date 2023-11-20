fn main() {
}

#[cfg(test)]
mod tests {
    use rusty_slack_response::common::{get_members_from_teams_string, get_members_from_user_ids_string};
    use slack_morphism::SlackUserId;
    use std::env;

    #[test]
    fn test_get_invitees_vec() {
        env::set_var("TEAM1", "user1,user2,user3");
        env::set_var("TEAM2", "user4,user5");

        let teams = "team1,team2";
        let result = get_members_from_teams_string(teams.to_string());

        let mut expected = Vec::new();
        expected.push(SlackUserId("user1".to_string()));
        expected.push(SlackUserId("user2".to_string()));
        expected.push(SlackUserId("user3".to_string()));
        expected.push(SlackUserId("user4".to_string()));
        expected.push(SlackUserId("user5".to_string()));

        assert_eq!(result, expected);

        env::remove_var("TEAM1");
        env::remove_var("TEAM2");
    }

    #[test]
    fn test_missing_team() {
        env::set_var("TEAM1", "user1,user2,user3");
        env::set_var("TEAM2", "user4,user5");

        let teams = "team1,team2,team3";
        let result = get_members_from_teams_string(teams.to_string());

        let mut expected = Vec::new();
        expected.push(SlackUserId("user1".to_string()));
        expected.push(SlackUserId("user2".to_string()));
        expected.push(SlackUserId("user3".to_string()));
        expected.push(SlackUserId("user4".to_string()));
        expected.push(SlackUserId("user5".to_string()));

        assert_eq!(result, expected);

        env::remove_var("TEAM1");
        env::remove_var("TEAM2");
    }

    #[test]
    fn test_string_to_vec() {
        let user_ids = "user1,user2,user3";
        let result = get_members_from_user_ids_string(user_ids.to_string());

        let mut expected = Vec::new();
        expected.push(SlackUserId("user1".to_string()));
        expected.push(SlackUserId("user2".to_string()));
        expected.push(SlackUserId("user3".to_string()));

        assert_eq!(result, expected);
    }
}