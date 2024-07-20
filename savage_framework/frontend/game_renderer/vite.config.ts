import { defineConfig } from 'vite'
import { resolve } from 'path'
import vue from '@vitejs/plugin-vue'
import { nodePolyfills } from 'vite-plugin-node-polyfills'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(),
    nodePolyfills({
      overrides: {
        fs: 'memfs', // Since `fs` is not supported in browsers, we can use the `memfs` package to polyfill it.
      },
    })],
  assetsInclude: [
    "**/*.glsl",
    "**/*.ps.glsl",
    "**/*.vs.glsl",
    "**/*.zip",
    "./src/graphics/assets/**/*.json",
  ],
  resolve: {
    alias: {
      'unzipper': resolve(__dirname, 'node_modules/unzipper'),
      'memjs': resolve(__dirname, 'node_modules/memjs'),
    }
  },
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
