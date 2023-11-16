import { ZipAssetStorage } from '@/components/game/backend/assets/ZipAssetStorage';
import fs from 'fs'
import memfs from 'memfs'
import path from 'path'

global.fetch = jest.fn();

const ASSETS_ROOT = '/assets/AssetStorage'

function validate_storage_contents (storage: ZipAssetStorage, sys_fs_dir: fs.Dir, sys_dir_rel_path: string): Promise<boolean> {
  return new Promise<boolean>(async (resolve, reject) => {
    for await (const sys_dir_entry of sys_fs_dir) {
      const relative_path = '/' + path.relative(sys_dir_rel_path, sys_dir_entry.path) + '/' + sys_dir_entry.name;

      if (sys_dir_entry.isFile()) {
        const storage_file_content = await storage.read_file(relative_path) as Buffer;
        const sys_fs_file_content = fs.readFileSync(sys_dir_entry.path + '/' + sys_dir_entry.name, null);

        if (Buffer.compare(storage_file_content, sys_fs_file_content) != 0) {
          reject(`Missmatch: file contents differ in ${relative_path}`)
        }
      }

      if (sys_dir_entry.isDirectory()) {
        const dir_match = await validate_storage_contents(storage, await fs.promises.opendir(sys_dir_entry.path + '/' + sys_dir_entry.name), sys_dir_rel_path)
        if (!dir_match) {
          reject(`Missmatch: directory contents validation failed in ${relative_path}`)
        }
      }
    }
    resolve(true)
  })
}

describe('from', () => {
  it.each([
    [0, ASSETS_ROOT + '/zip_folder_0.zip'],
    [2, ASSETS_ROOT + '/zip_folder_2.zip'],
    [5, ASSETS_ROOT + '/zip_folder_5.zip'],
    [9, ASSETS_ROOT + '/zip_folder_9.zip']
  ])('Unzup file - compression level %p', async (level: number, zip_file_path: string) => {
    // Given
    const zipped_file_path = __dirname + zip_file_path
    const validate_dir_path = __dirname + ASSETS_ROOT + '/zip_folder'

    const file_buffer = fs.readFileSync(zipped_file_path)
    const file_blob = new Blob([file_buffer]);

    // Assume fetch returns valid file.
    (fetch as jest.MockedFunction<typeof fetch>).mockResolvedValue({
      status: 200,
      blob: () => {
        return file_blob
      }
    } as any as Response)

    // When
    const asset_storage = await ZipAssetStorage.from_file(zipped_file_path)

    // Then
    const check_dir = await fs.promises.opendir(validate_dir_path)
    expect(await validate_storage_contents(asset_storage, check_dir, validate_dir_path)).toBe(true)
  })
})
