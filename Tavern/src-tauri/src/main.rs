// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::{Manager, State};
use tokio::sync::Mutex;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod commands;
mod database;
mod networking;
mod dice;
mod assets;
mod state;
mod config;
mod errors;
mod utils;

use crate::database::DatabaseManager;
use crate::networking::NetworkManager;
use crate::state::AppState;
use crate::config::AppConfig;
use crate::errors::AppError;

// Application state type
type AppStateType = Arc<Mutex<AppState>>;
type DatabaseType = Arc<Mutex<DatabaseManager>>;
type NetworkType = Arc<Mutex<NetworkManager>>;

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

    info!("Starting D&D VTT Application");

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
        ])
        .setup(|app| {
            // Setup application window
            let window = app.get_window("main").unwrap();
            
            // Set minimum window size
            let _ = window.set_min_size(Some(tauri::LogicalSize::new(1000.0, 700.0)));
            
            // Center window on screen
            let _ = window.center();
            
            // Setup window event listeners
            let app_handle = app.handle();
            window.on_window_event(move |event| {
                match event {
                    tauri::WindowEvent::CloseRequested { .. } => {
                        info!("Application closing");
                        // Perform cleanup here
                        let app_handle = app_handle.clone();
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
                    _ => {}
                }
            });
            
            Ok(())
        })
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "quit" => {
                    std::process::exit(0);
                }
                "about" => {
                    // Show about dialog
                }
                _ => {}
            }
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

// Application menu setup
fn create_menu() -> tauri::Menu {
    let quit = tauri::CustomMenuItem::new("quit".to_string(), "Quit").accelerator("CmdOrCtrl+Q");
    let about = tauri::CustomMenuItem::new("about".to_string(), "About");
    let new_campaign = tauri::CustomMenuItem::new("new_campaign".to_string(), "New Campaign").accelerator("CmdOrCtrl+N");
    let open_campaign = tauri::CustomMenuItem::new("open_campaign".to_string(), "Open Campaign").accelerator("CmdOrCtrl+O");
    let save_campaign = tauri::CustomMenuItem::new("save_campaign".to_string(), "Save Campaign").accelerator("CmdOrCtrl+S");
    
    let file_menu = tauri::Submenu::new("File", tauri::Menu::new()
        .add_item(new_campaign)
        .add_item(open_campaign)
        .add_item(save_campaign)
        .add_native_item(tauri::MenuItem::Separator)
        .add_item(quit));
    
    let help_menu = tauri::Submenu::new("Help", tauri::Menu::new().add_item(about));
    
    tauri::Menu::new()
        .add_submenu(file_menu)
        .add_submenu(help_menu)
}

// System tray setup (optional)
fn create_system_tray() -> tauri::SystemTray {
    let quit = tauri::CustomMenuItem::new("quit".to_string(), "Quit");
    let show = tauri::CustomMenuItem::new("show".to_string(), "Show");
    let tray_menu = tauri::SystemTrayMenu::new()
        .add_item(show)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(quit);
    
    tauri::SystemTray::new().with_menu(tray_menu)
}

// Handle system tray events
fn handle_system_tray_event(app: &tauri::AppHandle, event: tauri::SystemTrayEvent) {
    match event {
        tauri::SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let window = app.get_window("main").unwrap();
            let _ = window.show();
            let _ = window.set_focus();
        }
        tauri::SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
                _ => {}
            }
        }
        _ => {}
    }
}