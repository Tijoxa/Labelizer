#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::file::layout;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn choose_folder() -> bool {
    layout()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![choose_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
