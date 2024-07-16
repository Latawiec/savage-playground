import type { App } from 'vue';
import ProtoChat from "./components/ProtoChat.vue";

export default {
    install(app: App) {
        app.component('ProtoChat', ProtoChat);
    }
}

export { ProtoChat }
