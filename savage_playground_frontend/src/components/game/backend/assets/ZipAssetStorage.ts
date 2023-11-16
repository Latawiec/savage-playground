import { IAssetStorage } from "./IAssetStorage";
import JSZip from 'jszip'
import { IFs, memfs } from 'memfs'

export class ZipAssetStorage implements IAssetStorage {
    private _fs: IFs;
    private _path: string;

    static from_empty(): ZipAssetStorage {
        return new ZipAssetStorage(memfs().fs, "empty");
    }

    static async from_file(path: string): Promise<ZipAssetStorage> {
        return new Promise<ZipAssetStorage>((resolve, reject) => {
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
                .then(ZipAssetStorage.create_memfs_from_file)
                .then(
                    async function success(mem_fs) {
                        resolve(new ZipAssetStorage(mem_fs, path))
                    },
                    function error(e) {
                        reject(e)
                    }
                )
        })
    }

    // Impl IAssetStorage
    get source(): string {
        throw new Error("Method not implemented.");
    }

    read_file(asset_path: string): Promise<string | Buffer> {
        return this._fs.promises.readFile(asset_path);
    }

    // private
    private constructor(fs: IFs, path: string) {
        this._fs = fs;
        this._path = path;
    }

    private static async create_memfs_from_file(file_blob: Blob): Promise<IFs> {
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