//use crate::network::NetworkManager;
//use crate::errors::{AppError, AppResult};
//use crate::config::AppConfig;
use crate::database::models::{
    Campaign, Character, Map, Token, CampaignSettings, CreateCampaignData,
    CreateCharacterData, CreateMapRequest, CreateTokenRequest, MapData,
};      
                               

use tauri::{State, AppHandle, WebviewWindow, Manager, Emitter};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

// Type aliases for cleaner code
type AppStateType = Arc<Mutex<AppState>>;
type DatabaseType = Arc<Mutex<DatabaseManager>>;
type NetworkType = Arc<Mutex<NetworkManager>>;

// =============================================================================
// Campaign Commands
// =============================================================================

#[derive(Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub description: Option<String>,
    pub dm_name: String,
    pub settings: Option<CampaignSettings>,
}

#[derive(Deserialize)]
pub struct UpdateCampaignRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub dm_name: Option<String>,
    pub settings: Option<CampaignSettings>,
}

#[tauri::command]
pub async fn create_campaign(
    request: CreateCampaignRequest,
    database: State<'_, DatabaseType>,
    state: State<'_, AppStateType>,
    app_handle: AppHandle,
) -> AppResult<String> {
    let db = database.lock().await;
    let campaign_id = db.create_campaign(CreateCampaignData {
        name: request.name,
        description: request.description,
        dm_name: request.dm_name,
        settings: request.settings.unwrap_or_default(),
    }).await?;
    
    // Update application state
    let mut app_state = state.lock().await;
    app_state.set_active_campaign(campaign_id.clone());
    
    // Emit event to frontend about the new campaign
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("campaign-created", &campaign_id);
    }
    
    Ok(campaign_id)
}

#[tauri::command]
pub async fn get_campaigns(
    database: State<'_, DatabaseType>,
) -> AppResult<Vec<Campaign>> {
    let db = database.lock().await;
    let campaigns = db.get_all_campaigns().await?;
    Ok(campaigns)
}

#[tauri::command]
pub async fn get_campaign(
    campaign_id: String,
    database: State<'_, DatabaseType>,
) -> AppResult<Option<Campaign>> {
    let db = database.lock().await;
    let campaign = db.get_campaign(&campaign_id).await?;
    Ok(campaign)
}

#[tauri::command]
pub async fn update_campaign(
    campaign_id: String,
    request: UpdateCampaignRequest,
    database: State<'_, DatabaseType>,
    app_handle: AppHandle,
) -> AppResult<()> {
    let db = database.lock().await;
    db.update_campaign(campaign_id.clone(), request).await?;
    
    // Emit event to frontend about the campaign update
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("campaign-updated", &campaign_id);
    }
    
    Ok(())
}

#[tauri::command]
pub async fn delete_campaign(
    campaign_id: String,
    database: State<'_, DatabaseType>,
    state: State<'_, AppStateType>,
    app_handle: AppHandle,
) -> AppResult<()> {
    let db = database.lock().await;
    db.delete_campaign(&campaign_id).await?;
    
    // Update application state if this was the active campaign
    let mut app_state = state.lock().await;
    if app_state.get_active_campaign() == Some(&campaign_id) {
        app_state.clear_active_campaign();
    }
    
    // Emit event to frontend about the campaign deletion
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("campaign-deleted", &campaign_id);
    }
    
    Ok(())
}

// =============================================================================
// Character Commands
// =============================================================================

#[derive(Deserialize)]
pub struct CreateCharacterRequest {
    pub campaign_id: String,
    pub name: String,
    pub character_class: String,
    pub level: i32,
    pub race: String,
    pub stats: CharacterStats,
}

#[tauri::command]
pub async fn create_character(
    request: CreateCharacterRequest,
    database: State<'_, DatabaseType>,
    app_handle: AppHandle,
) -> AppResult<String> {
    let db = database.lock().await;
    let character_id = db.create_character(CreateCharacterData {
        campaign_id: request.campaign_id,
        name: request.name,
        character_class: request.character_class,
        level: request.level,
        race: request.race,
        stats: request.stats,
    }).await?;
    
    // Emit event to frontend
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("character-created", &character_id);
    }
    
    Ok(character_id)
}

#[tauri::command]
pub async fn get_characters(
    campaign_id: String,
    database: State<'_, DatabaseType>,
) -> AppResult<Vec<Character>> {
    let db = database.lock().await;
    let characters = db.get_characters_for_campaign(&campaign_id).await?;
    Ok(characters)
}

#[tauri::command]
pub async fn get_character(
    character_id: String,
    database: State<'_, DatabaseType>,
) -> AppResult<Option<Character>> {
    let db = database.lock().await;
    let character = db.get_character(&character_id).await?;
    Ok(character)
}

#[tauri::command]
pub async fn update_character(
    character_id: String,
    request: UpdateCharacterRequest,
    database: State<'_, DatabaseType>,
    app_handle: AppHandle,
) -> AppResult<()> {
    let db = database.lock().await;
    db.update_character(character_id.clone(), request).await?;
    
    // Emit event to frontend
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("character-updated", &character_id);
    }
    
    Ok(())
}

#[tauri::command]
pub async fn delete_character(
    character_id: String,
    database: State<'_, DatabaseType>,
    app_handle: AppHandle,
) -> AppResult<()> {
    let db = database.lock().await;
    db.delete_character(&character_id).await?;
    
    // Emit event to frontend
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("character-deleted", &character_id);
    }
    
    Ok(())
}

// =============================================================================
// Map Commands
// =============================================================================
/*
#[tauri::command]
pub async fn create_map(
    request: CreateMapRequest,
    database: State<'_, DatabaseType>,
    app_handle: AppHandle,
) -> AppResult<String> {
    let db = database.lock().await;
    let map_id = db.create_map(request).await?;
    
    // Emit event to frontend
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("map-created", &map_id);
    }
    
    Ok(map_id)
}

#[tauri::command]
pub async fn get_maps(
    campaign_id: String,
    database: State<'_, DatabaseType>,
) -> AppResult<Vec<Map>> {
    let db = database.lock().await;
    let maps = db.get_maps_for_campaign(&campaign_id).await?;
    Ok(maps)
}

#[tauri::command]
pub async fn load_map(
    map_id: String,
    database: State<'_, DatabaseType>,
    state: State<'_, AppStateType>,
    app_handle: AppHandle,
) -> AppResult<Map> {
    let db = database.lock().await;
    let map = db.get_map(&map_id).await?
        .ok_or_else(|| AppError::NotFound("Map not found".to_string()))?;
    
    // Update application state with active map
    let mut app_state = state.lock().await;
    app_state.set_active_map(map_id.clone());
    
    // Emit event to frontend
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("map-loaded", &map);
    }
    
    Ok(map)
}

#[tauri::command]
pub async fn save_map_state(
    map_id: String,
    map_data: MapData,
    database: State<'_, DatabaseType>,
    app_handle: AppHandle,
) -> AppResult<()> {
    let db = database.lock().await;
    db.save_map_state(&map_id, map_data).await?;
    
    // Emit event to frontend
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("map-saved", &map_id);
    }
    
    Ok(())
}

// =============================================================================
// Token Commands  
// =============================================================================

#[tauri::command]
pub async fn create_token(
    request: CreateTokenRequest,
    database: State<'_, DatabaseType>,
    network: State<'_, NetworkType>,
    app_handle: AppHandle,
) -> AppResult<String> {
    let db = database.lock().await;
    let token_id = db.create_token(request).await?;
    
    // Broadcast to network peers
    let network_manager = network.lock().await;
    if let Err(e) = network_manager.broadcast_token_created(&token_id).await {
        tracing::warn!("Failed to broadcast token creation: {}", e);
    }
    
    // Emit event to frontend
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("token-created", &token_id);
    }
    
    Ok(token_id)
}

#[tauri::command]
pub async fn update_token_position(
    token_id: String,
    x: f64,
    y: f64,
    database: State<'_, DatabaseType>,
    network: State<'_, NetworkType>,
    app_handle: AppHandle,
) -> AppResult<()> {
    let db = database.lock().await;
    db.update_token_position(&token_id, x, y).await?;
    
    // Broadcast to network peers
    let network_manager = network.lock().await;
    if let Err(e) = network_manager.broadcast_token_moved(&token_id, x, y).await {
        tracing::warn!("Failed to broadcast token movement: {}", e);
    }
    
    // Emit event to frontend
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("token-moved", serde_json::json!({
            "token_id": token_id,
            "x": x,
            "y": y
        }));
    }
    
    Ok(())
}

#[tauri::command]
pub async fn delete_token(
    token_id: String,
    database: State<'_, DatabaseType>,
    network: State<'_, NetworkType>,
    app_handle: AppHandle,
) -> AppResult<()> {
    let db = database.lock().await;
    db.delete_token(&token_id).await?;
    
    // Broadcast to network peers
    let network_manager = network.lock().await;
    if let Err(e) = network_manager.broadcast_token_deleted(&token_id).await {
        tracing::warn!("Failed to broadcast token deletion: {}", e);
    }
    
    // Emit event to frontend
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("token-deleted", &token_id);
    }
    
    Ok(())
}

// =============================================================================
// Dice Commands
// =============================================================================
/*
#[tauri::command]
pub async fn roll_dice(
    dice_expression: String,
    app_handle: AppHandle,
) -> AppResult<DiceResult> {
    let roller = DiceRoller::new();
    let result = roller.roll(&dice_expression)?;
    
    // Emit event to frontend for dice animation/effects
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("dice-rolled", &result);
    }
    
    Ok(result)
}

#[tauri::command]
pub async fn roll_initiative(
    character_ids: Vec<String>,
    database: State<'_, DatabaseType>,
    app_handle: AppHandle,
) -> AppResult<Vec<InitiativeResult>> {
    let db = database.lock().await;
    let roller = DiceRoller::new();
    let mut results = Vec::new();
    
    for character_id in character_ids {
        if let Some(character) = db.get_character(&character_id).await? {
            let initiative_modifier = character.stats.dexterity_modifier();
            let roll = roller.roll("1d20")?.total;
            let total = roll + initiative_modifier;
            
            results.push(InitiativeResult {
                character_id: character_id.clone(),
                character_name: character.name,
                roll,
                modifier: initiative_modifier,
                total,
            });
        }
    }
    
    // Sort by initiative (highest first)
    results.sort_by(|a, b| b.total.cmp(&a.total));
    
    // Emit event to frontend
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("initiative-rolled", &results);
    }
    
    Ok(results)
}
*/*/
// =============================================================================
// Utility Structs
// =============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCharacterData {
    pub campaign_id: String,
    pub name: String,
    pub character_class: String,
    pub level: i32,
    pub race: String,
    pub stats: CharacterStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterStats {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

impl CharacterStats {
    pub fn dexterity_modifier(&self) -> i32 {
        (self.dexterity - 10) / 2
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCharacterRequest {
    pub name: Option<String>,
    pub character_class: Option<String>,
    pub level: Option<i32>,
    pub race: Option<String>,
    pub stats: Option<CharacterStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMapRequest {
    pub campaign_id: String,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub background_image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapData {
    pub tokens: Vec<TokenData>,
    pub fog_of_war: Option<String>,
    pub grid_size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub image: String,
    pub size: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTokenRequest {
    pub map_id: String,
    pub character_id: Option<String>,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub image: String,
    pub size: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub asset_type: String,
    pub file_path: String,
    pub campaign_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct DiceResult {
    pub expression: String,
    pub rolls: Vec<i32>,
    pub total: i32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct InitiativeResult {
    pub character_id: String,
    pub character_name: String,
    pub roll: i32,
    pub modifier: i32,
    pub total: i32,
}
