use mysql::*;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Score {
    pub name: String,
    pub command: String,
    pub time_ns: i32,
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ran {} in {}ns",
            self.name, self.command, self.time_ns
        )
    }
}

impl Score {
    pub fn new(name: &str, command: &str, time_ns: i32) -> Self {
        Score {
            name: name.to_string(),
            command: command.to_string(),
            time_ns,
        }
    }
    pub fn schema() -> String {
        String::from(
            r"
            id INT NOT NULL AUTO_INCREMENT,
            name TEXT NOT NULL,
            command TEXT NOT NULL,
            time_ns INT NOT NULL,
            PRIMARY KEY (id)
        ",
        )
    }

    pub fn as_insert(&self) -> (String, Params) {
        (self.statement(), self.parameters())
    }

    fn statement(&self) -> String {
        String::from(
            r"
            (name, command, time_ns)
            VALUES (:name, :command, :time_ns)
        ",
        )
    }

    fn parameters(&self) -> Params {
        params! {
            "name" => &self.name,
            "command" => &self.command,
            "time_ns" => &self.time_ns,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        let score = Score::new("test", "echo", 1);
        assert_eq!(
            score,
            Score {
                name: String::from("test"),
                command: String::from("echo"),
                time_ns: 1,
            }
        );
    }

    #[test]
    fn test_score_schema() {
        let schema = Score::schema();
        assert_eq!(
            schema,
            String::from(
                r"
            id INT NOT NULL AUTO_INCREMENT,
            name TEXT NOT NULL,
            command TEXT NOT NULL,
            time_ns INT NOT NULL,
            PRIMARY KEY (id)
        ",
            )
        );
    }

    #[test]
    fn test_score_as_insert() {
        let score = Score::new("test", "echo", 1);
        let (statement, parameters) = score.as_insert();
        assert_eq!(
            statement,
            String::from(
                r"
            (name, command, time_ns)
            VALUES (:name, :command, :time_ns)
        ",
            )
        );
        assert_eq!(
            parameters,
            params! {
                "name" => "test",
                "command" => "echo",
                "time_ns" => 1,
            }
        );
    }
}
