import React, { useState, useRef, useCallback, useEffect } from 'react';
import { useTheme } from '../../contexts/ThemeContext';
import { invoke } from '@tauri-apps/api/core';

const campaigns: Array<Object> = await invoke('get_campaigns');
console.log(campaigns[0]);
const fighter: Object = await invoke('get_character', { characterId: "db4f64f8-3465-450f-a175-71c8a21c92ad", campaignId: "d6373db8-12cd-4ef5-972d-c942070c07df"});
console.log(fighter);
let campaignsJSONArray = JSON.stringify(campaigns, null, 2);
let fighterJSONObject = JSON.stringify(fighter, null, 2);

const DatabaseDisplay: React.FC = () => {

    return <div>
        <h1>Database Display</h1>
        <div>
        <h2>Campaigns</h2>
        <pre>{campaignsJSONArray}</pre>
        </div>
        <div>
        <h2>Fighter</h2>
        <pre>{fighterJSONObject}</pre>
        </div>
    </div>
}


export default DatabaseDisplay;