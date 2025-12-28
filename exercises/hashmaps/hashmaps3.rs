use std::collections::HashMap;

fn build_scores(results: Vec<(&str, i32)>) -> HashMap<String, i32> {
    let mut scores = HashMap::new();
    for (team, score) in results {
        // 首字母大写，其余小写
        let team_normalized = team
            .to_lowercase()
            .chars()
            .nth(0)
            .unwrap()
            .to_uppercase()
            .chain(team.to_lowercase().chars().skip(1))
            .collect::<String>();

        *scores.entry(team_normalized).or_insert(0) += score;
    }
    scores
}

fn get_score(scores: &HashMap<String, i32>, team: &str) -> i32 {
    let team_normalized = team
        .to_lowercase()
        .chars()
        .nth(0)
        .unwrap()
        .to_uppercase()
        .chain(team.to_lowercase().chars().skip(1))
        .collect::<String>();

    scores.get(&team_normalized).copied().unwrap_or(0)
}