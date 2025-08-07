import React, { useState, useRef, useCallback, useEffect } from 'react';
import { useTheme } from '../../contexts/ThemeContext';
import { invoke } from '@tauri-apps/api/core';

const campaigns: Array<Object> = await invoke('get_campaigns');
console.log(campaigns[0]);
const characters: Array<Object> = await invoke('get_characters', { campaignId: "d6373db8-12cd-4ef5-972d-c942070c07df"});
console.log(characters[0]);

let campaignString = JSON.stringify(campaigns, null, 2);
let charactersString = JSON.stringify(characters, null, 2);
let fighterStatsString = JSON.stringify(characters[0].stats, null, 2);


const DatabaseDisplay: React.FC = () => {
    
    const [isVisibleCampaigns, setIsVisibleCampaigns] = useState(false);
    const [isVisibleCharacters, setIsVisibleCharacters] = useState(false);
    return (
    <div>
        <div>
        <h1>Database Display</h1>
        <div className= 'row'>
            <div className = "column">
                <button onClick={() => setIsVisibleCampaigns(v => !v)}>Show Campaigns</button>
                <pre>{isVisibleCampaigns ? campaignString : null}</pre>
            </div>
            <div className = "column">
                <button onClick={() => setIsVisibleCharacters(v => !v)}>Show Characters</button>
                <pre>{isVisibleCharacters ? charactersString : null}</pre>
            </div>
            </div>
        </div>
    </div>
    )
}


export default DatabaseDisplay;