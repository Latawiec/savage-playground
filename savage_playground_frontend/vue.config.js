const { defineConfig } = require('@vue/cli-service')
const path = require('path')
const webpack = require('webpack')

module.exports = defineConfig({
  transpileDependencies: true,
  configureWebpack: {
    plugins: [
        // fix "process is not defined" error:
        // (do "npm install process" before running the build)
        // I did this only because I use memfs instead of memfs-browser. All fallbacks below are also because of that. I might just use memfs-browser then?
        new webpack.ProvidePlugin({
          process: 'process/browser',
        }),
    ],
    resolve: {
      alias: {
        pngjs: path.resolve(__dirname, 'node_modules/pngjs/browser.js')
      },
      fallback: {
        assert: require.resolve('assert'),
        buffer: require.resolve('buffer'),
        util: require.resolve('util'),
        path: require.resolve('path-browserify'),
        stream: require.resolve('stream-browserify'),
        process: require.resolve('process/browser'),
        url: require.resolve('url'),
      }
    },
    module: {
      rules: [
        {
          test: /\.glsl$/,
          type: 'asset/resource',
          generator: {
            outputPath: process.env.VUE_APP_LOCAL_ASSETS_PATH,
          }
        },
        {
          test: /\.json$/,
          type: 'asset/resource',
          generator: {
            outputPath: process.env.VUE_APP_LOCAL_ASSETS_PATH,
          }
        }
      ]
    }
  }
})
