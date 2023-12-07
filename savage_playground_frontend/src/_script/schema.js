const fs = require('fs')
const path = require('path')
const { compileFromFile } = require('json-schema-to-typescript')

const outputPath = '/src/components/game/backend/.generated'

async function buildSchemasFromDir(buildSourceDir, buildOutputDir) {

    if (!fs.existsSync(buildSourceDir)) {
        throw new Error(`Schema source dir [${buildOutputDir}] doesn't exist.`)
    }

    if (!fs.existsSync(buildOutputDir)) {
        fs.mkdirSync(buildOutputDir, { recursive: true })
    }

    const dir_entries = await fs.promises.readdir( buildSourceDir );

    for ( const file of dir_entries ) {
        const filepath = path.join(buildSourceDir, file);
        const stat = await fs.promises.stat(filepath);

        if (!stat.isFile()) {
            continue;
        }
        try {
            await compileFromFile(filepath).then(ts => fs.writeFileSync(path.join(buildOutputDir, path.parse(file).name + ".ts"), ts));
        } catch(e) {
            console.log(`Failed to compile ${file}: ${e}`);
        }
    }
}

async function buildRoomServerInterface() {
    const schemasDir = path.join(process.env.PWD, process.env.SCHEMA_ROOM_SERVER_INTERFACE_DIR);
    const outputDir = path.join(process.env.PWD, outputPath, '/room_server_interface');

    buildSchemasFromDir(schemasDir, outputDir);
}

buildRoomServerInterface();