use rusqlite::{params, Connection};
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub static DB: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open("honeypot.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS attempts (
            id INTEGER PRIMARY KEY,
            timestamp TEXT,
            ip TEXT,
            country TEXT,
            username TEXT,
            password TEXT,
            command TEXT
        )",
        [],
    )
    .unwrap();

    Mutex::new(conn)
});

pub fn log_attempt(
    timestamp: &str,
    ip: &str,
    country: &str,
    username: &str,
    password: &str,
    command: &str,
) {
    let conn = DB.lock().unwrap();

    conn.execute(
        "INSERT INTO attempts
        (timestamp, ip, country, username, password, command)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![timestamp, ip, country, username, password, command],
    )
    .unwrap();
}
