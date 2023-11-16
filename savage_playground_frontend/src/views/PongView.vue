<template>
    <div class="main">
        <h1>Pong!</h1>
        <GameComponent game_host_address="ws://localhost:3000/create"></GameComponent>
    </div>
</template>

<script lang="ts">

import { defineComponent } from 'vue'
import GameComponent from '@/components/game/GameComponent.vue'

export default defineComponent({
  name: 'PongView',
  components: {
    GameComponent
  }
})
</script>
