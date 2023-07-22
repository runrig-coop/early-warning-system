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

#[tauri::command]
fn farms() -> Vec<Farm> {
    let mut farms = Vec::new();
    farms.push(Farm {
        id: 0,
        name: "Joe's Farm".to_string(),
        // measurements: Vec::new(),
        timestamp: 20,
        status: Status::Red,
    });
    farms.push(Farm {
        id: 1,
        name: "Sally's Farm".to_string(),
        // measurements: Vec::new(),
        timestamp: 5,
        status: Status::Yellow,
    });
    farms.push(Farm {
        id: 2,
        name: "Joe's Other Farm".to_string(),
        // measurements: Vec::new(),
        timestamp: 10,
        status: Status::Yellow,
    });
    farms.push(Farm {
        id: 3,
        name: "Sam's Farm".to_string(),
        // measurements: Vec::new(),
        timestamp: 1,
        status: Status::Green,
    });
    farms
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

#[derive(Clone, Serialize, Deserialize)]
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![farms])
        .invoke_handler(tauri::generate_handler![save_sync])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
