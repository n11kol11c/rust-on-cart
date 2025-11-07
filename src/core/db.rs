use crate::errors::error::CartError;

pub trait Database {
    fn connect(&self) -> Result<(), CartError>;
    fn execute(&self, query: &str) -> Result<(), CartError>;
    fn query(&self, query: &str) -> Result<Vec<Vec<String>>, CartError>;
}

pub struct SqliteDB {
    pub path: String,
}

impl Database for SqliteDB {
    fn connect(&self) -> Result<(), CartError> {
        let _conn = rusqlite::Connection::open(&self.path)
            .map_err(|e| CartError::DbError(e.to_string()))?;
        Ok(())
    }

    fn execute(&self, query: &str) -> Result<(), CartError> {
        let conn = rusqlite::Connection::open(&self.path)
            .map_err(|e| CartError::DbError(e.to_string()))?;
        conn.execute(query, []).map_err(|e| CartError::DbError(e.to_string()))?;
        Ok(())
    }

    fn query(&self, query: &str) -> Result<Vec<Vec<String>>, CartError> {
        let conn = rusqlite::Connection::open(&self.path)
            .map_err(|e| CartError::DbError(e.to_string()))?;
        let mut stmt = conn.prepare(query).map_err(|e| CartError::DbError(e.to_string()))?;
        let rows = stmt.query_map([], |row| {
            let mut v = Vec::new();
            for i in 0..row.column_count() {
                let s: String = row.get(i)?;
                v.push(s);
            }
            Ok(v)
        }).map_err(|e| CartError::DbError(e.to_string()))?;
        let result = rows.map(|r| r.unwrap()).collect();
        Ok(result)
    }
}
