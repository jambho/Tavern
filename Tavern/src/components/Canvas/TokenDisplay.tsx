import React, { useState, useRef, useCallback, useEffect } from 'react';
import { extend } from '@pixi/react';
import { Texture, Container, Graphics, Assets, Sprite, FederatedWheelEvent, FederatedPointerEvent } from 'pixi.js';
import { invoke } from '@tauri-apps/api/core';

extend({ Texture, Graphics, Container, Sprite, FederatedWheelEvent, FederatedPointerEvent });

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
const TokenDisplay: React.FC<TokenDisplayProps> = (
   {tokenId,
    tokenPosition,
    size,
    texture,
    imgUrl,
    scale
}
) => {
    
    const spriteRef = useRef<Sprite>(null);
    const [image, setImage] = useState<Texture | null>(null);
    const [isHover, setIsHover] = useState(false);
    const [isActive, setIsActive] = useState(false);
    const [cursor, setCursor] = useState('pointer');

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

    useEffect(() => {
        if (texture === Texture.EMPTY && imgUrl) {
            Assets.load(imgUrl).then((loadedTexture) => {
                setImage(loadedTexture);
            });
        }
    }, [texture, imgUrl]);

    return (
        <pixiSprite
            ref={spriteRef}
            texture={image || texture}
            anchor={0}
            x={tokenPosition.x}
            y={tokenPosition.y}
            width={size}
            height={size}
            zIndex={4}
            cursor={cursor}
            //scale={scale}
            eventMode='static'
            interactive={true}
            onPointerOver={() => setIsHover(true)}
            onPointerOut={() => setIsHover(false)}
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
        />
    )
}

export default TokenDisplay;


// in GameBoard, handleTokenHover() is a callback that returns the component and then later on theres conditional rendering like tokenHovered ? handleTokenHover() : null