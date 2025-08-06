import React, { useState, useRef, useCallback, useEffect } from 'react';
import { useTheme } from '../../contexts/ThemeContext';
import { invoke } from '@tauri-apps/api/core';

interface Position {
  x: number;
  y: number;
}

interface TokenDisplayProps {
    tokenId: number;
    tokenPosition: Position;
}
const TokenDisplay: React.FC<TokenDisplayProps> = (
   {tokenId,
    tokenPosition}
) => {
    //get all token data
    //tokenInfo = invoke('get_character', tokenId, campaignId);
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