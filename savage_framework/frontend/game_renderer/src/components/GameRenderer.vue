<template>
    <canvas id="gameRendererCanvas" ref="gameRendererCanvas"></canvas>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { SceneUpdate, UpdateType } from "../.gen/proto/scene_update";
import { DrawBundle } from "../.gen/proto/draw_bundle";
import { IAssetStorage } from "../asset_storage/IAssetStorage";
import { IDrawCommand } from "../graphics/command/IDrawCommand";

export default defineComponent({
    name: 'GameRenderer',
    props: {
        asset_storage: {
            type: Object as PropType<IAssetStorage>,
            required: true,
        }
    },
    data() {
        return {
            scene: new Map<string, DrawBundle>(),
            commands: [] as IDrawCommand[],
        }
    },
    methods: {
        update(scene_update: SceneUpdate) {

        },
        render() {
            
        }
    }
})

</script>