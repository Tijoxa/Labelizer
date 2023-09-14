#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::{display::_display_images, file::_choose_folder};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn choose_folder() -> Vec<String> {
    _choose_folder().unwrap()
}

#[tauri::command]
fn display_images() {
    _display_images()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, choose_folder, display_images])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
