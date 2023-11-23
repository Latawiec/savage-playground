const fs = require('fs')
const path = require('path')
const { exec } = require('child_process')

const protosDir = path.join(process.env.PWD, process.env.PROTO_SOURCE_DIR)
const outputDir = path.join(process.env.PWD, '/src/_proto')
const pluginPath = './node_modules/.bin/protoc-gen-ts_proto'

function verifyPaths () {
  if (!fs.existsSync(protosDir)) {
    throw new Error(`PROTO_SOURCE_DIR [${process.env.PROTO_SOURCE_DIR}] does not exist.`)
  }

  if (!fs.existsSync(pluginPath)) {
    throw new Error(`TS Proto Generator plugin not found [${pluginPath}]`)
  }
}

async function buildProtosFromDir (sourceRelDir, outputRelDir) {
  const buildSourceDir = path.join(protosDir, sourceRelDir)
  const buildOutputDir = path.join(outputDir, outputRelDir)

  if (!fs.existsSync(buildSourceDir)) {
    throw new Error(`Proto source dir [${buildOutputDir}] doesn't exist.`)
  }

  if (!fs.existsSync(buildOutputDir)) {
    fs.mkdirSync(buildOutputDir, { recursive: true })
  }

  const buildCmd =
    `protoc --plugin="${pluginPath}" --proto_path="${buildSourceDir}" --ts_proto_out="${buildOutputDir}" "${buildSourceDir}"/*.proto`

  // console.log(`Running:\n${build_cmd}\n`);

  await exec(buildCmd, (_err, stdout, stderr) => {
    if (stderr) {
      console.log(`Errors:\n${stderr}\n`)
    }
    if (stdout) {
      console.log(`Output:\n${stdout}\n`)
    }
  })
}

verifyPaths()
buildProtosFromDir('/game_message/settings', '/settings')
buildProtosFromDir('/game_message/renderer', '/renderer')
buildProtosFromDir('/game_message/ui', '/ui')
