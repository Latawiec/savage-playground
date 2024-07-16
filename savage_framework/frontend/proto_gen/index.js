import * as fs from 'fs';
import * as path from 'path';
import { exec } from 'child_process'; 

const pluginPath = path.resolve(import.meta.dirname, './node_modules/.bin/protoc-gen-ts_proto')

async function build_protos_from_dir(buildSourceDir, buildOutputDir) {
    if (!fs.existsSync(buildSourceDir)) {
        throw new Error(`Proto source dir [${buildSourceDir}] doesn't exist.`)
    }

    if (!fs.existsSync(buildOutputDir)) {
        fs.mkdirSync(buildOutputDir, { recursive: true })
    }

    let plugin_arg = `--plugin="${pluginPath}`;
    if (process.platform === "win32") {
        plugin_arg = `--plugin=protoc-gen-ts_proto="${pluginPath}.cmd"`;
    }

    const buildCmd =
        `protoc ${plugin_arg} --ts_proto_opt=esModuleInterop=true --proto_path="${buildSourceDir}" --ts_proto_out="${buildOutputDir}" "${buildSourceDir}/*.proto"`

    await exec(buildCmd, (_err, stdout, stderr) => {
        if (stderr) {
            console.log(`Errors:\n${stderr}\n`)
        }
        if (stdout) {
            console.log(`Output:\n${stdout}\n`)
        }
    })
}

export { build_protos_from_dir }