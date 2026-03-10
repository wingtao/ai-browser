use rusqlite::{params, Connection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryItem {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub visited_at: i64,
}

pub struct HistoryRepository {
    conn: Connection,
}

impl HistoryRepository {
    pub fn in_memory() -> anyhow::Result<Self> {
        let conn = Connection::open_in_memory()?;
        let repo = Self { conn };
        repo.init_schema()?;
        Ok(repo)
    }

    pub fn open(path: &str) -> anyhow::Result<Self> {
        let conn = Connection::open(path)?;
        let repo = Self { conn };
        repo.init_schema()?;
        Ok(repo)
    }

    fn init_schema(&self) -> anyhow::Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS history(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT NOT NULL,
                title TEXT NOT NULL,
                visited_at INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_history_visited_at ON history(visited_at DESC);
        "#,
        )?;
        Ok(())
    }

    pub fn add_visit(&self, url: &str, title: &str, visited_at: i64) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO history(url, title, visited_at) VALUES (?1, ?2, ?3)",
            params![url, title, visited_at],
        )?;
        Ok(())
    }

    pub fn list_recent(&self, limit: usize) -> anyhow::Result<Vec<HistoryItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, url, title, visited_at FROM history ORDER BY visited_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok(HistoryItem {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                visited_at: row.get(3)?,
            })
        })?;
        Ok(rows.filter_map(Result::ok).collect())
    }

    pub fn clear(&self) -> anyhow::Result<()> {
        self.conn.execute("DELETE FROM history", [])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crud_history() {
        let repo = HistoryRepository::in_memory().unwrap();
        repo.add_visit("https://a.com", "A", 10).unwrap();
        repo.add_visit("https://b.com", "B", 20).unwrap();
        let list = repo.list_recent(10).unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].url, "https://b.com");
        repo.clear().unwrap();
        assert!(repo.list_recent(10).unwrap().is_empty());
    }
}
