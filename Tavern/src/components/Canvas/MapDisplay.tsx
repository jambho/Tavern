import React, { useState, useRef, useCallback, useEffect } from 'react';
import { extend, useApplication } from '@pixi/react';
import { Texture, Container, Graphics, Assets, Sprite, FederatedWheelEvent, FederatedPointerEvent } from 'pixi.js';
import TokenDisplay from './TokenDisplay';
import PixiViewport from '../UI/PixiViewport';
extend({ Texture, Graphics, Container, Sprite, FederatedWheelEvent, FederatedPointerEvent });

await Assets.init({
    basePath: '../../../',

});

const map = await Assets.load('GL_EldritchChurch_DeepSea.jpg');
const token = await Assets.load('Token.png');

const MapDisplay: React.FC<any> = () => {
    const app = useApplication();

    const containerRef = useRef<Container>(null);
    const [gridSize, setGridSize] = React.useState(90);
    const [imageSize, setImageSize] = React.useState(30);

    const drawCallback = useCallback<(graphics: Graphics) => void>((graphics) => {
        graphics.clear();
        // Vertical lines
        for (let i = 0; i < 122; i++) {
            graphics.moveTo(i * gridSize, 0).lineTo(i * gridSize, 7980).stroke({ color: 0x000000, width: 1, pixelLine: true, alpha: 0.5 });
        }

        // Horizontal lines
        for (let i = 0; i < 266; i++) {
            graphics.moveTo(0, i * gridSize).lineTo(3640, i * gridSize).stroke({ color: 0x000000, width: 1, pixelLine: true, alpha: 0.5 });
        }
    }, []);    

    const handleRightDown = (event: FederatedPointerEvent) => {
        if (containerRef.current && containerRef.current.parent) {
            containerRef.current.parent.on('pointermove', (event: FederatedPointerEvent) => {
            onDragMove(event);
        });
        }

    };
    const handleRightUp = (event: FederatedPointerEvent) => {
        if(containerRef.current && containerRef.current.parent) {
            containerRef.current.parent.off('pointermove');
        }
    };
    const onDragMove = (event: FederatedPointerEvent) => {
        if (containerRef.current && containerRef.current.parent) {
            containerRef.current.parent.toLocal(event.global, undefined, containerRef.current.position);
        }
    };

    return (    <PixiViewport events={app.app.renderer.events}>
                <pixiContainer
                    ref={containerRef}
                    position={{ x: 0, y: 0 }}
                    //scale={imageSize / 30}
                    eventMode='static'
                    /*onWheel={(e: FederatedWheelEvent) => {
                        const delta = e.deltaY > 0 ? -1 : 1;
                        setImageSize((prev) => Math.max(1, prev + delta));
                    }}
                    onPointerDown={(event: FederatedPointerEvent) => {
                        handleRightDown(event);
                    }}
                    onPointerUp={(event: FederatedPointerEvent) => {
                        handleRightUp(event);
                    }}
                    onPointerUpOutside={(event: FederatedPointerEvent) => {
                        handleRightUp(event);
                    }}*/
                >
                    <pixiSprite
                        texture={map}
                    />
                        <TokenDisplay
                            tokenId={1}
                            tokenPosition={{ x: 360, y: 360 }}
                            size={gridSize}
                            texture={token}
                            scale={imageSize}
                            gridScale={gridSize}
                        />
                        <pixiGraphics
                            //scale={gridSize}
                            draw={drawCallback}
                        />
                    </pixiContainer>
                </PixiViewport>
    )
}

export default MapDisplay;
