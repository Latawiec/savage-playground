<template>
    <GameOverlay ref="game_overlay"/>
    <GameCanvas ref="game_canvas"/>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import GameCanvas from './GameCanvas.vue'
import GameOverlay from './GameOverlay.vue'

export default defineComponent({
  name: 'GameComponent',
  components: {
    GameCanvas,
    GameOverlay
  },
  methods: {
    test() {
      const auto = this.$refs.game_canvas;
    }
  }
})

</script>
