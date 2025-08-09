import React, { useState, useRef, useEffect } from 'react';
import { extend } from '@pixi/react';
import { Texture, Container, Graphics, Assets, Sprite, FederatedWheelEvent, FederatedPointerEvent } from 'pixi.js';

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
    gridScale: number;
}
const TokenDisplay: React.FC<TokenDisplayProps> = (
   {tokenId,
    tokenPosition,
    size,
    texture,
    imgUrl,
    scale,
    gridScale
}
) => {
    
    const spriteRef = useRef<Sprite>(null);
    const [image, setImage] = useState<Texture | null>(null);
    const [isHover, setIsHover] = useState(false);
    const [isActive, setIsActive] = useState(false);
    const handlePointerDown = (event: FederatedPointerEvent) => {
        setIsActive(true);
        if(spriteRef.current) {
            console.log("Sprite reference is valid");
        }
        if (spriteRef.current && spriteRef.current.parent) {
            spriteRef.current.alpha = 0.5;
            console.log('Refs are checked');
            spriteRef.current.parent.on('pointermove', (event: FederatedPointerEvent) => {
                event.preventDefault();
                event.stopPropagation();
                console.log('Pointer move on token:', tokenId, 'at position:', tokenPosition);
                onDragMove(event);
            });
        }
        console.log('Pointer down on token:', tokenId, 'at position:', tokenPosition);
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

            spriteRef.current.position.set(newPosition.x-(newPosition.x%gridScale), newPosition.y-(newPosition.y%gridScale));
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
            eventMode='static'
            interactive={true}
            onPointerOver={() => setIsHover(true)}
            onPointerOut={() => setIsHover(false)}
            onPointerDown={(event: FederatedPointerEvent) => {
                event.preventDefault();
                handlePointerDown(event);
                event.stopPropagation();

            }}
            onPointerUp={(event: FederatedPointerEvent) => {
                event.preventDefault();
                handlePointerUp(event);
                event.stopPropagation();

            }}
            onPointerUpOutside={(event: FederatedPointerEvent) => {
                event.preventDefault();
                handlePointerUp(event);
                event.stopPropagation();

            }}
            onPointerMove={(event: FederatedPointerEvent) => {
                event.preventDefault();
                event.stopPropagation();
            }}
        />
    )
}

export default TokenDisplay;

