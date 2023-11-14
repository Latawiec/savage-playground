<template>
    <canvas id="gameCanvas" ref="gameCanvas"></canvas>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

// Test if it builds so far
import { CommitedResourceStorage } from './backend/renderer/gl_resource/CommitedResourceStorage'
import { DrawCommand } from './backend/renderer/pipeline/DrawCommand'
import { AssetStorage } from './backend/AssetStorage'
import { LocalAssets } from './backend/renderer/base_assets/LocalAssets'
import { ConnectionController } from './backend/ConnectionController'
import { GameRenderer } from "./backend/renderer/GameRenderer";

export default defineComponent({
  name: 'GameCanvas',
  data () {
    return {
      _game_renderer: undefined as GameRenderer | undefined,
    }
  },
  props: {
    gameHostAddress: {
      type: String
      // required: true
    }
  },
  mounted () {
    this._game_renderer = new GameRenderer(this.$refs.gameCanvas as HTMLCanvasElement);
    // const assetPackagePath = this.$props.assetsPackagePath;
    // const gameHostAddress = this.$props.gameHostAddress;

    // this.$data._private.runtime = new GameRuntime(this.$refs.gameCanvas as HTMLCanvasElement, assetPackagePath, gameHostAddress);
    // this.$data._private.runtime.initialize();
    const gl = (this.$refs.gameCanvas as HTMLCanvasElement).getContext("webgl")!;

    const gl_res_storage = new CommitedResourceStorage(gl, AssetStorage.empty());


    const conn_contr = new ConnectionController(new URL("ws://localhost:8080/create"));
  },
  methods: {
    
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
./backend/AssetStorage