<template>
    <GameOverlay ref="overlay"/>
    <GameCanvas ref="canvas" />
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import GameCanvas from './GameCanvas.vue'
import GameOverlay from './GameOverlay.vue'
import { GameCanvasInterface, GameOverlayInterface, GameRuntime } from './backend/GameRuntime';

const overlay_ref = ref<InstanceType<typeof GameOverlay>>();
const canvas_ref = ref<InstanceType<typeof GameCanvas>>();


export default defineComponent({
  name: 'GameComponent',
  props: {
    game_host_address: { type: String, required: true },
  },
  data() {
    return {
      _game_runtime: undefined as GameRuntime | undefined,
    }
  },
  components: {
    GameCanvas,
    GameOverlay
  },
  mounted() {
    const address_url = new URL(this.$props.game_host_address);
    const canvas_component = canvas_ref.value! as GameCanvasInterface;
    const overlay_component = overlay_ref.value! as GameOverlayInterface;
    
    this._game_runtime = new GameRuntime(address_url, canvas_component, overlay_component);
  }
})

</script>
