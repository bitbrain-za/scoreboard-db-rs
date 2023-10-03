use crate::filter;
use crate::score::{NiceTime, Score};
use std::process::Command;

pub struct ScoreBoard {
    scores: Vec<Score>,
}

impl ScoreBoard {
    pub fn new(scores: Vec<Score>) -> Self {
        ScoreBoard { scores }
    }

    pub fn filter(self, filters: filter::Builder) -> Self {
        let filters = filters.build();
        let scores = filters.apply(&self.scores);
        ScoreBoard { scores }
    }

    pub fn get(&self, filters: Option<&filter::Collection>) -> Vec<Score> {
        match filters {
            Some(filters) => filters.apply(&self.scores),
            None => self.scores.clone(),
        }
    }

    pub fn display(&self, filters: Option<&filter::Collection>) -> String {
        let scores = self.get(filters);
        let mut out: String = String::new();
        for (i, score) in scores.iter().enumerate() {
            out.push_str(&format!("{}. {}\n", i + 1, score));
        }
        out
    }

    pub fn display_with_real_name(&self, filters: Option<&filter::Collection>) -> String {
        let scores = self.get(filters);
        let mut out: String = String::new();
        for (i, score) in scores.iter().enumerate() {
            out.push_str(&format!("{}. {}\n", i + 1, Self::display_real_name(score),));
        }
        out
    }

    fn display_real_name(score: &Score) -> String {
        let name = Self::get_real_name(&score.name);
        format!(
            "{} ran {} in {}",
            name,
            score.command,
            NiceTime::new(score.time_ns)
        )
    }

    fn get_real_name(name: &str) -> String {
        let output = match Command::new("getent").arg("passwd").arg(name).output() {
            Ok(output) => output,
            Err(_) => return name.to_string(),
        };

        let find_name = || -> Result<String, Box<dyn std::error::Error>> {
            let output = String::from_utf8_lossy(&output.stdout);
            let output = output.split(':').collect::<Vec<&str>>();
            let output = output.get(4).ok_or("no name")?.to_string();
            let output = output
                .split(',')
                .collect::<Vec<&str>>()
                .first()
                .ok_or("err")?
                .to_string();
            Ok(output)
        };
        find_name().unwrap_or(name.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::{Builder, Filter, SortColumn};

    #[test]
    fn test_scoreboard() {
        let scores = vec![
            Score::new("foo", "echo foo", 1.0, "hash".to_string()),
            Score::new("bar", "echo bar", 2.0, "hash".to_string()),
            Score::new("baz", "echo baz", 3.0, "hash".to_string()),
        ];
        let scoreboard = ScoreBoard::new(scores);
        let filters = Builder::new()
            .add_filter(Filter::Player(vec!["foo".to_string()]))
            .build();
        let scores = scoreboard.get(Some(&filters));
        assert_eq!(scores.len(), 1);
        assert_eq!(scores[0].name, "foo");
    }

    #[test]
    fn test_scoreboard_display() {
        let scores = vec![
            Score::new("foo", "echo foo", 1.0, "hash".to_string()),
            Score::new("bar", "echo bar", 2.0, "hash".to_string()),
            Score::new("baz", "echo baz", 3.0, "hash".to_string()),
        ];
        let scoreboard = ScoreBoard::new(scores);
        let filters = Builder::new()
            .add_filter(Filter::Player(vec!["foo".to_string()]))
            .add_filter(Filter::Sort(SortColumn::Time))
            .build();
        let scores = scoreboard.display(Some(&filters));
        assert_eq!(scores, "1. foo ran echo foo in 1.000ns\n");
    }

    #[test]
    fn test_scoreboard_display_with_real_name() {
        let scores = vec![
            Score::new("foo", "echo foo", 1.0, "hash".to_string()),
            Score::new("bar", "echo bar", 2.0, "hash".to_string()),
            Score::new("baz", "echo baz", 3.0, "hash".to_string()),
        ];
        let scoreboard = ScoreBoard::new(scores);
        let filters = Builder::new()
            .add_filter(Filter::Player(vec!["foo".to_string()]))
            .build();
        let scores = scoreboard.display_with_real_name(Some(&filters));
        assert_eq!(scores, "1. foo ran echo foo in 1.000ns\n");
    }
}
