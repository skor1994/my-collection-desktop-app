// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod database;

use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_item_collections(app_handle: AppHandle) -> Vec<database::ItemCollection> {
    let items = app_handle.db(|db| database::get_all(db)).unwrap();
    return items
}

#[tauri::command]
fn create_item_collection(app_handle: AppHandle) -> database::ItemCollection {
    app_handle.db(|db| database::add_new_item_col(db)).unwrap();
    let entry = app_handle.db(|db| database::get_last_collection(db)).unwrap();

    return entry
}

#[tauri::command]
fn create_item(app_handle: AppHandle, id: i16) -> database::Item {
    app_handle.db(|db| database::add_new_item_to_col(db, &id)).unwrap();
    let entry = app_handle.db(|db| database::get_last_item(db)).unwrap();

    return entry
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { db: Default::default() })
        .invoke_handler(tauri::generate_handler![get_item_collections, create_item_collection, create_item])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = database::initialize_database().expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");    
}