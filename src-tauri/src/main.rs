// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use std::fs::{DirBuilder, OpenOptions};
use std::io::prelude::*;
use early_warning_system::data_model::{Farm, Status};
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

impl SavedState {
    fn default() -> SavedState {
        SavedState { farms: vec![] }
    }
}

// A temporary helper to open the directory safely and also confine the hard-
// coded path string to a single function.
fn path_to_cache(handle: tauri::AppHandle) -> std::path::PathBuf {
    let cache_path = handle
        .path_resolver()
        .resolve_resource(".cache/farms.json")
        .expect("Failed to resolve path to cache.");
    DirBuilder::new()
        .recursive(true)
        .create(cache_path.parent().unwrap())
        .unwrap_or_default();
    cache_path
}

#[tauri::command]
fn save(handle: tauri::AppHandle, farms: Vec<Farm>) {
    let save_state = SavedState {farms};
    let output_text = serde_json::to_string_pretty(&save_state).unwrap();
    let cache_path = path_to_cache(handle);
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(cache_path)
        .expect("Failed to open cache while saving");
    file.write_all(output_text.as_bytes()).expect("Failed to write data to file");
}

#[tauri::command]
fn load(handle: tauri::AppHandle) -> Vec<Farm> {
    let cache_path = path_to_cache(handle);
    let cache = OpenOptions::new()
        .read(true)
        .open(cache_path);
    let save_state: SavedState = match cache {
        Ok(file) => match serde_json::from_reader(file) {
            Ok(state) => state,
            Err(_) => SavedState::default(),
        },
        Err(_) => SavedState::default(),
    };
    save_state.farms
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save, load])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
