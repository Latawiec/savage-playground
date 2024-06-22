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
import { ProtoChatMessage, ProtoChatHistory, ProtoChatRequest, ProtoChatRequestType } from '@/components/game/backend/.generated/debug/proto_chat/proto_chat'
import { ClientOutput } from '@/components/game/backend/.generated/room_server_interface/client_output'
import { Any } from '@/components/game/backend/.generated/game_interface/game_output/google/protobuf/any'
import { send } from 'process';

export default defineComponent({
    name: 'DebugChat',
    components: {
        DebugChatInput,
        DebugChatOutput
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
            console.log(`... Connecting to: ${this.connection_address}`);
            this.socket = new WebSocket(this.connection_address);
            this.socket.binaryType = "arraybuffer";
            
            this.socket.addEventListener("open", (event) => {
                this.connected = true;
                this.load_history();
                console.log(`Connected to: ${this.connection_address}`);
            })

            this.socket.addEventListener("message", (event) => {
                let data = event.data;
                let message = Any.decode(new Uint8Array(data as ArrayBuffer));
                if (message.typeUrl === 'proto_chat.history') {
                    let chat_history = ProtoChatHistory.decode(new Uint8Array(message.value));
                    (this.$refs.chat_output as InstanceType<typeof DebugChatOutput>).clear();
                    for (var chat_message of chat_history.history) {
                        (this.$refs.chat_output as InstanceType<typeof DebugChatOutput>).write(chat_message.userMessage!);
                    }
                } else
                if (message.typeUrl === 'proto_chat.message') {
                    let chat_message = ProtoChatMessage.decode(new Uint8Array(message.value));
                    (this.$refs.chat_output as typeof DebugChatOutput).write(chat_message.userMessage);
                }

                console.log(message);
            })
        },
        send(message: string) {
            if (this.socket) {
                let proto_chat_message = ProtoChatMessage.create();
                proto_chat_message.userMessage = message;
                proto_chat_message.userId = 0;
                let bytes = ProtoChatMessage.encode(proto_chat_message).finish();

                console.log(proto_chat_message);
                
                let proto_msg = Any.create();
                proto_msg.typeUrl = 'proto_chat.message';
                proto_msg.value = bytes;
                let proto_bytes = Any.encode(proto_msg).finish();

                this.socket.send(proto_bytes);
            }
        },
        load_history() {
            if (this.socket) {
                let proto_chat_request = ProtoChatRequest.create();
                proto_chat_request.requests.push(ProtoChatRequestType.History);
                let bytes = ProtoChatRequest.encode(proto_chat_request).finish();

                let proto_msg = Any.create();
                proto_msg.typeUrl = 'proto_chat.request';
                proto_msg.value = bytes;
                let proto_bytes = Any.encode(proto_msg).finish();

                this.socket.send(proto_bytes);
            }
        }
    }
})
</script>

