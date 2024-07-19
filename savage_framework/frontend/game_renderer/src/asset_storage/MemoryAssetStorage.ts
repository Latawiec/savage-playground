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

    async append_from_zip_remote(file_path: string): Promise<void> {
        const response = await fetch(file_path, {
            method: 'GET',
            mode: 'cors',
            credentials: 'omit',
            redirect: 'follow',
            headers: {
                "Content-Type": "application/zip"
            }
        });

        if (response.status !== 200 && response.status !== 0) {
            throw new Error(`Failed to append MemoryAssetStorage from ${file_path}: ${response.statusText}`);
        }

        const file_buffer = await response.arrayBuffer();
        await MemoryAssetStorage.unpack_to_mem_fs(this._fs, file_buffer);
    }

    async append_from_zip_local(file_path: string): Promise<void> {
        const fs_read_file = promisify(fs.readFile);
        const file_buffer = await fs_read_file(file_path);
        await MemoryAssetStorage.unpack_to_mem_fs(this._fs, file_buffer);
    }

    private static async unpack_to_mem_fs(mem_fs: IFs, file_data: ArrayBuffer): Promise<void> {
        const mem_fs_write_file = promisify(mem_fs.writeFile);
        const mem_fs_mkdir = promisify<PathLike, { recursive: true }>(mem_fs.mkdir);
        const file_zip = await unzipper_read.buffer(Buffer.from(file_data));

        for (const entry of file_zip.files) {
            const filepath = '/' + entry.path;
            const dirname = path.dirname(filepath);
            const type = entry.type;

            if (type === "File") {
                const file_buffer = await entry.buffer();
                await mem_fs_mkdir(dirname, { recursive: true });
                await mem_fs_write_file(filepath, file_buffer);
            }
        }
    }

    // implements IAssetStorage
    readFile(assetPath: string): Promise<string | Buffer> {
        return this._fs.promises.readFile('/' + assetPath);
    }
}