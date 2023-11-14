import JSZip from 'jszip'
import { IFs, memfs } from 'memfs'

// Constructor is private. Create this structure from either `from_remote` or `from_local` static functions.
export class AssetStorage {
    private _fs: IFs;
    private _path: string;

    private constructor (path: string, fs: IFs) {
      this._fs = fs
      this._path = path
    }

    get fs () {
      return this._fs
    }

    get path () {
      return this._path
    }

    static empty(): AssetStorage {
      return new AssetStorage("empty", memfs().fs);
    }

    static async form(path: string): Promise<AssetStorage> {
      return new Promise<AssetStorage>((resolve, reject) => {
        const mem_fs = memfs().fs

        fetch(path)
          .then((response) => {
            if (response.status === 200 || response.status === 0) {
              return response.blob()
            } else {
              console.log(response.statusText)
              return Promise.reject(new Error(response.statusText))
            }
          })
          .then(AssetStorage.create_memfs_from_file)
          .then(
            async function success (mem_fs) {
              resolve(new AssetStorage(path, mem_fs))
            },
            function error (e) {
              reject(e)
            }
          )
      })
    }

    private static async create_memfs_from_file (file_blob: Blob): Promise<IFs> {
      const mem_fs = memfs().fs

      return new Promise((resolve, reject) => {
        JSZip.loadAsync(file_blob)
          .then((zip) => {
            const promises: Promise<void>[] = []
            zip.forEach((relativePath, zipEntry) => {
              if (zipEntry.dir) {
                mem_fs.mkdirSync('/' + relativePath)
              } else {
                promises.push(
                  zipEntry.async('uint8array')
                    .then((blob) => {
                      mem_fs.writeFileSync('/' + relativePath, Buffer.from(blob.buffer))
                    })
                )
              }
            })
            return promises
          })
          .then(async (promises) => {
            await Promise.all(promises)
            resolve(mem_fs)
          }
          )
      })
    }
}
