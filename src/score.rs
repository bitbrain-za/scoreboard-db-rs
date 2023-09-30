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
        write!(
            f,
            "{} ran {} in {}",
            self.name,
            self.command,
            NiceTime::new(self.time_ns)
        )
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

pub struct NiceTime {
    pub time_ns: f64,
}

impl NiceTime {
    pub fn new(time_ns: f64) -> Self {
        NiceTime { time_ns }
    }
}

impl Display for NiceTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.time_ns < 1_000.0 {
            write!(f, "{:.3}ns", self.time_ns)
        } else if self.time_ns < 1_000_000.0 {
            write!(f, "{:.3}us", self.time_ns / 1_000.0)
        } else if self.time_ns < 1_000_000_000.0 {
            write!(f, "{:.3}ms", self.time_ns / 1_000_000.0)
        } else {
            write!(f, "{:.3}s", self.time_ns / 1_000_000_000.0)
        }
    }
}
