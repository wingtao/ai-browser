use rusqlite::{params, Connection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bookmark {
    pub id: i64,
    pub title: String,
    pub url: String,
}

pub struct BookmarkRepository {
    conn: Connection,
}

impl BookmarkRepository {
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
            CREATE TABLE IF NOT EXISTS bookmarks(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                url TEXT NOT NULL UNIQUE
            );
            CREATE INDEX IF NOT EXISTS idx_bookmarks_title ON bookmarks(title);
        "#,
        )?;
        Ok(())
    }

    pub fn add(&self, title: &str, url: &str) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO bookmarks(title, url) VALUES (?1, ?2)",
            params![title, url],
        )?;
        Ok(())
    }

    pub fn remove_by_url(&self, url: &str) -> anyhow::Result<()> {
        self.conn
            .execute("DELETE FROM bookmarks WHERE url = ?1", params![url])?;
        Ok(())
    }

    pub fn list(&self) -> anyhow::Result<Vec<Bookmark>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, url FROM bookmarks ORDER BY id DESC")?;
        let rows = stmt.query_map([], |row| {
            Ok(Bookmark {
                id: row.get(0)?,
                title: row.get(1)?,
                url: row.get(2)?,
            })
        })?;
        Ok(rows.filter_map(Result::ok).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crud_bookmark() {
        let repo = BookmarkRepository::in_memory().unwrap();
        repo.add("A", "https://a.com").unwrap();
        repo.add("B", "https://b.com").unwrap();
        let list = repo.list().unwrap();
        assert_eq!(list.len(), 2);
        repo.remove_by_url("https://a.com").unwrap();
        let list = repo.list().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].url, "https://b.com");
    }
}
