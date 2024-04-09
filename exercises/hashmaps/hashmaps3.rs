// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Debug, Copy, Clone)]
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

impl Team {
    fn new(goals_scored: u8, goals_conceded: u8) -> Team {
        Team {
            goals_scored,
            goals_conceded,
        }
    }

    fn add(&mut self, goals_scored: u8, goals_conceded: u8) -> Team {
        self.goals_scored += goals_scored;
        self.goals_conceded += goals_conceded;
        *self
    }
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let mut v = r.split(',');
        if v.clone().count() != 4 {
            panic!("parse line error: {}", r);
        }

        let team_1_name = v.next().unwrap().to_string();
        let team_2_name = v.next().unwrap().to_string();
        let team_1_score: u8 = v.next().unwrap().parse().unwrap();
        let team_2_score: u8 = v.next().unwrap().parse().unwrap();

        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.
        scores.entry(team_1_name.clone())
            .and_modify(|t| {
                t.add(team_1_score, team_2_score);
            }).or_insert(Team::new(team_1_score, team_2_score));

        scores.entry(team_2_name.clone())
            .and_modify(|t| {
                t.add(team_2_score, team_1_score);
            }).or_insert(Team::new(team_2_score, team_1_score));
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        println!("score: {:?}", scores);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
