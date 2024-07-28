<template>
    <div>huh</div>
    <a>HELLOOO</a>
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
        input_changed: (_input_set: number) => { return true }
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
        },
        emit_input_changed() {
            this.$emit('input_changed', this.input_set);
        }
    },
    mounted() {
        console.log("mounted");
        addEventListener('keydown', (event) => {
            if (event.repeat) return;
            for (let [key, input_value] of this.$props.inputMap) {
                if (event.code === key) {
                    this.input_set |= input_value;
                    this.emit_input_changed();
                    return;
                }
            }
        })
        addEventListener('keyup', (event) => {
            for (let [key, input_value] of this.$props.inputMap) {
                if (event.code === key) {
                    this.input_set &= ~input_value;
                    this.emit_input_changed();
                    return;
                }
            }
        })
    }
})

</script>