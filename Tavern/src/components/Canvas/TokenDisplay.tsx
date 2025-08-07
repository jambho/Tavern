import React, { useState, useRef, useCallback, useEffect } from 'react';
import { useTheme } from '../../contexts/ThemeContext';
import { invoke } from '@tauri-apps/api/core';

interface Position {
  x: number;
  y: number;
}

interface TokenDisplayProps {
    tokenId: string;
    tokenPosition: Position;
}


const TokenDisplay: React.FC<TokenDisplayProps> = (
   {tokenId,
    tokenPosition}
) => {
    const [tokenInfo, setTokenInfo] = useState<String>("NO TOKEN INFO"); 
    const [activeCampaign, setActiveCampaign] = useState<String>("NO ACTIVE CAMPAIGN"); 

   
    const handleCampaignIdUpdate = (campaignId: String) => {
        if(campaignId != activeCampaign) {
            setActiveCampaign(campaignId);
        }
    }
    const handleTokenInfoUpdate = (tInfo: String) => {
        if(tInfo != tokenInfo) {
            setTokenInfo(tInfo);
        }
    }
    //get all token data
    invoke('get_active_campaign_id').then((cId) => handleCampaignIdUpdate(cId as String));
    invoke('get_character', { characterId: tokenId, activeCampaign }).then((t) => handleTokenInfoUpdate(JSON.stringify(t as Object)));

    console.log(activeCampaign);
    console.log(tokenInfo);
    //format all token data & return it


    return (
        <div
            style={{
              position: "fixed",
              left: tokenPosition.x,
              top: tokenPosition.y,
              backgroundColor: "rgba(0, 0, 0, 0.8)",
              color: "white",
              padding: "4px 8px",
              borderRadius: "4px",
              fontSize: "28px",
              pointerEvents: "none",
              zIndex: 1000,
              transform: "translate(-50%, -100%)",
              whiteSpace: "pre-line",
              textAlign: "center",
            }}
        
        > 
            
            {tokenId} 
            
        </div>
    )
}

export default TokenDisplay;


// in GameBoard, handleTokenHover() is a callback that returns the component and then later on theres conditional rendering like tokenHovered ? handleTokenHover() : null