const fs = require('fs')
const path = require('path')
const { exec } = require('child_process')

const pluginPath = './node_modules/.bin/protoc-gen-ts_proto'
const outputPath = '/src/components/game/backend/.generated'

async function buildProtosFromDir(buildSourceDir, buildOutputDir) {

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

async function buildHostRuntimeInterface() {
  const protosDir = path.join(process.env.PWD, process.env.PROTO_HOST_RUNTIME_INTERFACE_DIR);
  const outputDir = path.join(process.env.PWD, outputPath, '/host_runtime_interface');

  if (!fs.existsSync(protosDir)) {
    throw new Error(`Proto source dir [${protosDir}] doesn't exist.`)
  }

  buildProtosFromDir(protosDir, outputDir);
}

async function buildGameInterface() {
  const protosDir = path.join(process.env.PWD, process.env.PROTO_GAME_INTERFACE_DIR);
  const outputDir = path.join(process.env.PWD, outputPath, '/game_interface');

  if (!fs.existsSync(protosDir)) {
    throw new Error(`Proto source dir [${protosDir}] doesn't exist.`)
  }

  buildProtosFromDir(path.join(protosDir, '/game_output/renderer'), path.join(outputDir, '/game_output/renderer'));
  buildProtosFromDir(path.join(protosDir, '/game_output/settings'), path.join(outputDir, '/game_output/settings'));
  buildProtosFromDir(path.join(protosDir, '/game_output/ui'), path.join(outputDir, '/game_output/ui'));
}




if (!fs.existsSync(pluginPath)) {
  throw new Error(`TS Proto Generator plugin not found [${pluginPath}]`)
}

buildHostRuntimeInterface();
buildGameInterface();