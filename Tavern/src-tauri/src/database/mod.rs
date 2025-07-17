use sqlx::{SqlitePool, Row};
use std::path::Path;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json;

use crate::errors::{AppError, AppResult};
use crate::database::models::*;

pub mod models;
pub mod migrations;

#[derive(Debug, Clone)]
pub struct DatabaseManager {
    pool: SqlitePool,
}

impl DatabaseManager {
    /// Create a new database manager instance
    pub async fn new(database_path: &str) -> AppResult<Self> {
        // Ensure the directory exists
        if let Some(parent) = Path::new(database_path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let database_url = format!("sqlite:{}", database_path);
        let pool = SqlitePool::connect(&database_url).await?;

        Ok(Self { pool })
    }

    /// Run database migrations
    pub async fn run_migrations(&self) -> AppResult<()> {
        sqlx::migrate!("./src/database").run(&self.pool).await?;
        Ok(())
    }

    /// Close the database connection
    pub async fn close(&self) -> AppResult<()> {
        self.pool.close().await;
        Ok(())
    }

    // =============================================================================
    // Campaign Operations
    // =============================================================================

    /// Create a new campaign
    pub async fn create_campaign(&self, data: CreateCampaignData) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let settings_json = serde_json::to_string(&data.settings)?;

        sqlx::query!(
            r#"
            INSERT INTO campaigns (id, name, description, dm_name, settings, created_at, updated_at, is_active)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            id,
            data.name,
            data.description,
            data.dm_name,
            settings_json,
            now,
            now,
            true
        )
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    /// Get all campaigns
    pub async fn get_all_campaigns(&self) -> AppResult<Vec<Campaign>> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, description, dm_name, settings, created_at, updated_at, is_active
            FROM campaigns
            ORDER BY updated_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut campaigns = Vec::new();
        for row in rows {
            let id: String = row.try_get("id").unwrap_or_default();
            let name: String = row.try_get("name").unwrap_or_default();
            let description: String = row.try_get("description").unwrap_or_default();
            let dm_name: String = row.try_get("dm_name").unwrap_or_default();
            let settings: CampaignSettings = match row.try_get::<Option<&str>, _>("settings")? {
                Some(s) => serde_json::from_str(s)?,
                None => CampaignSettings::default(),
            };
            let created_at: DateTime<Utc> = row.try_get("created_at")?;
            let updated_at: DateTime<Utc> = row.try_get("updated_at")?;
            let is_active: bool = row.try_get("is_active").unwrap_or(true);
            campaigns.push(Campaign {
                id,
                name,
                description: Some(description),
                dm_name,
                settings,
                created_at: created_at,
                updated_at: updated_at,
                is_active,
            });
        }
        Ok(campaigns)
    }

    /// Get a specific campaign by ID
    pub async fn get_campaign(&self, campaign_id: &str) -> AppResult<Option<Campaign>> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, dm_name, settings, created_at, updated_at, is_active
            FROM campaigns
            WHERE id = ?1
            "#
        )
        .bind(campaign_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let id: String = row.try_get("id").unwrap_or_default();
            let name: String = row.try_get("name").unwrap_or_default();
            let description: Option<String> = row.try_get("description").unwrap_or_default();
            let dm_name: String = row.try_get("dm_name").unwrap_or_default();
            let settings: CampaignSettings = match row.try_get::<Option<&str>, _>("settings")? {
                Some(s) => serde_json::from_str(s)?,
                None => CampaignSettings::default(),
            };
            let created_at: DateTime<Utc> = row.try_get("created_at")?;
            let updated_at: DateTime<Utc> = row.try_get("updated_at")?;
            let is_active: bool = row.try_get("is_active").unwrap_or(true);
            Ok(Some(Campaign {
                id,
                name,
                description,
                dm_name,
                settings,
                created_at: created_at,
                updated_at: updated_at,
                is_active,
            }))
        } else {
            Ok(None)
        }
    }

    /// Update a campaign
    pub async fn update_campaign(&self, campaign_id: String, data: UpdateCampaignRequest) -> AppResult<()> {
        let now = Utc::now();

        // Build dynamic update query based on provided fields
        let mut query_parts = Vec::new();
        let mut bind_count = 0;

        if data.name.is_some() {
            query_parts.push("name = ?");
            bind_count += 1;
        }
        if data.description.is_some() {
            query_parts.push("description = ?");
            bind_count += 1;
        }
        if data.settings.is_some() {
            query_parts.push("settings = ?");
            bind_count += 1;
        }
        query_parts.push("updated_at = ?");
        bind_count += 1;

        let query = format!("UPDATE campaigns SET {} WHERE id = ?", query_parts.join(", "));
        let mut query_builder = sqlx::query(&query);

        if let Some(name) = &data.name {
            query_builder = query_builder.bind(name);
        }
        if let Some(description) = &data.description {
            query_builder = query_builder.bind(description);
        }
        if let Some(settings) = &data.settings {
            let settings_json = serde_json::to_string(settings)?;
            query_builder = query_builder.bind(settings_json);
        }
        query_builder = query_builder.bind(now);
        query_builder = query_builder.bind(campaign_id);

        query_builder.execute(&self.pool).await?;
        Ok(())
    }

    /// Delete a campaign
    pub async fn delete_campaign(&self, campaign_id: &str) -> AppResult<()> {
        // Delete related data first (cascade delete)
        sqlx::query!("DELETE FROM characters WHERE campaign_id = ?", campaign_id)
            .execute(&self.pool)
            .await?;
        
        sqlx::query!("DELETE FROM maps WHERE campaign_id = ?", campaign_id)
            .execute(&self.pool)
            .await?;
        
        sqlx::query!("DELETE FROM assets WHERE campaign_id = ?", campaign_id)
            .execute(&self.pool)
            .await?;

        sqlx::query!("DELETE FROM campaigns WHERE id = ?", campaign_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // =============================================================================
    // Character Operations
    // =============================================================================

    /// Create a new character
    pub async fn create_character(&self, data: CreateCharacterRequest) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let stats_json = serde_json::to_string(&data.stats)?;
        let combat_stats = CombatStats::default();
        let combat_stats_json = serde_json::to_string(&combat_stats)?;
        let skills = Skills::default();
        let skills_json = serde_json::to_string(&skills)?;
        let equipment = Equipment::default();
        let equipment_json = serde_json::to_string(&equipment)?;
        let spells_json = serde_json::to_string(&Vec::<Spell>::new())?;
        let features_json = serde_json::to_string(&Vec::<Feature>::new())?;

        sqlx::query!(
            r#"
            INSERT INTO characters (
                id, campaign_id, name, player_name, character_class, level, race, background,
                stats, combat_stats, skills, equipment, spells, features, notes, is_npc,
                created_at, updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)
            "#,
            id,
            data.campaign_id,
            data.name,
            data.player_name,
            data.character_class,
            data.level,
            data.race,
            data.background,
            stats_json,
            combat_stats_json,
            skills_json,
            equipment_json,
            spells_json,
            features_json,
            "",
            data.is_npc,
            now,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    /// Get all characters for a campaign
    pub async fn get_characters(&self, campaign_id: &str) -> AppResult<Vec<Character>> {
        let rows = sqlx::query(
            r#"
            SELECT id, campaign_id, name, player_name, character_class, level, race, background,
                   stats, combat_stats, skills, equipment, spells, features, notes, avatar_url,
                   is_npc, created_at, updated_at
            FROM characters
            WHERE campaign_id = ?1
            ORDER BY name ASC
            "#
        )
        .bind(campaign_id)
        .fetch_all(&self.pool)
        .await?;

        let mut characters = Vec::new();
        for row in rows {
            let id: String = row.try_get("id").unwrap_or_default();
            let campaign_id: String = row.try_get("campaign_id").unwrap_or_default();
            let name: String = row.try_get("name").unwrap_or_default();
            let player_name: String = row.try_get("player_name").unwrap_or_default();
            let character_class: String = row.try_get("character_class").unwrap_or_default();
            let level = row.try_get("level").unwrap_or(1);
            let race: String = row.try_get("race").unwrap_or_default();
            let background: String = row.try_get("background").unwrap_or_default();
            let stats: CharacterStats = match row.try_get::<Option<&str>, _>("stats")? {
                Some(s) => serde_json::from_str(s)?,
                None => CharacterStats::default(),
            };
            let combat_stats: CombatStats = match row.try_get::<Option<&str>, _>("combat_stats")? {
                Some(s) => serde_json::from_str(s)?,
                None => CombatStats::default(),
            };
            let skills: Skills = match row.try_get::<Option<&str>, _>("skills")? {
                Some(s) => serde_json::from_str(s)?,
                None => Skills::default(),
            };
            let equipment: Equipment = match row.try_get::<Option<&str>, _>("equipment")? {
                Some(s) => serde_json::from_str(s)?,
                None => Equipment::default(),
            };
            let spells: Vec<Spell> = match row.try_get::<Option<&str>, _>("spells")? {
                Some(s) => serde_json::from_str(s)?,
                None => Vec::new(),
            };
            let features: Vec<Feature> = match row.try_get::<Option<&str>, _>("features")? {
                Some(s) => serde_json::from_str(s)?,
                None => Vec::new(),
            };
            let notes: String = row.try_get("notes").unwrap_or_default();
            let avatar_url: Option<String> = row.try_get("avatar_url").ok();
            let is_npc: bool = row.try_get("is_npc").unwrap_or(false);
            let created_at: DateTime<Utc> = row.try_get("created_at")?;
            let updated_at: DateTime<Utc> = row.try_get("updated_at")?;
            characters.push(Character {
                id,
                campaign_id,
                name,
                player_name: Some(player_name),
                character_class,
                level,
                race,
                background,
                stats,
                combat_stats,
                skills,
                equipment,
                spells,
                features,
                notes,
                avatar_url,
                is_npc,
                created_at: created_at,
                updated_at: updated_at,
            });
        }
        Ok(characters)
    }

    pub async fn get_characters_for_campaign(&self, _campaign_id: &String) -> AppResult<Vec<Character>> {
        todo!()
    }
    /// Get a specific character
    pub async fn get_character(&self, character_id: &str) -> AppResult<Option<Character>> {
        let row = sqlx::query(
            r#"
            SELECT id, campaign_id, name, player_name, character_class, level, race, background,
                   stats, combat_stats, skills, equipment, spells, features, notes, avatar_url,
                   is_npc, created_at, updated_at
            FROM characters
            WHERE id = ?1
            "#
        )
        .bind(character_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let id: String = row.try_get("id").unwrap_or_default();
            let campaign_id: String = row.try_get("campaign_id").unwrap_or_default();
            let name: String = row.try_get("name").unwrap_or_default();
            let player_name: String = row.try_get("player_name").unwrap_or_default();
            let character_class: String = row.try_get("character_class").unwrap_or_default();
            let level = row.try_get("level").unwrap_or(1);
            let race: String = row.try_get("race").unwrap_or_default();
            let background: String = row.try_get("background").unwrap_or_default();
            let stats: CharacterStats = match row.try_get::<Option<&str>, _>("stats")? {
                Some(s) => serde_json::from_str(s)?,
                None => CharacterStats::default(),
            };
            let combat_stats: CombatStats = match row.try_get::<Option<&str>, _>("combat_stats")? {
                Some(s) => serde_json::from_str(s)?,
                None => CombatStats::default(),
            };
            let skills: Skills = match row.try_get::<Option<&str>, _>("skills")? {
                Some(s) => serde_json::from_str(s)?,
                None => Skills::default(),
            };
            let equipment: Equipment = match row.try_get::<Option<&str>, _>("equipment")? {
                Some(s) => serde_json::from_str(s)?,
                None => Equipment::default(),
            };
            let spells: Vec<Spell> = match row.try_get::<Option<&str>, _>("spells")? {
                Some(s) => serde_json::from_str(s)?,
                None => Vec::new(),
            };
            let features: Vec<Feature> = match row.try_get::<Option<&str>, _>("features")? {
                Some(s) => serde_json::from_str(s)?,
                None => Vec::new(),
            };
            let notes: String = row.try_get("notes").unwrap_or_default();
            let avatar_url: Option<String> = row.try_get("avatar_url").ok();
            let is_npc: bool = row.try_get("is_npc").unwrap_or(false);
            let created_at: DateTime<Utc> = row.try_get("created_at")?;
            let updated_at: DateTime<Utc> = row.try_get("updated_at")?;
            Ok(Some(Character {
                id,
                campaign_id,
                name,
                player_name: Some(player_name),
                character_class,
                level,
                race,
                background,
                stats,
                combat_stats,
                skills,
                equipment,
                spells,
                features,
                notes,
                avatar_url,
                is_npc,
                created_at: created_at,
                updated_at: updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    /// Update a character
    pub async fn update_character(&self, character_id: &str, data: UpdateCharacterRequest) -> AppResult<()> {
        let now = Utc::now();

        // Build dynamic update query
        let mut query_parts = Vec::new();
        let mut bind_values = Vec::new();

        if let Some(name) = &data.name {
            query_parts.push("name = ?".to_string());
            bind_values.push(name.clone());
        }

        if let Some(level) = data.level {
            query_parts.push("level = ?".to_string());
            bind_values.push(level.to_string());
        }

        if let Some(stats) = &data.stats {
            query_parts.push("stats = ?".to_string());
            bind_values.push(serde_json::to_string(stats)?);
        }

        if let Some(combat_stats) = &data.combat_stats {
            query_parts.push("combat_stats = ?".to_string());
            bind_values.push(serde_json::to_string(combat_stats)?);
        }

        if let Some(equipment) = &data.equipment {
            query_parts.push("equipment = ?".to_string());
            bind_values.push(serde_json::to_string(equipment)?);
        }

        if let Some(notes) = &data.notes {
            query_parts.push("notes = ?".to_string());
            bind_values.push(notes.clone());
        }

        if query_parts.is_empty() {
            return Ok(());
        }

        query_parts.push("updated_at = ?".to_string());
        bind_values.push(now.to_rfc3339());
        bind_values.push(character_id.to_string());

        let query = format!(
            "UPDATE characters SET {} WHERE id = ?",
            query_parts.join(", ")
        );

        let mut query_builder = sqlx::query(&query);
        for value in bind_values {
            query_builder = query_builder.bind(value);
        }

        query_builder.execute(&self.pool).await?;
        Ok(())
    }

    /// Delete a character
    pub async fn delete_character(&self, character_id: &str) -> AppResult<()> {
        sqlx::query!("DELETE FROM characters WHERE id = ?", character_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // =============================================================================
    // Map Operations
    // =============================================================================

    /// Create a new map
    pub async fn create_map(&self, data: CreateMapRequest) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let tokens_json = serde_json::to_string(&Vec::<Token>::new())?;

        sqlx::query!(
            r#"
            INSERT INTO maps (id, campaign_id, name, description, image_url, grid_size, width, height, tokens, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
            id,
            data.campaign_id,
            data.name,
            data.description,
            data.image_url,
            data.grid_size,
            data.width,
            data.height,
            tokens_json,
            now,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    /// Get all maps for a campaign
    pub async fn get_maps(&self, campaign_id: &str) -> AppResult<Vec<Map>> {
        let rows = sqlx::query(
            r#"
            SELECT id, campaign_id, name, description, image_url, grid_size, width, height, tokens, fog_of_war, created_at, updated_at
            FROM maps
            WHERE campaign_id = ?1
            ORDER BY name ASC
            "#
        )
        .bind(campaign_id)
        .fetch_all(&self.pool)
        .await?;

        let mut maps = Vec::new();
        for row in rows {
            let id: String = row.try_get("id").unwrap_or_default();
            let campaign_id: String = row.try_get("campaign_id").unwrap_or_default();
            let name: String = row.try_get("name").unwrap_or_default();
            let description: String = row.try_get("description").unwrap_or_default();
            let image_url: String = row.try_get("image_url").unwrap_or_default();
            let grid_size = row.try_get("grid_size").unwrap_or(32);
            let width = row.try_get("width").unwrap_or(0);
            let height = row.try_get("height").unwrap_or(0);
            let tokens: Vec<Token> = match row.try_get::<Option<&str>, _>("tokens")? {
                Some(s) => serde_json::from_str(s)?,
                None => Vec::new(),
            };
            let fog_of_war: Option<FogOfWar> = match row.try_get::<Option<&str>, _>("fog_of_war")? {
                Some(s) => serde_json::from_str(s).ok(),
                None => None,
            };
            let created_at: DateTime<Utc> = row.try_get("created_at")?;
            let updated_at: DateTime<Utc> = row.try_get("updated_at")?;
            maps.push(Map {
                id,
                campaign_id,
                name,
                description: Some(description),
                image_url,
                grid_size,
                width,
                height,
                tokens,
                fog_of_war,
                created_at: created_at,
                updated_at: updated_at,
            });
        }
        Ok(maps)
    }

    /// Get a specific map
    pub async fn get_map(&self, map_id: &str) -> AppResult<Option<Map>> {
        let row = sqlx::query(
            r#"
            SELECT id, campaign_id, name, description, image_url, grid_size, width, height, tokens, fog_of_war, created_at, updated_at
            FROM maps
            WHERE id = ?1
            "#
        )
        .bind(map_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let id: String = row.try_get("id").unwrap_or_default();
            let campaign_id: String = row.try_get("campaign_id").unwrap_or_default();
            let name: String = row.try_get("name").unwrap_or_default();
            let description: String = row.try_get("description").unwrap_or_default();
            let image_url: String = row.try_get("image_url").unwrap_or_default();
            let grid_size = row.try_get("grid_size").unwrap_or(32);
            let width = row.try_get("width").unwrap_or(0);
            let height = row.try_get("height").unwrap_or(0);
            let tokens: Vec<Token> = match row.try_get::<Option<&str>, _>("tokens")? {
                Some(s) => serde_json::from_str(s)?,
                None => Vec::new(),
            };
            let fog_of_war: Option<FogOfWar> = match row.try_get::<Option<&str>, _>("fog_of_war")? {
                Some(s) => serde_json::from_str(s).ok(),
                None => None,
            };
            let created_at: DateTime<Utc> = row.try_get("created_at")?;
            let updated_at: DateTime<Utc> = row.try_get("updated_at")?;
            Ok(Some(Map {
                id,
                campaign_id,
                name,
                description: Some(description),
                image_url,
                grid_size,
                width,
                height,
                tokens,
                fog_of_war,
                created_at: created_at,
                updated_at: updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    /// Save map state (tokens, fog of war, etc.)
    pub async fn save_map_state(&self, map_id: &str, tokens: Vec<Token>, fog_of_war: Option<FogOfWar>) -> AppResult<()> {
        let now = Utc::now();
        let tokens_json = serde_json::to_string(&tokens)?;
        let fog_json = fog_of_war.as_ref().map(|f| serde_json::to_string(f)).transpose()?;

        sqlx::query!(
            "UPDATE maps SET tokens = ?1, fog_of_war = ?2, updated_at = ?3 WHERE id = ?4",
            tokens_json,
            fog_json,
            now,
            map_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // =============================================================================
    // Asset Operations
    // =============================================================================

    /// Import an asset
    pub async fn import_asset(&self, campaign_id: Option<String>, name: String, file_path: String, asset_type: AssetType, file_size: i64, mime_type: String, tags: Vec<String>) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let asset_type_str = serde_json::to_string(&asset_type)?;
        let tags_json = serde_json::to_string(&tags)?;

        sqlx::query!(
            r#"
            INSERT INTO assets (id, campaign_id, name, file_path, asset_type, file_size, mime_type, tags, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
            id,
            campaign_id,
            name,
            file_path,
            asset_type_str,
            file_size,
            mime_type,
            tags_json,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    /// Get assets for a campaign (or global assets if campaign_id is None)
    pub async fn get_assets(&self, campaign_id: Option<&str>) -> AppResult<Vec<Asset>> {
        let rows = if let Some(cid) = campaign_id {
            sqlx::query(
                r#"
                SELECT id, campaign_id, name, file_path, asset_type, file_size, mime_type, tags, created_at
                FROM assets
                WHERE campaign_id = ?1 OR campaign_id IS NULL
                ORDER BY name ASC
                "#
            )
            .bind(cid)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"
                SELECT id, campaign_id, name, file_path, asset_type, file_size, mime_type, tags, created_at
                FROM assets
                WHERE campaign_id IS NULL
                ORDER BY name ASC
                "#
            )
            .fetch_all(&self.pool)
            .await?
        };

        let mut assets = Vec::new();
        for row in rows {
            let id: String = row.try_get("id").unwrap_or_default();
            let campaign_id: Option<String> = row.try_get("campaign_id").ok();
            let name: String = row.try_get("name").unwrap_or_default();
            let file_path: String = row.try_get("file_path").unwrap_or_default();
            let asset_type: AssetType = match row.try_get::<Option<&str>, _>("asset_type")? {
                Some(s) => serde_json::from_str(s)?,
                None => AssetType::default(),
            };
            let file_size: i64 = row.try_get("file_size").unwrap_or(0);
            let mime_type: String = row.try_get("mime_type").unwrap_or_default();
            let tags: Vec<String> = match row.try_get::<Option<&str>, _>("tags")? {
                Some(s) => serde_json::from_str(s)?,
                None => Vec::new(),
            };
            let created_at: DateTime<Utc> = row.try_get("created_at")?;
            assets.push(Asset {
                id,
                campaign_id,
                name,
                file_path,
                asset_type,
                file_size,
                mime_type,
                tags,
                created_at: created_at,
            });
        }
        Ok(assets)
    }

    /// Delete an asset
    pub async fn delete_asset(&self, asset_id: &str) -> AppResult<()> {
        sqlx::query!("DELETE FROM assets WHERE id = ?", asset_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // =============================================================================
    // Token Operations
    // =============================================================================

    /// Add a token to a map
    pub async fn add_token_to_map(&self, map_id: &str, token: Token) -> AppResult<()> {
        // Get current map state
        if let Some(mut map) = self.get_map(map_id).await? {
            map.tokens.push(token);
            self.save_map_state(map_id, map.tokens, map.fog_of_war).await?;
        }
        Ok(())
    }

    /// Update token position
    pub async fn update_token_position(&self, map_id: &str, token_id: &str, position: Position) -> AppResult<()> {
        if let Some(mut map) = self.get_map(map_id).await? {
            if let Some(token) = map.tokens.iter_mut().find(|t| t.id == token_id) {
                token.position = position;
                self.save_map_state(map_id, map.tokens, map.fog_of_war).await?;
            }
        }
        Ok(())
    }

    /// Remove token from map
    pub async fn remove_token_from_map(&self, map_id: &str, token_id: &str) -> AppResult<()> {
        if let Some(mut map) = self.get_map(map_id).await? {
            map.tokens.retain(|t| t.id != token_id);
            self.save_map_state(map_id, map.tokens, map.fog_of_war).await?;
        }
        Ok(())
    }
}