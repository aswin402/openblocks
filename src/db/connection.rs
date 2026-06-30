use rusqlite::Connection;
use std::path::Path;

pub fn open_connection(db_path: &str) -> rusqlite::Result<Connection> {
    let conn = if db_path == ":memory:" {
        Connection::open_in_memory()?
    } else {
        // Ensure parent directory exists
        if let Some(parent) = Path::new(db_path).parent() {
            std::fs::create_dir_all(parent).ok();
        }
        Connection::open(db_path)?
    };

    // Enable WAL mode for concurrent read/write
    conn.pragma_update(None, "journal_mode", "WAL")?;
    // Enable foreign keys
    conn.pragma_update(None, "foreign_keys", "ON")?;

    Ok(conn)
}
