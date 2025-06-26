use crate::databsase::{DatabaseManager, models::*};
use crate::network::NetworkManager;
use crate::state::AppState;
use crate::dice::DiceRoller;
use crate::assets::AssetManager;
use crate::errors::{AppError, AppResult};
use crate::config::AppConfig;

use tauri::State;
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

#[tauri::command]
pub async fn create_campaign(
    request: CreateCampaignRequest,
    database: State<'_, DatabaseType>,
    state: State<'_, AppStateType>,
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
) -> AppResult<()> {
    let db = database.lock().await;
    db.update_campaign(campaign_id, request).await?;
    Ok(())
}

