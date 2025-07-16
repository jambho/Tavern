use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};



// =============================================================================
// Core Campaign Models
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Campaign {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub dm_name: String,
    pub settings: CampaignSettings,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CampaignSettings {
    pub system: GameSystem,
    pub house_rules: Vec<HouseRule>,
    pub variant_rules: VariantRules,
    pub dice_rolling: DiceSettings,
    pub combat_settings: CombatSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameSystem {
    #[serde(rename = "dnd5e")]
    DnD5e,
    #[serde(rename = "pathfinder2e")]
    Pathfinder2e,
    #[serde(rename = "generic")]
    Generic,
}

impl Default for GameSystem {
    fn default() -> Self {
        Self::DnD5e
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VariantRules {
    pub flanking: bool,
    pub feats: bool,
    pub multiclassing: bool,
    pub optional_class_features: bool,
    pub customizing_ability_scores: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiceSettings {
    pub advantage_mode: AdvantageMode,
    pub critical_hit_rules: CriticalHitRules,
    pub fumble_rules: bool,
    pub exploding_dice: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdvantageMode {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "automatic")]
    Automatic,
    #[serde(rename = "ask")]
    Ask,
}

impl Default for AdvantageMode {
    fn default() -> Self {
        Self::Manual
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CriticalHitRules {
    #[serde(rename = "double_dice")]
    DoubleDice,
    #[serde(rename = "max_plus_roll")]
    MaxPlusRoll,
    #[serde(rename = "standard")]
    Standard,
}

impl Default for CriticalHitRules {
    fn default() -> Self {
        Self::Standard
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CombatSettings {
    pub initiative_type: InitiativeType,
    pub death_saves: bool,
    pub healing_surge: bool,
    pub action_surge_limit: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InitiativeType {
    #[serde(rename = "individual")]
    Individual,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "side")]
    Side,
}

impl Default for InitiativeType {
    fn default() -> Self {
        Self::Individual
    }
}

// =============================================================================
// Character Models
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Character {
    pub id: String,
    pub campaign_id: String,
    pub name: String,
    pub player_name: Option<String>,
    pub character_class: String,
    pub level: i32,
    pub race: String,
    pub background: String,
    pub stats: CharacterStats,
    pub combat_stats: CombatStats,
    pub skills: Skills,
    pub equipment: Equipment,
    pub spells: Vec<Spell>,
    pub features: Vec<Feature>,
    pub notes: String,
    pub avatar_url: Option<String>,
    pub is_npc: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacterStats {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
    pub proficiency_bonus: i32,
}

impl CharacterStats {
    pub fn get_modifier(&self, stat: &str) -> i32 {
        let score = match stat.to_lowercase().as_str() {
            "strength" | "str" => self.strength,
            "dexterity" | "dex" => self.dexterity,
            "constitution" | "con" => self.constitution,
            "intelligence" | "int" => self.intelligence,
            "wisdom" | "wis" => self.wisdom,
            "charisma" | "cha" => self.charisma,
            _ => return 0,
        };
        (score - 10) / 2
    }

    pub fn get_saving_throw(&self, stat: &str, proficient: bool) -> i32 {
        let modifier = self.get_modifier(stat);
        if proficient {
            modifier + self.proficiency_bonus
        } else {
            modifier
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CombatStats {
    pub armor_class: i32,
    pub hit_points: i32,
    pub max_hit_points: i32,
    pub temporary_hit_points: i32,
    pub speed: i32,
    pub initiative_bonus: i32,
    pub death_saves_success: i32,
    pub death_saves_failure: i32,
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub description: String,
    pub duration: Option<i32>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Skills {
    pub acrobatics: SkillProficiency,
    pub animal_handling: SkillProficiency,
    pub arcana: SkillProficiency,
    pub athletics: SkillProficiency,
    pub deception: SkillProficiency,
    pub history: SkillProficiency,
    pub insight: SkillProficiency,
    pub intimidation: SkillProficiency,
    pub investigation: SkillProficiency,
    pub medicine: SkillProficiency,
    pub nature: SkillProficiency,
    pub perception: SkillProficiency,
    pub performance: SkillProficiency,
    pub persuasion: SkillProficiency,
    pub religion: SkillProficiency,
    pub sleight_of_hand: SkillProficiency,
    pub stealth: SkillProficiency,
    pub survival: SkillProficiency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillProficiency {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "proficient")]
    Proficient,
    #[serde(rename = "expertise")]
    Expertise,
}

impl Default for SkillProficiency {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Equipment {
    pub items: Vec<Item>,
    pub weapons: Vec<Weapon>,
    pub armor: Vec<Armor>,
    pub currency: Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub quantity: i32,
    pub weight: f32,
    pub value: i32, // in copper pieces
    pub rarity: ItemRarity,
    pub item_type: ItemType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemRarity {
    #[serde(rename = "common")]
    Common,
    #[serde(rename = "uncommon")]
    Uncommon,
    #[serde(rename = "rare")]
    Rare,
    #[serde(rename = "very_rare")]
    VeryRare,
    #[serde(rename = "legendary")]
    Legendary,
    #[serde(rename = "artifact")]
    Artifact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "adventuring_gear")]
    AdventuringGear,
    #[serde(rename = "tool")]
    Tool,
    #[serde(rename = "consumable")]
    Consumable,
    #[serde(rename = "treasure")]
    Treasure,
    #[serde(rename = "magic_item")]
    MagicItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub id: String,
    pub name: String,
    pub damage_dice: String,
    pub damage_type: DamageType,
    pub properties: Vec<WeaponProperty>,
    pub range: Option<WeaponRange>,
    pub is_equipped: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DamageType {
    #[serde(rename = "acid")]
    Acid,
    #[serde(rename = "bludgeoning")]
    Bludgeoning,
    #[serde(rename = "cold")]
    Cold,
    #[serde(rename = "fire")]
    Fire,
    #[serde(rename = "force")]
    Force,
    #[serde(rename = "lightning")]
    Lightning,
    #[serde(rename = "necrotic")]
    Necrotic,
    #[serde(rename = "piercing")]
    Piercing,
    #[serde(rename = "poison")]
    Poison,
    #[serde(rename = "psychic")]
    Psychic,
    #[serde(rename = "radiant")]
    Radiant,
    #[serde(rename = "slashing")]
    Slashing,
    #[serde(rename = "thunder")]
    Thunder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponProperty {
    #[serde(rename = "ammunition")]
    Ammunition,
    #[serde(rename = "finesse")]
    Finesse,
    #[serde(rename = "heavy")]
    Heavy,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "loading")]
    Loading,
    #[serde(rename = "reach")]
    Reach,
    #[serde(rename = "special")]
    Special,
    #[serde(rename = "thrown")]
    Thrown,
    #[serde(rename = "two_handed")]
    TwoHanded,
    #[serde(rename = "versatile")]
    Versatile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponRange {
    pub normal: i32,
    pub long: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Armor {
    pub id: String,
    pub name: String,
    pub armor_class: i32,
    pub armor_type: ArmorType,
    pub stealth_disadvantage: bool,
    pub is_equipped: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArmorType {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "heavy")]
    Heavy,
    #[serde(rename = "shield")]
    Shield,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Currency {
    pub copper: i32,
    pub silver: i32,
    pub electrum: i32,
    pub gold: i32,
    pub platinum: i32,
}

impl Currency {
    pub fn total_in_copper(&self) -> i32 {
        self.copper + (self.silver * 10) + (self.electrum * 50) + (self.gold * 100) + (self.platinum * 1000)
    }

    pub fn total_in_gold(&self) -> f32 {
        self.total_in_copper() as f32 / 100.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    pub id: String,
    pub name: String,
    pub level: i32,
    pub school: SpellSchool,
    pub casting_time: String,
    pub range: String,
    pub components: SpellComponents,
    pub duration: String,
    pub description: String,
    pub is_prepared: bool,
    pub is_ritual: bool,
    pub damage: Option<SpellDamage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpellSchool {
    #[serde(rename = "abjuration")]
    Abjuration,
    #[serde(rename = "conjuration")]
    Conjuration,
    #[serde(rename = "divination")]
    Divination,
    #[serde(rename = "enchantment")]
    Enchantment,
    #[serde(rename = "evocation")]
    Evocation,
    #[serde(rename = "illusion")]
    Illusion,
    #[serde(rename = "necromancy")]
    Necromancy,
    #[serde(rename = "transmutation")]
    Transmutation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellComponents {
    pub verbal: bool,
    pub somatic: bool,
    pub material: bool,
    pub material_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellDamage {
    pub damage_dice: String,
    pub damage_type: DamageType,
    pub scaling: Option<SpellScaling>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellScaling {
    pub per_level: String,
    pub max_level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source: FeatureSource,
    pub uses: Option<FeatureUses>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureSource {
    #[serde(rename = "class")]
    Class,
    #[serde(rename = "race")]
    Race,
    #[serde(rename = "background")]
    Background,
    #[serde(rename = "feat")]
    Feat,
    #[serde(rename = "magic_item")]
    MagicItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureUses {
    pub max_uses: i32,
    pub current_uses: i32,
    pub recharge: RechargeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RechargeType {
    #[serde(rename = "short_rest")]
    ShortRest,
    #[serde(rename = "long_rest")]
    LongRest,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
}

// =============================================================================
// Map and Token Models
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Map {
    pub id: String,
    pub campaign_id: String,
    pub name: String,
    pub description: Option<String>,
    pub image_url: String,
    pub grid_size: i32,
    pub width: i32,
    pub height: i32,
    pub tokens: Vec<Token>,
    pub fog_of_war: Option<FogOfWar>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub character_id: Option<String>,
    pub name: String,
    pub image_url: Option<String>,
    pub position: Position,
    pub size: TokenSize,
    pub conditions: Vec<String>,
    pub notes: String,
    pub is_hidden: bool,
    pub initiative: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: Option<f32>, // For elevation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenSize {
    #[serde(rename = "tiny")]
    Tiny,      // 2.5x2.5 ft
    #[serde(rename = "small")]
    Small,     // 5x5 ft
    #[serde(rename = "medium")]
    Medium,    // 5x5 ft
    #[serde(rename = "large")]
    Large,     // 10x10 ft
    #[serde(rename = "huge")]
    Huge,      // 15x15 ft
    #[serde(rename = "gargantuan")]
    Gargantuan, // 20x20 ft or larger
}

impl TokenSize {
    pub fn grid_squares(&self) -> i32 {
        match self {
            TokenSize::Tiny => 1,
            TokenSize::Small => 1,
            TokenSize::Medium => 1,
            TokenSize::Large => 4,
            TokenSize::Huge => 9,
            TokenSize::Gargantuan => 16,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FogOfWar {
    pub revealed_areas: Vec<RevealedArea>,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevealedArea {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub shape: AreaShape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AreaShape {
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "polygon")]
    Polygon(Vec<Position>),
}

// =============================================================================
// Request/Response DTOs
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateCampaignData {
    pub name: String,
    pub description: Option<String>,
    pub dm_name: String,
    pub settings: CampaignSettings,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCampaignRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub settings: Option<CampaignSettings>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCharacterRequest {
    pub campaign_id: String,
    pub name: String,
    pub player_name: Option<String>,
    pub character_class: String,
    pub level: i32,
    pub race: String,
    pub background: String,
    pub stats: CharacterStats,
    pub is_npc: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCharacterRequest {
    pub name: Option<String>,
    pub level: Option<i32>,
    pub stats: Option<CharacterStats>,
    pub combat_stats: Option<CombatStats>,
    pub equipment: Option<Equipment>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMapRequest {
    pub campaign_id: String,
    pub name: String,
    pub description: Option<String>,
    pub image_url: String,
    pub grid_size: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateTokenRequest {
    pub map_id: String,
    pub character_id: Option<String>,
    pub name: String,
    pub image_url: Option<String>,
    pub position: Position,
    pub size: TokenSize,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTokenPositionRequest {
    pub token_id: String,
    pub position: Position,
}

// =============================================================================
// Dice Rolling Models
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiceRoll {
    pub dice_notation: String,
    pub individual_rolls: Vec<i32>,
    pub modifiers: Vec<DiceModifier>,
    pub total: i32,
    pub roll_type: RollType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiceModifier {
    pub name: String,
    pub value: i32,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RollType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "advantage")]
    Advantage,
    #[serde(rename = "disadvantage")]
    Disadvantage,
    #[serde(rename = "critical")]
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiativeRoll {
    pub character_id: String,
    pub character_name: String,
    pub roll: DiceRoll,
    pub initiative_bonus: i32,
    pub total: i32,
}

// =============================================================================
// Network Models
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub id: String,
    pub sender_id: String,
    pub sender_name: String,
    pub message_type: MessageType,
    pub content: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    #[serde(rename = "chat")]
    Chat,
    #[serde(rename = "dice_roll")]
    DiceRoll,
    #[serde(rename = "token_update")]
    TokenUpdate,
    #[serde(rename = "map_change")]
    MapChange,
    #[serde(rename = "initiative")]
    Initiative,
    #[serde(rename = "system")]
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message: String,
    pub is_whisper: bool,
    pub target_players: Option<Vec<String>>,
    pub is_in_character: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: String,
    pub name: String,
    pub role: PlayerRole,
    pub is_connected: bool,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerRole {
    #[serde(rename = "dm")]
    DungeonMaster,
    #[serde(rename = "player")]
    Player,
    #[serde(rename = "observer")]
    Observer,
}

// =============================================================================
// Asset Models
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Asset {
    pub id: String,
    pub campaign_id: Option<String>,
    pub name: String,
    pub file_path: String,
    pub asset_type: AssetType,
    pub file_size: i64,
    pub mime_type: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "portrait")]
    Portrait,
    #[serde(rename = "handout")]
    Handout,
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "other")]
    Other,
}