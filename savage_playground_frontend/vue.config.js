const { defineConfig } = require('@vue/cli-service')
const path = require('path')

module.exports = defineConfig({
  transpileDependencies: true,
  configureWebpack: {
    resolve: {
      alias: {
        pngjs: path.resolve(__dirname, 'node_modules/pngjs/browser.js')
      }
    }
  }
})
