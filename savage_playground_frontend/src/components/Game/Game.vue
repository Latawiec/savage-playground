<template>
    <GameOverlay/>
    <GameCanvas/>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import GameCanvas from './GameCanvas.vue'
import GameOverlay from './GameOverlay.vue'


export default defineComponent({
    name: "Game",
    components: {
        GameCanvas,
        GameOverlay,
    }
})
</script>