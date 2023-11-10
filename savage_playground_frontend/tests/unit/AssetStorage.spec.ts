import { AssetStorage } from "@/components/game/backend/renderer/AssetStorage";
import fs from 'fs';
import memfs from 'memfs';
import path from 'path';
import fetch, { Response } from 'node-fetch';

jest.mock('node-fetch', ()=>jest.fn())

function validate_memfs_contents(mem_fs: memfs.IFs, sys_fs_dir: fs.Dir, sys_dir_rel_path: string): Promise<boolean> {

    return new Promise<boolean>(async (resolve, reject) => {

        for await (const sys_dir_entry of sys_fs_dir) {
            const relative_path = "/" + path.relative(sys_dir_rel_path, sys_dir_entry.path) + "/" + sys_dir_entry.name;

            if (!mem_fs.existsSync(relative_path)) {
                reject(`Path ${relative_path} does not exist in in-memory fs`);
            }

            if (sys_dir_entry.isFile()) {
                if (!mem_fs.statSync(relative_path).isFile()) {
                    reject(`Missmatch: expected file at ${relative_path}`);
                }

                let mem_fs_file_content = mem_fs.readFileSync(relative_path, undefined) as Buffer;
                let sys_fs_file_content = fs.readFileSync(sys_dir_entry.path + "/" + sys_dir_entry.name, null);

                if (0 != Buffer.compare(mem_fs_file_content, sys_fs_file_content)) {
                    reject(`Missmatch: file contents differ in ${relative_path}`);
                }
            }

            if (sys_dir_entry.isDirectory()) {
                if (!mem_fs.statSync(relative_path).isDirectory()) {
                    reject(`Missmatch: expected directory at ${relative_path}`);
                }
                let dir_match = await validate_memfs_contents(mem_fs, await fs.promises.opendir(sys_dir_entry.path + "/" + sys_dir_entry.name), sys_dir_rel_path);
                if (!dir_match) {
                    reject(`Missmatch: directory contents validation failed in ${relative_path}`);
                }
            }
        }
        resolve(true);
    })
}

describe('from local', () => {
    it.each([
        [0, "/assets/zip_folder_0.zip"],
        [2, "/assets/zip_folder_2.zip"],
        [5, "/assets/zip_folder_5.zip"],
        [9, "/assets/zip_folder_9.zip"],
    ])('Unzup file - compression level %p', async (level: number, zip_file_path: string) => {
        // Given
        const zipped_file_path = __dirname + zip_file_path;
        const validate_dir_path = __dirname + "/assets/zip_folder";

        // When
        const asset_storage = await AssetStorage.from_local(zipped_file_path);

        // Then
        const check_dir = await fs.promises.opendir(validate_dir_path);
        expect(await validate_memfs_contents(asset_storage.fs, check_dir, validate_dir_path)).toBe(true);
    });
})

describe('from remote', () => {
    it.each([
        [0, "/assets/zip_folder_0.zip"],
        [2, "/assets/zip_folder_2.zip"],
        [5, "/assets/zip_folder_5.zip"],
        [9, "/assets/zip_folder_9.zip"],
    ])('Unzup file - compression level %p', async (level: number, zip_file_path: string) => {
        // Given
        const zipped_file_path = __dirname + zip_file_path;
        const validate_dir_path = __dirname + "/assets/zip_folder";

        let file_buffer = fs.readFileSync(zipped_file_path);
        let file_blob = new Blob([file_buffer]);

        // Assume fetch returns valid file.
        (fetch as jest.MockedFunction<typeof fetch>).mockResolvedValue({
            status: 200,
            blob: () => {
                return file_blob;
            }
        } as any as Response);

        // When
        const asset_storage = await AssetStorage.form_remote(zipped_file_path);

        // Then
        const check_dir = await fs.promises.opendir(validate_dir_path);
        expect(await validate_memfs_contents(asset_storage.fs, check_dir, validate_dir_path)).toBe(true);
    });
})