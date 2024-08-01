<template>

</template>


<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { IAssetStorage } from 'game-renderer-frontend';

export default defineComponent({
    name: 'CharacterSelection',
    props: {
        assetStorage: {
            type: Object as PropType<IAssetStorage>,
            required: true,
        }
    },
    data() {
        return {
            
        }
    }
})

</script>