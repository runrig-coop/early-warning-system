// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use rgo_early_warning_system::data_model::{Farm, Status};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, Clone)]
enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone)]
enum SaveError {
    File,
    Write,
    Format,
}

#[derive(Clone, Serialize, Deserialize, Default)]
struct SavedState {
    farms: Vec<Farm>
}

fn path() -> std::path::PathBuf {
    let mut path = std::env::current_dir().unwrap_or_default();
    path.push("rgo_early_warning.json");
    path
}

#[tauri::command]
fn save_sync(data: SavedState) {
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path())
        .expect("Couldn't open file");
    serde_json::to_writer_pretty(f, &data).unwrap();
}

#[tauri::command]
fn load_sync() -> Vec<Farm> {
    let try_file = std::fs::File::open(path());
    let save_state:SavedState = match try_file{
        Ok(file) => serde_json::from_reader(file).expect("Could not read values."),
        Err(e)   => SavedState::default(),
    };
    save_state.farms
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![save_sync])
        .invoke_handler(tauri::generate_handler![load_sync])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
