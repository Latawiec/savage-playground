import type { App } from 'vue';
import GameRenderer from "./components/GameRenderer.vue";
import { SceneUpdate } from './.gen/proto/scene_update';

function install(app: App) {
    app.component('GameRenderer', GameRenderer);
}

export { SceneUpdate, GameRenderer, install }