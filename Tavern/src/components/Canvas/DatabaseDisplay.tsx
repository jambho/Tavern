import React, { useState, useRef, useCallback, useEffect } from 'react';
import { useTheme } from '../../contexts/ThemeContext';
import { invoke } from '@tauri-apps/api/core';

const campaigns = await invoke('get_campaigns');
console.log(campaigns);
const fighter = invoke('get_character', { characterId: "db4f64f8-3465-450f-a175-71c8a21c92ad", campaignId: "d6373db8-12cd-4ef5-972d-c942070c07df"})
console.log(fighter);

const DatabaseDisplay: React.FC = () => {

    return <div>
    </div>
}


export default DatabaseDisplay;