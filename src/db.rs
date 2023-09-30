use crate::score::Score;
use log::debug;
use mysql::prelude::*;
use mysql::*;

pub struct Db {
    connection: PooledConn,
    table: String,
}

impl Db {
    pub fn new(
        host: &str,
        port: u16,
        db: &str,
        password: &str,
        table: &str,
    ) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let url = format!("{}:{}/{}", host, port, db);
        let conn = Self::create_connection(password, &url)?;

        let mut me = Db {
            connection: conn,
            table: table.to_string(),
        };

        me.create_table()?;

        Ok(me)
    }

    fn create_connection(
        password: &str,
        url: &str,
    ) -> Result<PooledConn, Box<dyn std::error::Error>> {
        let pool = Pool::new(format!("mysql://root:{}@{}", password, url).as_str())?;
        Ok(pool.get_conn()?)
    }

    fn create_table(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let query = format!(
            "CREATE TABLE if not exists {} ( {} )",
            self.table,
            Score::schema()
        );
        self.connection.query_drop(query)?;
        Ok(())
    }

    pub fn insert_score(
        &mut self,
        name: &str,
        command: &str,
        time_ns: i32,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let score: Score = Score::new(name, command, time_ns);
        let (statement, parameters) = score.as_insert();
        let statement = format!("INSERT INTO {} {}", self.table, statement);
        self.connection.exec_drop(statement, parameters)?;
        Ok(())
    }

    pub fn get_scores(&mut self) -> std::result::Result<Vec<Score>, Box<dyn std::error::Error>> {
        let query = format!(
            "SELECT name, command, time_ns FROM {} ORDER BY time_ns ASC",
            self.table
        );
        let scores = self
            .connection
            .query_map(&query, |(name, command, time_ns)| Score {
                name,
                command,
                time_ns,
            })?;

        debug!("Scores: {:?}", scores);
        Ok(scores)
    }

    pub fn clear_table(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let drop = format!("DROP TABLE IF EXISTS {}", self.table);
        self.connection.query_drop(drop.as_str())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_db() {
        let db_pass = env::var("DB_PASS").expect("$DB_PASS must be set");

        let mut db = Db::new("localhost", 3306, "code_challenge", &db_pass, "test").unwrap();
        db.clear_table().unwrap();
        db.create_table().unwrap();
        db.insert_score("test", "echo", 1).unwrap();
        let scores = db.get_scores().unwrap();
        assert_eq!(scores.len(), 1);

        db.insert_score("test", "echo", 2).unwrap();
        let scores = db.get_scores().unwrap();
        assert_eq!(scores.len(), 2);
        assert_eq!(scores[0].time_ns, 1);
        assert_eq!(scores[1].time_ns, 2);
    }
}
