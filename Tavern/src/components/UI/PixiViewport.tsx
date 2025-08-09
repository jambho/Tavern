import { autoDetectRenderer, Container, EventSystem, Graphics, Sprite, Text } from "pixi.js";
import { IViewportOptions, Viewport } from "pixi-viewport";
import { extend, PixiReactElementProps } from "@pixi/react";

//import type { PixiReactElementProps } from "@pixi/react/types/typedefs/PixiReactNode";
extend({ EventSystem})

class CustomViewport extends Viewport {
  constructor(
    options: IViewportOptions & {
      decelerate?: boolean;
      drag?: boolean;
      pinch?: boolean;
      wheel?: boolean;
    }
  ) {
    const { decelerate, drag, pinch, wheel, ...rest } = options;
    super(rest);
    if (decelerate) this.decelerate();
    if (drag) this.drag();
    if (pinch) this.pinch();
    if (wheel) this.wheel();
  }
}

declare module "@pixi/react" {
  interface PixiElements {
    pixiCustomViewport: PixiReactElementProps<typeof CustomViewport>;

  }
}



extend({ Container, Graphics, Sprite, Text, CustomViewport });
const renderer = await autoDetectRenderer({
  width: 800,
  height: 600,
  antialias: true,
});
const PixiViewport: React.FC<PixiReactElementProps<typeof CustomViewport>> = ({ children, events }) => {
    //const events = new EventSystem(renderer);
    return (
        <pixiCustomViewport events={events} drag pinch wheel>
            {children}
        </pixiCustomViewport>
    );
}
export default PixiViewport;



