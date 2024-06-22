<template>
    <div class="home">
      <DebugChat ref="debug_chat"/>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent } from 'vue'
  import DebugChat from '../components/DebugChat.vue' // @ is an alias to /src
  
  export default defineComponent({
    name: 'HomeView',
    components: {
        DebugChat
    },
    mounted() {
      (this.$refs.debug_chat as InstanceType<typeof DebugChat>).connect();
    }
  })
  </script>
  