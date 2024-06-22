<template>
    <h1>Debug chat</h1>
    <p>Connection address:</p>
    <input v-model="connection_address">
    <button @click="connect">Connect</button>
    <div v-if="connected">
        <DebugChatOutput ref="chat_output"/>
        <DebugChatInput ref="chat_input" @message-sent="send"/>
    </div>
</template>

<script lang="ts">
import { defineComponent, toHandlers } from 'vue';
import DebugChatInput from './DebugChatInput.vue';
import DebugChatOutput from './DebugChatOutput.vue';
import { GameOutputMessage }  from '@/components/game/backend/.generated/game_interface/game_output/message'
import { ClientOutput } from '@/components/game/backend/.generated/room_server_interface/client_output'
import { Struct } from '@/components/game/backend/.generated/game_interface/game_output/google/protobuf/struct'
import { send } from 'process';

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
                let message = ClientOutput.decode(new Uint8Array(data as ArrayBuffer));
                console.log(message);
            })
        },
        send(message: string) {
            if (this.socket) {
                let proto_struct = Struct.create();
                proto_struct.fields["message"] = message;
                let bytes = Struct.encode(proto_struct).finish();
                this.socket.send(bytes)
            }
        }
    }
})
</script>

