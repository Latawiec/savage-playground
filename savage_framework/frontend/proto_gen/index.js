const fs = require('fs')
const path = require('path')
const { exec } = require('child_process')

const pluginPath = path.resolve(__dirname, './node_modules/.bin/protoc-gen-ts_proto')

exports = module.exports = build_protos_from_dir;

async function build_protos_from_dir(buildSourceDir, buildOutputDir) {
    if (!fs.existsSync(buildSourceDir)) {
        throw new Error(`Proto source dir [${buildSourceDir}] doesn't exist.`)
    }

    if (!fs.existsSync(buildOutputDir)) {
        fs.mkdirSync(buildOutputDir, { recursive: true })
    }

    const buildCmd =
        `protoc --plugin="${pluginPath}" --ts_proto_opt=esModuleInterop=true --proto_path="${buildSourceDir}" --ts_proto_out="${buildOutputDir}" "${buildSourceDir}"/*.proto`

    await exec(buildCmd, (_err, stdout, stderr) => {
        if (stderr) {
            console.log(`Errors:\n${stderr}\n`)
        }
        if (stdout) {
            console.log(`Output:\n${stdout}\n`)
        }
    })
}