
use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();

        // 处理球队1的数据：更新或插入球队1的得分和失球
        scores.entry(team_1_name.clone())
            .and_modify(|t| {
                t.goals_scored += team_1_score;
                t.goals_conceded += team_2_score;
            })
            .or_insert(Team {
                goals_scored: team_1_score,
                goals_conceded: team_2_score,
            });

        // 处理球队2的数据：更新或插入球队2的得分和失球
        scores.entry(team_2_name.clone())
            .and_modify(|t| {
                t.goals_scored += team_2_score;
                t.goals_conceded += team_1_score;
            })
            .or_insert(Team {
                goals_scored: team_2_score,
                goals_conceded: team_1_score,
            });
    }

    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "England,france,4,2\n".to_string()
            + "france,italy,3,1\n"
            + "poland,spain,2,0\n"
            + "germany,england,2,1\n";
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
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5); // 4（对法国） + 1（对德国）=5
        assert_eq!(team.goals_conceded, 4); // 2（对法国） + 2（对德国）=4
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0); // 对波兰未进球
        assert_eq!(team.goals_conceded, 2); // 对波兰失2球
    }
}
