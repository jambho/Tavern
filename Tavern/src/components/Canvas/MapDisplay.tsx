import React, { useState, useRef, useCallback, useEffect } from 'react';
import { extend } from '@pixi/react';
import { Texture, Container, Graphics, Assets, Sprite, FederatedWheelEvent, FederatedPointerEvent } from 'pixi.js';
import { invoke } from '@tauri-apps/api/core';
import TokenDisplay from './TokenDisplay';

extend({ Texture, Graphics, Container, Sprite, FederatedWheelEvent, FederatedPointerEvent });

await Assets.init({
    basePath: '../../../',

});

const map = await Assets.load('GL_EldritchChurch_DeepSea.jpg');
const token = await Assets.load('Token.png');

interface Position {
  x: number;
  y: number;
}

interface TokenDisplayProps {
    tokenId: number;
    tokenPosition: Position;
    size: number;
    texture: Texture;
    imgUrl?: string;
    scale: number;
}
const MapDisplay: React.FC<any> = () => {
    
    const spriteRef = useRef<Sprite>(null);
    const [image, setImage] = useState<Texture | null>(null);
    const [isHover, setIsHover] = useState(false);
    const [isActive, setIsActive] = useState(false);
    const [cursor, setCursor] = useState('pointer');
    const [gridSize, setGridSize] = React.useState(30);
    const [imageSize, setImageSize] = React.useState(30);

    const drawCallback = useCallback<(graphics: Graphics) => void>((graphics) => {
    graphics.clear();
    // Vertical lines
    for (let i = 0; i < 364; i++) {
        graphics.moveTo(i * 30, 0).lineTo(i * 30, 7980).stroke({ color: 0x000000, width: 1, pixelLine: true, alpha: 0.5 });
    }

    // Horizontal lines
    for (let i = 0; i < 798; i++) {
        graphics.moveTo(0, i * 30).lineTo(3640, i * 30).stroke({ color: 0x000000, width: 1, pixelLine: true, alpha: 0.5 });
    }
    }, []);    

    const handlePointerDown = (event: FederatedPointerEvent) => {
        setIsActive(true);
        if (spriteRef.current && spriteRef.current.parent) {
            spriteRef.current.alpha = 0.5;
            spriteRef.current.parent.on('pointermove', (event: FederatedPointerEvent) => {
            onDragMove(event);
        });
        }

    };
    const handlePointerUp = (event: FederatedPointerEvent) => {
        setIsActive(false);
        if(spriteRef.current && spriteRef.current.parent) {
            spriteRef.current.alpha = 1;
            spriteRef.current.parent.off('pointermove');
        }
    };
    const onDragMove = (event: FederatedPointerEvent) => {

        if ( spriteRef.current && spriteRef.current.parent) {
            spriteRef.current.parent.toLocal(event.global, undefined, spriteRef.current.position);
            var newPosition = spriteRef.current.position;

            spriteRef.current.position.set(newPosition.x-(newPosition.x%30), newPosition.y-(newPosition.y%30));
        }
    };

    /*useEffect(() => {
        if (texture === Texture.EMPTY && imgUrl) {
            Assets.load(imgUrl).then((loadedTexture) => {
                setImage(loadedTexture);
            });
        }
    }, [texture, imgUrl]);*/

    return (
                    <pixiContainer
                        position={{ x: 0, y: 0 }}
                        scale={imageSize / 30}
                        eventMode='static'
                        onWheel={(e: FederatedWheelEvent) => {
                            const delta = e.deltaY > 0 ? -1 : 1;
                            setImageSize((prev) => Math.max(1, prev + delta));
                        }}
                        onPointerDown={(event: FederatedPointerEvent) => {
                            handlePointerDown(event);
                            setCursor('grabbing');
                        }}
                        onPointerUp={(event: FederatedPointerEvent) => {
                            handlePointerUp(event);
                            setCursor('pointer');
                        }}
                        onPointerUpOutside={(event: FederatedPointerEvent) => {
                            handlePointerUp(event);
                            setCursor('pointer');
                        }}
                    >
                        <pixiSprite
                            texture={map}
                        />
                            <TokenDisplay
                                tokenId={1}
                                tokenPosition={{ x: 360, y: 360 }}
                                size={30}
                                texture={token}
                                scale={imageSize / 30}
                            />
                            <pixiGraphics
                                scale={gridSize / 30}
                                draw={drawCallback}
                            />
                    </pixiContainer>
    )
}

export default MapDisplay;


// in GameBoard, handleTokenHover() is a callback that returns the component and then later on theres conditional rendering like tokenHovered ? handleTokenHover() : null