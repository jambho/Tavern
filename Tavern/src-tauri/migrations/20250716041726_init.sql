-- Campaigns
CREATE TABLE campaigns (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    dm_name TEXT NOT NULL,
    settings JSON NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    is_active BOOLEAN NOT NULL
);

-- Characters
CREATE TABLE characters (
    id TEXT PRIMARY KEY,
    campaign_id TEXT NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    player_name TEXT,
    character_class TEXT NOT NULL,
    level INTEGER NOT NULL,
    race TEXT NOT NULL,
    background TEXT NOT NULL,
    stats JSON NOT NULL,
    combat_stats JSON NOT NULL,
    skills JSON NOT NULL,
    equipment JSON NOT NULL,
    spells JSON NOT NULL,
    features JSON NOT NULL,
    notes TEXT NOT NULL,
    avatar_url TEXT,
    is_npc BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Maps
CREATE TABLE maps (
    id TEXT PRIMARY KEY,
    campaign_id TEXT NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    image_url TEXT NOT NULL,
    grid_size INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    tokens JSON NOT NULL,
    fog_of_war JSON,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Assets
CREATE TABLE assets (
    id TEXT PRIMARY KEY,
    campaign_id TEXT REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    asset_type TEXT NOT NULL, -- enum: map, token, portrait, etc.
    file_size INTEGER NOT NULL,
    mime_type TEXT NOT NULL,
    tags JSON NOT NULL,
    created_at TIMESTAMP NOT NULL
);

