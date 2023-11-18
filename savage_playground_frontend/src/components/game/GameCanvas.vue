<template>
    <canvas id="gameCanvas" ref="gameCanvas"></canvas>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'GameCanvas',
  data () {
    return {}
  },
  methods: {
    getHtmlCanvas (): HTMLCanvasElement {
      return this.$refs.gameCanvas as HTMLCanvasElement
    }
  }
})
</script>

<style scoped>
#gameCanvas {
    width: 100%;
    aspect-ratio: 1 / 1;
}
</style>
