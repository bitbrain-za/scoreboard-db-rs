use crate::score::Score;
use log::trace;
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
        score: &Score,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let (statement, parameters) = score.as_insert();
        let statement = format!("INSERT INTO {} {}", self.table, statement);
        self.connection.exec_drop(statement, parameters)?;
        Ok(())
    }

    pub fn get_scores(
        &mut self,
        count: Option<usize>,
    ) -> std::result::Result<Vec<Score>, Box<dyn std::error::Error>> {
        let query = format!(
            "SELECT language, hash, name, command, time_ns FROM {} ORDER BY time_ns ASC",
            self.table
        );

        let query = if let Some(count) = count {
            format!("{} LIMIT {}", query, count)
        } else {
            query
        };

        let scores =
            self.connection
                .query_map(&query, |(language, hash, name, command, time_ns)| Score {
                    name,
                    command,
                    time_ns,
                    hash,
                    language,
                })?;

        trace!("Scores: {:?}", scores);
        Ok(scores)
    }

    pub fn clear_table(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let drop = format!("DROP TABLE IF EXISTS {}", self.table);
        self.connection.query_drop(drop.as_str())?;
        Ok(())
    }
}
