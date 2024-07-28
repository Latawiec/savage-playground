<script lang="ts">
import { defineComponent } from 'vue';
import { FfxivGameInput, InputHandler } from 'ffxiv-toolkit-frontend';
import { Any } from 'game_interface/proto/google/protobuf/any';

export default defineComponent({
    name: 'FfxivToolkitDebug',
    data() {
        return {
            connection_address: undefined as string | undefined,
            connected: false,
            socket: undefined as WebSocket | undefined
        }
    },
    components: {
        InputHandler
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
                (this.$refs.server_address_input as HTMLInputElement).disabled = true;
                (this.$refs.connect_path_input as HTMLInputElement).disabled = true;
            });

            this.socket.addEventListener("close", _event => {
                this.connected = false;
                this.socket = undefined;
                (this.$refs.server_address_input as HTMLInputElement).disabled = false;
                (this.$refs.connect_path_input as HTMLInputElement).disabled = false;
            });

            this.socket.addEventListener("message", _event => {
                // ignore for now
            })
        },
        disconnect() {
            if (this.socket) {
                this.socket.close();
            }
        },
        send(input_set: number) {
            if (this.socket) {
                let message = FfxivGameInput.create();
                message.inputActionsSet = input_set;
                let bytes = FfxivGameInput.encode(message).finish();

                let proto_msg = Any.create();
                proto_msg.typeUrl = "savage_playgrounds/ffxiv_game_input";
                proto_msg.value = bytes;
                let proto_bytes = Any.encode(proto_msg).finish();

                this.socket.send(proto_bytes);
            }
        }
    }
})
</script>

<template>
    <h1>Ffxiv Toolkit Debug</h1>
    <InputHandler ref="input_handler" @input_changed="send"></InputHandler>
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
</template>