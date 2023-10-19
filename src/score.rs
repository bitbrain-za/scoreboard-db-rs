#[cfg(feature = "database")]
use mysql::*;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Score {
    pub name: String,
    pub command: String,
    pub time_ns: f64,
    #[serde(skip)]
    pub hash: String,
    pub language: String,
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ran {} ({}) in {}",
            self.name,
            self.command,
            self.language,
            NiceTime::new(self.time_ns)
        )
    }
}

impl Score {
    pub fn new(name: &str, command: &str, time_ns: f64, hash: String, language: &str) -> Self {
        Score {
            name: name.to_string(),
            command: command.to_string(),
            time_ns,
            hash,
            language: language.to_string(),
        }
    }
    pub fn schema() -> String {
        String::from(
            r"
            id INT NOT NULL AUTO_INCREMENT,
            name TEXT NOT NULL,
            command TEXT NOT NULL,
            time_ns DOUBLE NOT NULL,
            hash TEXT NOT NULL,
            language TEXT NOT NULL,
            PRIMARY KEY (id)
        ",
        )
    }

    #[cfg(feature = "database")]
    pub fn as_insert(&self) -> (String, Params) {
        (self.statement(), self.parameters())
    }

    #[cfg(feature = "database")]
    fn statement(&self) -> String {
        String::from(
            r"
            (name, command, time_ns, hash, language)
            VALUES (:name, :command, :time_ns, :hash, :language)
        ",
        )
    }

    #[cfg(feature = "database")]
    fn parameters(&self) -> Params {
        params! {
            "name" => &self.name,
            "command" => &self.command,
            "time_ns" => &self.time_ns,
            "hash" => &self.hash,
            "language" => &self.language,
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
