#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, WebviewWindow, Emitter};
use tracing::{info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tavern=debug,tauri=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_path = "data/tavern.db"; // customize path as needed
    let db = DatabaseManager::new(db_path)
        .await
        .expect("Failed to initialize database");
    db.run_migrations().await.expect("Migration failed");

    let database = Arc::new(Mutex::new(db));
    let app_state = Arc::new(Mutex::new(AppState::default())); // Ensure AppState::default() exists


    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            get_app_version,
            restart_app,
            handle_menu_action,
            show_main_window,
            create_campaign,
            get_campaigns,
            get_campaign,
            update_campaign,
            delete_campaign,
            create_character,
            get_characters,
            get_character,
            update_character,
            delete_character,
            create_map,
            get_maps,
            load_map,
            save_map_state,
            create_token,
            update_token_position,
            delete_token,
            roll_dice,
            roll_initiative
        ])
        .setup(|app| {
            // Window setup
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_min_size(Some(tauri::LogicalSize::new(1000.0, 700.0)));
                let _ = window.center();
                setup_window_events(app.handle().clone(), &window);

            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}

// Example utility commands
#[tauri::command]
async fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
async fn restart_app(app_handle: AppHandle) {
    app_handle.restart();
}

#[tauri::command]
async fn handle_menu_action(action: String, app_handle: AppHandle) -> Result<(), String> {
    match action.as_str() {
        "quit" => {
            app_handle.exit(0);
            Ok(())
        }
        _ => {
            if let Some(window) = app_handle.get_webview_window("main") {
                let _ = window.emit("menu-action", action);
            }
            Ok(())
        }
    }
}

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

fn setup_window_events(_app_handle: AppHandle, window: &WebviewWindow){
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { .. } = event {
            info!("Window close requested");
            // Add cleanup logic here in the future
        }
    });
}
