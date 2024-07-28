import type { App } from 'vue';
import FfxivToolkitDebug from "./components/FfxivToolkitDebug.vue";
import FfxivToolkitFrontend from "ffxiv-toolkit-frontend";

export default {
    install(app: App) {
        FfxivToolkitFrontend.install(app);
        app.component('FfxivToolkitDebug', FfxivToolkitDebug);
    }
}

export { FfxivToolkitDebug }