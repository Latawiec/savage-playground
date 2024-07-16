<template>
    <!-- Bootstrap needs either data-bs-theme="light" or :root accessor. -->
    <div data-bs-theme="light" class="container-fluid">
        <h1>ProtoChat</h1>

        <div class="row">
            <div class="input-group mb-2">
                <span class="input-group-text">ws://</span>
                <input ref="server_address_input" class="form-control" aria-label="Server address">
                <span class="input-group-text">/</span>
                <input ref="connect_path_input" class="form-control" aria-label="Server path">
                <button v-if="!connected" class="btn btn-outline-secondary" type="button" @click="connect">Connect</button>
                <button v-if="connected" class="btn btn-danger" type="button" @click="disconnect">Disconnect</button>
            </div>
        </div>

        <div v-if="connected">

            <div class="row m-0">
                <div class="border mb-2 rounded-2 col-12 mh-100" style="overflow: auto; height: 400px; display: flex; flex-direction: column-reverse; overflow-anchor: auto !important;">
                    <div style="transform: translateZ(0)">
                        <p v-for="message in chat_messages" class="text-wrap text-start" :key="message" style="transform: translateZ(0)">
                            <hr/>
                            <b>User {{ message.user_id }}:</b> 
                            <br>
                            {{ message.message }}
                        </p>
                    </div>
                </div>
            </div>

            <div class="row">
                <div class="input-group col-12 mb-2" >
                    <textarea ref="message_input" class="form-control" aria-label="Type your message here..." @keyup.enter="send"></textarea>
                    <button class="btn btn-success" @click="send">Send</button>
                </div>
            </div>

        </div>

    </div>
</template>

<script lang="ts">
import 'bootstrap/dist/js/bootstrap.min.js'
import { defineComponent } from 'vue';
import { ProtoChatHistory, ProtoChatMessage, ProtoChatRequest, ProtoChatRequestType } from '../.gen/proto/proto_chat'
import { Any } from 'game_interface/proto/google/protobuf/any';

class Message {
    user_id: number;
    message: string;

    constructor(user_id: number, message: string) {
        this.user_id = user_id;
        this.message = message;
    }
}

export default defineComponent({
    name: 'ProtoChat',
    data() {
        return {
            connection_address: undefined as string | undefined,
            connected: false,
            socket: undefined as WebSocket | undefined,
            chat_messages: [] as Message[],
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

            this.socket.addEventListener("open", _event => {
                this.connected = true;
                this.load_history();
                (this.$refs.server_address_input as HTMLInputElement).disabled = true;
                (this.$refs.connect_path_input as HTMLInputElement).disabled = true;
            });

            this.socket.addEventListener("close", _event => {
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
                        this.chat_messages.push(new Message(chat_message.userId as number, chat_message.userMessage as string));
                    }
                } else
                if (message.typeUrl === 'proto_chat.message') {
                    let chat_message = ProtoChatMessage.decode(new Uint8Array(message.value));
                    this.chat_messages.push(new Message(chat_message.userId as number,chat_message.userMessage as string));
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
        },
        send() {
            if (this.socket) {
                if (this.$refs.message_input) {
                    let message = ProtoChatMessage.create();
                    message.userId = 0;
                    message.userMessage = (this.$refs.message_input as HTMLTextAreaElement).value;

                    let bytes = ProtoChatMessage.encode(message).finish();

                    let proto_msg = Any.create();
                    proto_msg.typeUrl = "proto_chat.message";
                    proto_msg.value = bytes;
                    let proto_bytes = Any.encode(proto_msg).finish();

                    this.socket.send(proto_bytes);
                    (this.$refs.message_input as HTMLTextAreaElement).value = "";
                }
            }
        }
    }
})
</script>

<style lang="scss" scoped>
@import "bootstrap/scss/bootstrap"
</style>
