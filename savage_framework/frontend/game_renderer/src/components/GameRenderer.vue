<template>
    <canvas id="gameRendererCanvas" ref="gameRendererCanvas"></canvas>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { SceneUpdate } from "../.gen/proto/scene_update";
import { Renderer } from "../graphics/Renderer";
import { IAssetStorage } from "../asset_storage/IAssetStorage";

export default defineComponent({
    name: 'GameRenderer',
    props: {
        assetStorage: {
            type: Object as PropType<IAssetStorage>,
            required: true,
        }
    },
    mounted() {
        const canvasElement = this.$refs.gameRendererCanvas as HTMLCanvasElement;
        const assetStorage = this.assetStorage;
        this.renderer = new Renderer(canvasElement, assetStorage);
    },
    data() {
        return {
            renderer: undefined as undefined | Renderer
        }
    },
    methods: {
        resolution(width: number, height: number) {
            const canvas = (this.$refs.gameRendererCanvas as HTMLCanvasElement);
            canvas.width = width;
            canvas.height = height;
            this.renderer?.setResolution(width, height);
        },
        async update(scene_update: SceneUpdate) {
            await this.renderer?.update(scene_update);
        },
        async render() {
            await this.renderer?.render();
        }
    }
})

</script>