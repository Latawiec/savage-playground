<template>
    <div>
        <textarea v-model="message" placeholder="Write your message!"></textarea>
        <button @click="send">Send!</button>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'DebugChatInput',
    data() {
        return {
            message: "" as string,
        }
    },
    emits: ['message-sent'],
    methods: {
        send() {
            if (this.message) {
                this.$emit('message-sent', this.message)
            }
        }
    }
})
</script>