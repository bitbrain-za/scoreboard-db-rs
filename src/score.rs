use mysql::*;
use std::fmt::Display;

#[derive(Debug)]
pub struct Score {
    pub name: String,
    pub command: String,
    pub time_ns: f64,
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elapsed = self.time_ns / 1_000_000.0;
        write!(f, "{} ran {} in {}ms", self.name, self.command, elapsed)
    }
}

impl Score {
    pub fn new(name: &str, command: &str, time_ns: f64) -> Self {
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
            time_ns DOUBLE NOT NULL,
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
