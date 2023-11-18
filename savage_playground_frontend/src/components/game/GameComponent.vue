<template>
    <GameOverlay ref="overlay"/>
    <GameCanvas @vue:mounted="createRenderer" ref="canvas" />
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import GameCanvas from './GameCanvas.vue'
import GameOverlay from './GameOverlay.vue'
import { GameCanvasInterface, GameOverlayInterface, GameRuntime } from './backend/GameRuntime'

export default defineComponent({
  name: 'GameComponent',
  props: {
    gameHostAddress: { type: String, required: true }
  },
  data () {
    return {
      gameRuntime: undefined as GameRuntime | undefined
    }
  },
  components: {
    GameCanvas,
    GameOverlay
  },
  methods: {
    createRenderer () {
      const addressUrl = new URL(this.$props.gameHostAddress)
      const canvasComponent = this.$refs.canvas as GameCanvasInterface
      const overlayComponent = this.$refs.overlay as GameOverlayInterface

      this.gameRuntime = new GameRuntime(addressUrl, canvasComponent, overlayComponent)
    }
  }
})

</script>
