import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type Stats = {
    strength: number;
    dexterity: number;
    constitution: number;
    intelligence: number;
    wisdom: number;
    charisma: number;
};

type CreateCharacterRequest = {
    campaign_id: string;
    player_name: string;
    name: string;
    level: number;
    race: string;
    character_class: string;
    stats: Stats;
    background: string;
    proficiency_bonus: number;
    is_npc: boolean;
    // Adding other potential required fields
    hit_points?: number;
    armor_class?: number;
    speed?: number;
    initiative?: number;
};

const defaultStats: Stats = {
    strength: 10,
    dexterity: 10,
    constitution: 10,
    intelligence: 10,
    wisdom: 10,
    charisma: 10,
};

// Helper function to calculate proficiency bonus based on level
const calculateProficiencyBonus = (level: number): number => {
    return Math.ceil(level / 4) + 1;
};

// Helper function to calculate modifier from ability score
const calculateModifier = (score: number): number => {
    return Math.floor((score - 10) / 2);
};

const TokenCreator: React.FC = () => {
    const [name, setName] = useState("");
    const [race, setRace] = useState("");
    const [campaignId, setCampaignId] = useState("d6373db8-12cd-4ef5-972d-c942070c07df");
    const [charClass, setCharClass] = useState("");
    const [level, setLevel] = useState(1);
    const [stats, setStats] = useState<Stats>(defaultStats);
    const [loading, setLoading] = useState(false);
    const [result, setResult] = useState<string | null>(null);
    const [error, setError] = useState<string | null>(null);

    const handleStatChange = (stat: keyof Stats, value: number) => {
        setStats((prev) => ({ ...prev, [stat]: value }));
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setLoading(true);
        setError(null);
        setResult(null);

        const proficiencyBonus = calculateProficiencyBonus(level);
        const constitutionModifier = calculateModifier(stats.constitution);
        const dexterityModifier = calculateModifier(stats.dexterity);

        const characterData: CreateCharacterRequest = {
            campaign_id: campaignId,
            player_name: name,
            name,
            level,
            race,
            character_class: charClass,
            stats,
            background: "Folk Hero", // Default background
            proficiency_bonus: proficiencyBonus,
            is_npc: false,
            hit_points: 8 + constitutionModifier, // Assuming d8 hit die + con mod
            armor_class: 10 + dexterityModifier, // Base AC + dex mod
            speed: 30, // Standard speed
            initiative: dexterityModifier,
        };

        try {
            console.log("Sending character data:", characterData);
            const id = await invoke<string>("create_character", { request: characterData });
            setResult(`Character created with ID: ${id}`);
        } catch (err: any) {
            console.error("Full error:", err);
            setError(JSON.stringify(err, null, 2));
        } finally {
            setLoading(false);
        }
    };

    return (
        <div style={{
            maxWidth: 500,
            margin: "2rem auto",
            padding: "2rem",
            borderRadius: "1rem",
            background: "linear-gradient(135deg, #232526 0%, #414345 100%)",
            boxShadow: "0 4px 24px rgba(0,0,0,0.2)",
            color: "#fff"
        }}>
            <h2 style={{ textAlign: "center", marginBottom: "1.5rem" }}>Create Character Token</h2>
            <form onSubmit={handleSubmit} style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
                <input
                    type="text"
                    placeholder="Character Name"
                    value={name}
                    required
                    onChange={e => setName(e.target.value)}
                    style={inputStyle}
                />
                <input
                    type="text"
                    placeholder="Race (e.g., Human, Elf, Dwarf)"
                    value={race}
                    required
                    onChange={e => setRace(e.target.value)}
                    style={inputStyle}
                />
                <input
                    type="text"
                    placeholder="Class (e.g., Fighter, Wizard, Rogue)"
                    value={charClass}
                    required
                    onChange={e => setCharClass(e.target.value)}
                    style={inputStyle}
                />
                <div style={{ display: "flex", alignItems: "center", gap: "1rem" }}>
                    <label>Level:</label>
                    <input
                        type="number"
                        min={1}
                        max={20}
                        value={level}
                        onChange={e => setLevel(Number(e.target.value))}
                        style={{ ...inputStyle, width: "80px" }}
                    />
                    <span style={{ fontSize: "0.9em", color: "#ccc" }}>
                        Proficiency Bonus: +{calculateProficiencyBonus(level)}
                    </span>
                </div>
                <div>
                    <label style={{ fontWeight: "bold", marginBottom: "0.5rem", display: "block" }}>Ability Scores</label>
                    <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "0.5rem" }}>
                        {Object.entries(stats).map(([stat, value]) => (
                            <div key={stat} style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
                                <label style={{ fontSize: "0.85em", minWidth: "70px" }}>
                                    {stat.charAt(0).toUpperCase() + stat.slice(1)}:
                                </label>
                                <input
                                    type="number"
                                    min={1}
                                    max={20}
                                    value={value}
                                    onChange={e => handleStatChange(stat as keyof Stats, Number(e.target.value))}
                                    style={{ ...inputStyle, width: "60px", marginBottom: 0 }}
                                />
                                <span style={{ fontSize: "0.8em", color: "#aaa", minWidth: "25px" }}>
                                    ({calculateModifier(value) >= 0 ? '+' : ''}{calculateModifier(value)})
                                </span>
                            </div>
                        ))}
                    </div>
                </div>
                <button
                    type="submit"
                    disabled={loading}
                    style={{
                        padding: "0.75rem",
                        borderRadius: "0.5rem",
                        border: "none",
                        background: loading ? "#555" : "#6a82fb",
                        color: "#fff",
                        fontWeight: "bold",
                        cursor: loading ? "not-allowed" : "pointer",
                        boxShadow: "0 2px 8px rgba(106,130,251,0.2)"
                    }}
                >
                    {loading ? "Creating..." : "Create Character"}
                </button>
                {result && (
                    <div style={{ 
                        color: "#4ade80", 
                        marginTop: "1rem", 
                        padding: "0.5rem", 
                        background: "rgba(74, 222, 128, 0.1)", 
                        borderRadius: "0.5rem" 
                    }}>
                        {result}
                    </div>
                )}
                {error && (
                    <div style={{ 
                        color: "#ff6b6b", 
                        marginTop: "1rem", 
                        padding: "0.5rem", 
                        background: "rgba(255, 107, 107, 0.1)", 
                        borderRadius: "0.5rem",
                        fontSize: "0.85em",
                        whiteSpace: "pre-wrap"
                    }}>
                        {error}
                    </div>
                )}
            </form>
        </div>
    );
};

const inputStyle: React.CSSProperties = {
    padding: "0.5rem",
    borderRadius: "0.4rem",
    border: "1px solid #444",
    background: "#2c2c2c",
    color: "#fff",
    fontSize: "1rem",
    marginBottom: "0.5rem"
};

export default TokenCreator;