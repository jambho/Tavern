import React, { useState, useRef, useCallback, useEffect } from 'react';
import { useTheme } from '../../contexts/ThemeContext';
import { invoke } from '@tauri-apps/api/core';



const TokenDisplay: React.FC<{tokenId: number}> = (
    {tokenId},
) => {
    //get all token data
    const tokenIdRef = useRef(tokenId)


    //format all token data & return it
    return (
        <> {tokenId} </>
    )
}

export default TokenDisplay;


// in GameBoard, handleTokenHover() is a callback that returns the component and then later on theres conditional rendering like tokenHovered ? handleTokenHover() : null