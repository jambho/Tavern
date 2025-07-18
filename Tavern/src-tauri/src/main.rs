#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, WebviewWindow, Emitter, async_runtime};
use tracing::{info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;
use tokio::sync::Mutex;


mod state;
use crate::state::AppState;
mod database;
use crate::database::DatabaseManager;
use crate::database::models::{Campaign, Character, Map, Token};
mod errors;
mod commands;
mod networking;
use commands::*;

fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tavern=debug,tauri=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    async_runtime::block_on(async {

    
    let db_path = "data/tavern.db";
    let db = DatabaseManager::new(db_path)
        .await
        .expect("Failed to initialize database");
    db.run_migrations().await.expect("Migration failed");

    let database = Arc::new(Mutex::new(db.clone()));
    let app_state = Arc::new(Mutex::new(AppState::default()));

    //dev code
    // let createCampaignData = database::models::CreateCampaignData {
    //     name: "TestCampaign".to_string(),
    //     description: Some("TestDescription".to_string()),
    //     dm_name: "Jame".to_string(),
    //     settings: database::models::CampaignSettings {
    //         system: database::models::GameSystem::DnD5e,
    //         house_rules: vec!(database::models::HouseRule { 
    //             id: "1".to_string(), 
    //             name: "TestRule".to_string(), 
    //             description: "TestRuleDescription".to_string(), 
    //             is_enabled: true 
    //         }),
    //         variant_rules: database::models::VariantRules {
    //             flanking: true,
    //             feats: true,
    //             multiclassing: true,
    //             optional_class_features: true,
    //             customizing_ability_scores: true,
    //         },
    //         dice_rolling: database::models::DiceSettings {
    //             advantage_mode: database::models::AdvantageMode::Ask,
    //             critical_hit_rules: database::models::CriticalHitRules::Standard,
    //             fumble_rules: false,
    //             exploding_dice: false,
    //         },
    //         combat_settings: database::models::CombatSettings {
    //             initiative_type: database::models::InitiativeType::Individual,
    //             death_saves: true,
    //             healing_surge: true,
    //             action_surge_limit: 1,
    //         },
    //     },
    // }; 
    // let id = DatabaseManager::create_campaign(&db, createCampaignData).await.expect("Failed to Create Campaign");
    // let fighterRequest = database::models::CreateCharacterRequest {
    //     campaign_id: id,
    //     name: "TestFighter".to_string(),
    //     player_name: Some("Jame".to_string()),
    //     character_class: "Fighter".to_string(),
    //     level: 1,
    //     race: "Human".to_string(),
    //     background: "Hermit".to_string(),
    //     stats: database::models::CharacterStats {
    //         strength: 10,
    //         dexterity: 10,
    //         constitution: 10,
    //         intelligence: 10,
    //         wisdom: 10,
    //         charisma: 10,
    //         proficiency_bonus: 1,
    //     },
    //     is_npc: false,
    // };
    // let _ = DatabaseManager::create_character(&db, fighterRequest).await.expect("Failed to Create Character");
    // end dev code

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .manage(database)
        .invoke_handler(tauri::generate_handler![
            get_app_version,
            restart_app,
            handle_menu_action,
            show_main_window,
            create_campaign,
            get_campaigns,
            get_campaign,
            // update_campaign,
            delete_campaign,
            create_character,
            get_characters,
            get_character,
            update_character,
            delete_character,
            // create_map,
            // get_maps,
            // load_map,
            // save_map_state,
            // create_token,
            // update_token_position,
            // delete_token,
            // roll_dice,
            // roll_initiative
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
    });
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
