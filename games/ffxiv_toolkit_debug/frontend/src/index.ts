import type { App } from 'vue';
import FfxivToolkitDebug from "./components/FfxivToolkitDebug.vue";

function install(app: App) {
    app.component('FfxivToolkitDebug', FfxivToolkitDebug);
}

export { FfxivToolkitDebug, install }