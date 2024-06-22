<template>
    <div class="home">
      <img alt="Vue logo" src="../assets/logo.png">
      <DebugChat :chat_room_id="12"/>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent } from 'vue'
  import DebugChat from '../components/DebugChat.vue' // @ is an alias to /src
  
  export default defineComponent({
    name: 'HomeView',
    components: {
        DebugChat
    }
  })
  </script>
  