<template>
    <h1>Debug chat</h1>
    <p>Connection address:</p>
    <input v-model="connection_address">
    <button @click="connect">Connect</button>
    <div v-if="connected">
        <DebugChatOutput/>
        <DebugChatInput/>
    </div>
</template>

<script lang="ts">
import { defineComponent, toHandlers } from 'vue';
import DebugChatInput from './DebugChatInput.vue';
import DebugChatOutput from './DebugChatOutput.vue';
import { GameOutputMessage }  from '@/components/game/backend/.generated/game_interface/game_output/message'
import { Struct } from '@/components/game/backend/.generated/game_interface/game_output/google/protobuf/struct'

export default defineComponent({
    name: 'DebugChat',
    components: {
        DebugChatInput,
        DebugChatOutput
    },
    props: {
        chat_room_id: Number,
    },
    data() {
        return {
            connection_address: "" as string,
            connected: false,
            socket: undefined as WebSocket | undefined,
        }
    },
    methods: {
        connect() {
            if (this.socket) {
                this.connected = false;
                this.socket.close()
            }

            if (!this.connection_address) {
                return;
            }

            this.socket = new WebSocket(this.connection_address);
            this.socket.binaryType = "arraybuffer";
            
            this.socket.addEventListener("open", (event) => {
                this.connected = true;
            })

            this.socket.addEventListener("message", (event) => {
                let data = event.data;
                let message = GameOutputMessage.decode(new Uint8Array(data as ArrayBuffer));
                if (message.ui) {
                    if (message.ui.data?.typeUrl == 'type.googleapis.com/google.protobuf.Struct') {
                        let struct = Struct.decode(message.ui.data?.value!);
                        struct.fields.message;
                    }
                }
            })
        }
    }
})
</script>

