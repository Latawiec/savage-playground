import type { App } from 'vue';
import InputHandler from "./components/InputHandler.vue";
import { GameRenderer, SceneUpdate } from 'game-renderer-frontend';
import { FfxivGameInput, FfxivGameOutput } from './.gen/proto/ffxiv_toolkit';


export default {
    install(app: App) {
        app.component('InputHandler', InputHandler);
        app.component('GameRenderer', GameRenderer);
    }
}

export { InputHandler, GameRenderer, SceneUpdate, FfxivGameInput, FfxivGameOutput }