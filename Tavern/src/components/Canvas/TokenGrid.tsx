import React, { useCallback, useRef } from 'react';
import { Application, extend } from '@pixi/react';
import {  } from '@pixi/events';
import { Container, Graphics, Assets, Sprite, FederatedWheelEvent, FederatedPointerEvent } from 'pixi.js';
import { app } from '@tauri-apps/api';


extend({ Graphics, Container, Sprite, FederatedWheelEvent, FederatedPointerEvent });

await Assets.init({
    basePath: '../../../',

});

const map = await Assets.load('GL_EldritchChurch_DeepSea.jpg');



const TokenGrid: React.FC = () => {
    const [gridSize, setGridSize] = React.useState(30);
    const [imageSize, setImageSize] = React.useState(30);

    const appRef = useRef<any>(null);

    React.useEffect(() => {
        if(appRef.current?.view){
            const canvas = appRef.current.view as HTMLCanvasElement;
            const handleWheel = (event: WheelEvent) => {
                event.preventDefault();
                const delta = Math.sign(event.deltaY);
                setImageSize((prev) => Math.max(1, prev + delta));
            };
            canvas.addEventListener('wheel', handleWheel, { passive: false });
            return () => {
                canvas.removeEventListener('wheel', handleWheel);
            };
        }
    }, []);

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


    // Handle grid size change
    const handleGridSizeChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
        const newGridSize = parseInt(e.target.value);
        setGridSize(newGridSize);
        
    };
    const handleImageSizeChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
        const newImageSize = parseInt(e.target.value);
        setImageSize(newImageSize);
    };


    return  <>
                <Application
                    resizeTo={window}
                    backgroundColor={0xFFFFFF}
                    antialias={true}>   
                    <pixiContainer
                        position={{ x: 0, y: 0 }}
                        scale={imageSize / 30}
                        eventMode='static'
                        onWheel={(e: FederatedWheelEvent) => {
                            e.originalEvent.preventDefault();
                            e.stopImmediatePropagation();
                            e.originalEvent.stopPropagation();
                            console.log('Wheel event:', e.propagationImmediatelyStopped);
                            console.log('Bubble:', e.bubbles);
                            console.log('Default:', e.defaultPrevented);
                            const delta = e.deltaY > 0 ? -1 : 1;
                            setImageSize((prev) => Math.max(1, prev + delta));
                            console.log('Wheel event:', e.deltaY, 'New image size:', imageSize + delta);
                        }}
                        onClick={(event: FederatedPointerEvent) => {
                            console.log('Click event:', event);
                        }}
                    >
                        <pixiSprite

                            texture={map}
                        />
                        <pixiGraphics
                            scale={gridSize / 30}
                            draw={drawCallback}
                        />
                    </pixiContainer>
                </Application>
                <label className="flex items-center gap-2">
                    <span className="text-sm text-gray-600">Grid Size:</span>
                        <input
                        type="range"
                        min="0"
                        max="100"
                        value={gridSize}
                        onChange={handleGridSizeChange}
                        className="w-20"
                        />
                    <span className="text-sm w-8">{gridSize}</span>
                </label>
                    <label className="flex items-center gap-2">
                        <span className="text-sm text-gray-600">Grid Size:</span>
                            <input
                            type="range"
                            min="0"
                            max="100"
                            value={imageSize}
                            onChange={handleImageSizeChange}
                            className="w-20"
                            />
                        <span className="text-sm w-8">{imageSize}</span>
                </label>
            </>
};

export default TokenGrid;
