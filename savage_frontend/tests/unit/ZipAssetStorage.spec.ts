import { ZipAssetStorage } from '@/components/game/backend/assets/ZipAssetStorage'
import fs from 'fs'
import path from 'path'

global.fetch = jest.fn()

const ASSETS_ROOT = '/assets/AssetStorage'

async function validateStorageContents (storage: ZipAssetStorage, sysFsDir: fs.Dir, sysDirRelPath: string) {
  for await (const sysDirEntry of sysFsDir) {
    const relativePath = '/' + path.relative(sysDirRelPath, sysDirEntry.path) + '/' + sysDirEntry.name

    if (sysDirEntry.isFile()) {
      const storageFileContent = await storage.readFile(relativePath) as Buffer
      const sysFsFileContent = fs.readFileSync(sysDirEntry.path + '/' + sysDirEntry.name, null)

      if (Buffer.compare(storageFileContent, sysFsFileContent) !== 0) {
        throw new Error(`Missmatch: file contents differ in ${relativePath}`)
      }
    }

    if (sysDirEntry.isDirectory()) {
      await validateStorageContents(storage, await fs.promises.opendir(sysDirEntry.path + '/' + sysDirEntry.name), sysDirRelPath)
    }
  }
}

describe('from', () => {
  it.each([
    [0, ASSETS_ROOT + '/zip_folder_0.zip'],
    [2, ASSETS_ROOT + '/zip_folder_2.zip'],
    [5, ASSETS_ROOT + '/zip_folder_5.zip'],
    [9, ASSETS_ROOT + '/zip_folder_9.zip']
  ])('Unzup file - compression level %p', async (level: number, zipFilePath: string) => {
    // Given
    const zippedFilePath = path.join(__dirname, zipFilePath)
    const validateDirPath = path.join(__dirname, ASSETS_ROOT, '/zip_folder')

    const fileBuffer = fs.readFileSync(zippedFilePath)
    const fileBlob = new Blob([fileBuffer]);

    // Assume fetch returns valid file.
    (fetch as jest.MockedFunction<typeof fetch>).mockResolvedValue({
      status: 200,
      blob: () => {
        return fileBlob
      }
    } as unknown as Response)

    // When
    const assetStorage = await ZipAssetStorage.fromFile(zippedFilePath)

    // Then
    const checkDir = await fs.promises.opendir(validateDirPath)
    expect(async () => { await validateStorageContents(assetStorage, checkDir, validateDirPath) }).not.toThrow()
  })
})
