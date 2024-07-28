import type { App } from 'vue';
import InputHandler from "./components/InputHander.vue";
import { GameRenderer, SceneUpdate } from 'game-renderer-frontend';
import { FfxivGameInput, FfxivGameOutput } from './.gen/proto/ffxiv_toolkit';


function install(app: App) {
    app.component('InputHandler', InputHandler);
    app.component('GameRenderer', GameRenderer);
}

export { InputHandler, GameRenderer, SceneUpdate, FfxivGameInput, FfxivGameOutput, install }