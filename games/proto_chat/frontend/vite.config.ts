import { defineConfig } from 'vite'
import { resolve } from 'path'
import vue from '@vitejs/plugin-vue'
import { libInjectCss } from "vite-plugin-lib-inject-css"

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), libInjectCss()],
  build: {
    lib: {
      entry: resolve(__dirname, "src/index.ts"),
      name: "ProtoChatFrontend",
      fileName: "proto-chat-frontend"
    },
    rollupOptions: {
      external: ["vue"],
      output: {
        globals: {
          vue: "Vue"
        }
      }
    }
  }
})
