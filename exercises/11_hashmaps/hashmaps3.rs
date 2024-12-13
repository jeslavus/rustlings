use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<&str, TeamScores> = HashMap::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // Update team 1's scores
        scores.entry(team_1_name)
            .or_insert_with(TeamScores::default)
            .goals_scored += team_1_score;

        scores.entry(team_1_name)
            .or_insert_with(TeamScores::default)
            .goals_conceded += team_2_score;

        // Update team 2's scores
        scores.entry(team_2_name)
            .or_insert_with(TeamScores::default)
            .goals_scored += team_2_score;

        scores.entry(team_2_name)
            .or_insert_with(TeamScores::default)
            .goals_conceded += team_1_score;
    }

    scores
}

fn main() {
    // Example usage
    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    let scores = build_scores_table(RESULTS);
    for (team, stats) in scores {
        println!(
            "{}: scored {}, conceded {}",
            team, stats.goals_scored, stats.goals_conceded
        );
    }
}
