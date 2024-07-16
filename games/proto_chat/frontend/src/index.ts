import type { App } from 'vue';
import ProtoChat from "./components/ProtoChat.vue";

function install(app: App) {
    app.component('ProtoChat', ProtoChat);
}


export { ProtoChat, install }