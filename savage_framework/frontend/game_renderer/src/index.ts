import type { App } from 'vue';
import GameRenderer from "./components/GameRenderer.vue";
import { SceneUpdate } from './.gen/proto/game_renderer';

import { MemoryAssetStorage } from './asset_storage/MemoryAssetStorage';

export default {
    install(app: App) {
        app.component('GameRenderer', GameRenderer);
    }
}
export type { IAssetStorage } from './asset_storage/IAssetStorage';
export { SceneUpdate, GameRenderer, MemoryAssetStorage }