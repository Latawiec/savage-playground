import { IAssetStorage } from './IAssetStorage'
import JSZip from 'jszip'
import { IFs, memfs } from 'memfs'
import { Buffer } from 'buffer'
import path from 'path'

export class ZipAssetStorage implements IAssetStorage {
    private _fs: IFs;
    private _path: string;

    static fromEmpty (): ZipAssetStorage {
      return new ZipAssetStorage(memfs().fs, 'empty')
    }

    static async fromFile (path: string): Promise<ZipAssetStorage> {
      const response = await fetch(path, {
        method: 'GET',
        mode: 'cors',
        credentials: 'omit',
        redirect: 'follow'
      })

      if (response.status !== 200 && response.status !== 0) {
        throw new Error(`Failed to create ZipAssetStorage from ${path}: ${response.statusText}`)
      }

      const memfs = await ZipAssetStorage.createMemfsFromFile(await response.blob())
      return new ZipAssetStorage(memfs, path)
    }

    // Impl IAssetStorage
    get source (): string {
      throw new Error('Method not implemented.')
    }

    readFile (assetPath: string): Promise<string | Buffer> {
      return this._fs.promises.readFile(assetPath)
    }

    // private
    private constructor (fs: IFs, path: string) {
      this._fs = fs
      this._path = path
    }

    private static async createMemfsFromFile (fileBlob: Blob): Promise<IFs> {
      const memFs = memfs().fs

      const zip = await JSZip.loadAsync(fileBlob)
      const promises: Promise<void>[] = []
      zip.forEach((relativePath, zipEntry) => {
        if (zipEntry.dir) {
          memFs.mkdirSync('/' + relativePath)
        } else {
          const promise = zipEntry.async('uint8array').then((data) => {
            memFs.writeFileSync(path.join('/', relativePath), Buffer.from(data.buffer))
          })
          promises.push(promise)
        }
      })

      await Promise.all(promises)
      return memFs
    }
}
