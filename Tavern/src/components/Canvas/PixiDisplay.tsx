import React from 'react';
import { Application } from '@pixi/react';
import MapDisplay from './MapDisplay';

const PixiDisplay: React.FC = () => {
    return      <Application
                    resizeTo={window}
                    backgroundColor={0xFFFFFF}
                    antialias={true}
                >
                    <MapDisplay/>
                </Application>
};

export default PixiDisplay;
