import { IFs, memfs } from "memfs";
import path from 'path';
import fs, { PathLike } from 'fs';
import { IAssetStorage } from "./IAssetStorage";
import { promisify } from "util";
import { Open as unzipper_read } from "unzipper";

export class MemoryAssetStorage implements IAssetStorage {
    private _fs: IFs;

    constructor() {
        this._fs = memfs().fs;
    }

    async appendFromZipRemote(filePath: string): Promise<void> {
        const response = await fetch(filePath, {
            method: 'GET',
            mode: 'cors',
            credentials: 'omit',
            redirect: 'follow',
            headers: {
                "Content-Type": "application/zip"
            }
        });

        if (response.status !== 200 && response.status !== 0) {
            throw new Error(`Failed to append MemoryAssetStorage from ${filePath}: ${response.statusText}`);
        }

        const file_buffer = await response.arrayBuffer();
        await MemoryAssetStorage.UnpackToMemFs(this._fs, file_buffer);
    }

    async appendFromZipLocal(filePath: string): Promise<void> {
        const fsReadFile = promisify(fs.readFile);
        const file_buffer = await fsReadFile(filePath);
        await MemoryAssetStorage.UnpackToMemFs(this._fs, file_buffer);
    }

    private static async UnpackToMemFs(mem_fs: IFs, file_data: ArrayBuffer): Promise<void> {
        const memFsWriteFile = promisify(mem_fs.writeFile);
        const memFsMkDir = promisify<PathLike, { recursive: true }>(mem_fs.mkdir);
        const file_zip = await unzipper_read.buffer(Buffer.from(file_data));

        for (const entry of file_zip.files) {
            const filepath = '/' + entry.path;
            const dirname = path.dirname(filepath);
            const type = entry.type;

            if (type === "File") {
                const file_buffer = await entry.buffer();
                await memFsMkDir(dirname, { recursive: true });
                await memFsWriteFile(filepath, file_buffer);
            }
        }
    }

    // implements IAssetStorage
    readFile(assetPath: string): Promise<string | Buffer> {
        return this._fs.promises.readFile('/' + assetPath);
    }
}