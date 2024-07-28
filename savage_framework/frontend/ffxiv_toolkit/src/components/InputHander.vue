<template>
    <!-- Nothing -->
</template>

<script lang="ts">
import { defineComponent } from 'vue';

const DEFAULT_INPUT_MAP: Map<string, number> = new Map([
    ['KeyW', 1 << 0],
    ['KeyS', 1 << 1],
    ['KeyA', 1 << 2],
    ['KeyD', 1 << 3],
    ['Space', 1 << 4],
]);

export default defineComponent({
    name: 'InputHandler',
    emits: {
        input_changed: (input_set) => { return true }
    },
    props: {
        inputMap: {
            type: Map<string, number>,
            required: false,
            default: DEFAULT_INPUT_MAP
        }
    },
    data() {
        return {
            input_set: 0
        }
    },
    methods: {
        getInputSet() {
            return this.input_set;
        }
    },
    onMounted() {
        addEventListener('keydown', (event) => {
            for (let [key, input_value] of this.$props.inputMap) {
                if (event.code === key) {
                    this.input_set &= input_value;
                    this.$emit('input_changed', this._input_set);
                    return;
                }
            }
        })
    }
})

</script>