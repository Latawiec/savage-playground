<template>
    <div v-for="message in messages">
        <p>{{ message }}</p>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'DebugChatOutput',
    data() {
        return {
            messages: [] as string[]
        }
    },
    methods: {
        write(message: string) {
            this.messages.push(message)
        },
        write_many(messages: string[]) {
            this.messages.push(...messages)
        },
        clear() {
            this.messages = []
        }
    }
})
</script>