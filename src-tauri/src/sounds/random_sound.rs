use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use rand::RngExt;
use rodio::Player;

pub fn random_sound(directory: &str) -> PathBuf {
    let path = fs::read_dir(directory).unwrap();

    let files: Vec<PathBuf> = path
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();

    if files.is_empty() {
        return PathBuf::new();
    }

    let index = rand::rng().random_range(0..files.len());

    files[index].clone()
}