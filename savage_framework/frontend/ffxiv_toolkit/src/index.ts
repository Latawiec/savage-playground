import type { App } from 'vue';
import InputHandler from "./components/InputHandler.vue";
import CharacterSelection from './components/CharacterSelection.vue';
import { GameRenderer, SceneUpdate } from 'game-renderer-frontend';
import { FfxivGameInput, FfxivGameOutput } from './.gen/proto/ffxiv_toolkit';


export default {
    install(app: App) {
        app.component('InputHandler', InputHandler);
        app.component('GameRenderer', GameRenderer);
    }
}

export { InputHandler, CharacterSelection, GameRenderer, SceneUpdate, FfxivGameInput, FfxivGameOutput }