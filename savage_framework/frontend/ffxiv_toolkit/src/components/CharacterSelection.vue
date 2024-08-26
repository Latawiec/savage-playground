<template>
    <h1>Character Selection</h1>

    <h2>Pick character:</h2>
    <div v-for="character in data?.charactersData" :key="character.id">
        <h3>{{ character.name }}</h3>
        <h3>Job: {{ character.job?.nameShort }}</h3>
        <p>{{ character.description }}</p>
    </div>

    <h2>Pick role:</h2>
    <div v-for="role in data?.raidRolesData" :key="role.id">
        <p v-if="data?.takenRaidRoleIds.includes(role.id)">Occupied!</p>
        <h3>{{ role.name }}</h3>
        <p>{{ role.class }}</p>
    </div>

    <p ref="user_message" v-if="data?.userMessage">{{ data.userMessage }}</p>
    <p ref="user_warning" v-if="data?.userWarning">Warning: {{ data.userWarning }}</p>
    <p ref="user_error" v-if="data?.userError">Error: {{ data.userError }}</p>

    <button>Lock in!</button>
</template>


<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { IAssetStorage } from 'game-renderer-frontend';
import { FfxivCharacterSelection, FfxivCharacterSelectionInput, FfxivPlayerCharacter, FfxivPlayerRaidRole } from "../.gen/proto/components/character_selection"

export default defineComponent({
    name: 'CharacterSelection',
    emits: {
        selected: (data: FfxivCharacterSelectionInput) => { return true }
    },
    props: {
        assetStorage: {
            type: Object as PropType<IAssetStorage>,
            required: true,
        }
    },
    data() {
        return {
            data: undefined as FfxivCharacterSelection | undefined
        }
    },

    methods: {
        updateCharacterSelection(data: FfxivCharacterSelection) {
            this.data = data;
        }
    }
})

</script>