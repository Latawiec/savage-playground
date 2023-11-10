<template>
    <div class="main">
        <h1>Pong!</h1>
        <GameComponent></GameComponent>
    </div>
</template>

<script lang="ts">

import { defineComponent } from 'vue'
import GameComponent from '@/components/Game/GameComponent.vue'

export default defineComponent({
  name: 'PongView',
  components: {
    GameComponent
  }
})
</script>
