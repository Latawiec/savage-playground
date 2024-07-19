import { defineConfig } from 'vite'
import { resolve } from 'path'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  assetsInclude: [
    "**/*.glsl",
    "**/*.ps.glsl",
    "**/*.vs.glsl",
    "./src/graphics/assets/**/*.json",
  ],
  build: {
    lib: {
      entry: resolve(__dirname, "src/index.ts"),
      name: "GameRendererFrontend",
      fileName: "game-renderer-frontend"
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
