// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
/*
use std::sync::Arc;
use tauri::{Manager, State, AppHandle, WebviewWindow};
use tokio::sync::Mutex;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod commands;
mod database;
mod network;
mod dice;
mod assets;
mod state;
mod config;
mod errors;
mod utils;

use crate::database::DatabaseManager;
use crate::network::NetworkManager;
use crate::state::AppState;
use crate::config::AppConfig;
use crate::errors::AppError;

// Application state type
type AppStateType = Arc<Mutex<AppState>>;
type DatabaseType = Arc<Mutex<DatabaseManager>>;
type NetworkType = Arc<Mutex<NetworkManager>>;
*/
fn main() {
    // Initialize the Tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Add your command handlers here
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "dnd_vtt=debug,tauri=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Tavern");

    // Load configuration
    let config = AppConfig::load().await?;
    info!("Configuration loaded");

    // Initialize database
    let database = DatabaseManager::new(&config.database_path).await?;
    database.run_migrations().await?;
    info!("Database initialized");

    // Initialize networking
    let network_manager = NetworkManager::new(config.networking.clone()).await?;
    info!("Network manager initialized");

    // Initialize application state
    let app_state = AppState::new();
    info!("Application state initialized");

    // Build Tauri application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .manage(Arc::new(Mutex::new(app_state)))
        .manage(Arc::new(Mutex::new(database)))
        .manage(Arc::new(Mutex::new(network_manager)))
        .manage(config)
        .invoke_handler(tauri::generate_handler![
            // Campaign commands
            commands::create_campaign,
            commands::get_campaigns,
            commands::get_campaign,
            commands::update_campaign,
            commands::delete_campaign,
            
            // Character commands
            commands::create_character,
            commands::get_characters,
            commands::get_character,
            commands::update_character,
            commands::delete_character,
            
            // Map commands
            commands::create_map,
            commands::get_maps,
            commands::load_map,
            commands::save_map_state,
            
            // Token commands
            commands::create_token,
            commands::update_token_position,
            commands::delete_token,
            
            // Dice commands
            commands::roll_dice,
            commands::roll_initiative,
            
            // Network commands
            commands::create_room,
            commands::join_room,
            commands::leave_room,
            commands::get_peers,
            commands::send_message,
            
            // Asset commands
            commands::import_asset,
            commands::get_assets,
            commands::delete_asset,
            
            // Settings commands
            commands::get_settings,
            commands::update_settings,
            
            // Utility commands
            commands::get_app_info,
            commands::export_campaign,
            commands::import_campaign,
            
            // Additional V2 utility commands
            handle_menu_action,
            show_main_window,
            get_app_version,
            restart_app,
        ])
        .setup(|app| {
            // Setup application window
            let window = app.get_webview_window("main").unwrap();
            
            // Set minimum window size using the new API
            let _ = window.set_min_size(Some(tauri::LogicalSize::new(1000.0, 700.0)));
            
            // Center window on screen
            let _ = window.center();
            
            // Window event handling is now done differently in V2
            setup_window_events(app.handle(), &window);
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

// Global error handler
fn handle_error(error: AppError) {
    error!("Application error: {}", error);
    // Could also show user-facing error dialog here
}

// Note: In Tauri V2, native menus are handled differently
// You'll need to use the menu plugin or implement menus via the frontend
// Here's an example of how you might handle menu actions via commands instead:

#[tauri::command]
async fn handle_menu_action(action: String, app_handle: AppHandle) -> Result<(), String> {
    match action.as_str() {
        "quit" => {
            app_handle.exit(0);
            Ok(())
        }
        "new_campaign" => {
            // Emit event to frontend to show new campaign dialog
            if let Some(window) = app_handle.get_webview_window("main") {
                let _ = window.emit("menu-action", "new_campaign");
            }
            Ok(())
        }
        "open_campaign" => {
            // Emit event to frontend to show open campaign dialog
            if let Some(window) = app_handle.get_webview_window("main") {
                let _ = window.emit("menu-action", "open_campaign");
            }
            Ok(())
        }
        "save_campaign" => {
            // Emit event to frontend to trigger save
            if let Some(window) = app_handle.get_webview_window("main") {
                let _ = window.emit("menu-action", "save_campaign");
            }
            Ok(())
        }
        "about" => {
            // Show about dialog via frontend
            if let Some(window) = app_handle.get_webview_window("main") {
                let _ = window.emit("menu-action", "about");
            }
            Ok(())
        }
        _ => Err(format!("Unknown menu action: {}", action))
    }
}

// Example of how to handle app events in V2
#[tauri::command]
async fn show_main_window(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

// Additional utility commands for V2
#[tauri::command]
async fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
async fn restart_app(app_handle: AppHandle) {
    app_handle.restart();
}

// Setup window event listeners (V2 approach)
fn setup_window_events(app_handle: AppHandle, window: &WebviewWindow) {
    let app_handle_clone = app_handle.clone();
    
    window.on_window_event(move |event| {
        match event {
            tauri::WindowEvent::CloseRequested { .. } => {
                info!("Application closing");
                // Perform cleanup here
                let app_handle = app_handle_clone.clone();
                tauri::async_runtime::spawn(async move {
                    if let Some(db) = app_handle.try_state::<DatabaseType>() {
                        if let Ok(db) = db.lock().await {
                            let _ = db.close().await;
                        }
                    }
                    if let Some(network) = app_handle.try_state::<NetworkType>() {
                        if let Ok(network) = network.lock().await {
                            let _ = network.shutdown().await;
                        }
                    }
                });
            }
            tauri::WindowEvent::Destroyed => {
                info!("Window destroyed");
            }
            _ => {}
        }
    });
}

// Example of how to handle app events in V2
#[tauri::command]
async fn show_main_window(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

// Additional utility commands for V2
#[tauri::command]
async fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
async fn restart_app(app_handle: AppHandle) {
    app_handle.restart();
}
*/