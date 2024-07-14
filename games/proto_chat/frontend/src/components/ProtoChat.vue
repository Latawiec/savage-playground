<template>
    <div class="proto-chat-main container-fluid">
        <h1>ProtoChat</h1>

        <div class="row">
            <div class="input-group m-3">
                <span class="input-group-text">ws://</span>
                <input ref="server_address_input" class="form-control" aria-label="Server address">
                <span class="input-group-text">/</span>
                <input ref="connect_path_input" class="form-control" aria-label="Server path">
                <button v-if="!connected" class="btn btn-outline-secondary" type="button" @click="connect">Connect</button>
                <button v-if="connected" class="btn btn-danger" type="button" @click="disconnect">Disconnect</button>
            </div>
        </div>

        <div v-if="connected" class="container-fluid">
            <div class="row">
                <div class="col-12">
                    <p v-for="message in chat_messages" :key="message">{{ message }}</p>
                </div>
                <div class="input-group col-12">
                    <textarea class="form-control" aria-label="Type your message here..."></textarea>
                    <button class="btn btn-success">Send</button>
                </div>
            </div>
        </div>

    </div>
</template>

<script lang="ts">
import 'bootstrap/dist/js/bootstrap.min.js'
// If variables are not exported globally, they're not visible. So I need to do this.
// I need to make sure it doesn't leak to any other style though.
// import './bootstrap-core.scss'

import { defineComponent } from 'vue';
import { ProtoChatHistory, ProtoChatMessage, ProtoChatRequest, ProtoChatRequestType } from '../.gen/proto/proto_chat'
import { Any } from 'game_interface/proto/google/protobuf/any'

// require("../../node_modules/bootstrap/dist/css/bootstrap.css")

export default defineComponent({
    name: 'ProtoChat',
    data() {
        return {
            connection_address: undefined as string | undefined,
            connected: false,
            socket: undefined as WebSocket | undefined,
            chat_messages: [] as string[],
        }
    },
    methods: {
        connect() {
            let server_address = (this.$refs.server_address_input as HTMLInputElement).value;
            let connect_path = (this.$refs.connect_path_input as HTMLInputElement).value;

            if (this.connected) {
                console.error("Already connected.");
                return;
            }

            this.socket = new WebSocket(`ws://${server_address}/${connect_path}`);
            this.socket.binaryType = "arraybuffer";

            this.socket.addEventListener("open", event => {
                this.connected = true;
                this.load_history();
                (this.$refs.server_address_input as HTMLInputElement).disabled = true;
                (this.$refs.connect_path_input as HTMLInputElement).disabled = true;
            });

            this.socket.addEventListener("close", event => {
                this.connected = false;
                this.socket = undefined;
                (this.$refs.server_address_input as HTMLInputElement).disabled = false;
                (this.$refs.connect_path_input as HTMLInputElement).disabled = false;
            });

            this.socket.addEventListener("message", event => {
                let data = event.data;
                let message = Any.decode(new Uint8Array(data as ArrayBuffer));
                
                if (message.typeUrl === 'proto_chat.history') {
                    let chat_history = ProtoChatHistory.decode(new Uint8Array(message.value));
                    this.chat_messages = [];
                    for (var chat_message of chat_history.history) {
                        this.chat_messages.push(`${chat_message.userId}: ${chat_message.userMessage}`);
                    }
                } else
                if (message.typeUrl === 'proto_chat.message') {
                    let chat_message = ProtoChatMessage.decode(new Uint8Array(message.value));
                    this.chat_messages.push(`${chat_message.userId}: ${chat_message.userMessage}`);
                }
            })
        },
        disconnect() {
            if (this.socket) {
                this.socket.close();
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

<style lang="scss" scoped>
@import "bootstrap/scss/bootstrap";
</style>
