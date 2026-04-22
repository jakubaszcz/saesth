use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use rusqlite::Connection;
use rusqlite::fallible_iterator::FallibleIterator;
use std::sync::{Mutex, OnceLock};

static DATABASE: OnceLock<Mutex<Connection>> = OnceLock::new();

fn db() -> std::sync::MutexGuard<'static, Connection> {
    DATABASE
        .get()
        .unwrap()
        .lock()
        .unwrap()
}

fn get_database_path() -> PathBuf {
    let directory = ProjectDirs::from("com", "saesth", "saesth").unwrap();

    let local = directory.data_dir();
    fs::create_dir_all(local).unwrap();


    local.join("database.db")
}

pub fn init_db() {
    let path = get_database_path();
    let conn = Connection::open(path).unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS sounds (
            id TEXT PRIMARY KEY,
            volume REAL
        )
        ",[]).unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS settings (
            id TEXT PRIMARY KEY,
            value TEXT
        )", []).unwrap();

    DATABASE.set(Mutex::new(conn)).unwrap();
}

pub fn create_if_missing(sound: &str) {
    let conn = db();

    conn.execute(
        "INSERT OR IGNORE INTO sounds (id, volume) VALUES (?1, 0.5)",
        [sound],
    ).unwrap();
}

pub fn get_volume(sound: &str) -> f32 {
    let conn = db();

    conn.query_row(
        "SELECT volume FROM sounds WHERE id = ?1",
        [sound],
        |row| row.get(0),
    ).unwrap_or(0.5)
}

pub fn set_volume(sound: &str, volume: f32) {
    let conn = db();

    println!("Setting volume of {} to {}", sound, volume);

    let updated = conn.execute(
        "UPDATE sounds SET volume = ?1 WHERE id = ?2",
        rusqlite::params![volume, sound],
    ).unwrap();

    println!("Rows updated: {}", updated);
}