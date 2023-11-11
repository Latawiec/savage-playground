<template>
    <canvas id="gameCanvas" ref="gameCanvas"></canvas>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

// Test if it builds so far
import { CommitedResourceStorage } from './backend/renderer/gl_resource/CommitedResourceStorage'
import { DrawCommand } from './backend/renderer/pipeline/DrawCommand'
import { AssetStorage } from './backend/renderer/AssetStorage'

export default defineComponent({
  name: 'GameCanvas',
  data () {
    return {
      private: {
        // runtime: {} as GameRuntime
      }
    }
  },
  props: {
    gameHostAddress: {
      type: String
      // required: true
    }
  },
  mounted () {
    // const assetPackagePath = this.$props.assetsPackagePath;
    // const gameHostAddress = this.$props.gameHostAddress;

    // this.$data._private.runtime = new GameRuntime(this.$refs.gameCanvas as HTMLCanvasElement, assetPackagePath, gameHostAddress);
    // this.$data._private.runtime.initialize();
  }
})

</script>

<style scoped>
#gameCanvas {
    width: 100%;
    aspect-ratio: 1 / 1;
    background-color: black;
}
</style>
