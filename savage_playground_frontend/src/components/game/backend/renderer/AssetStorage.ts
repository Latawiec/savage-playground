import JSZip, { OutputType } from "jszip";
import { IFs, memfs } from "memfs";
import fs from "fs";
import fetch from "node-fetch";

export class AssetStorage {
    private _fs;

    get fs() {
        return this._fs;
    }

    private constructor(fs: IFs) {
        this._fs = fs;
    }

    static form_remote(path: string): Promise<AssetStorage> {
        return new Promise<AssetStorage>((resolve, reject) => {
            const mem_fs = memfs().fs;

            fetch(path)
            .then((response) => {
                if (response.status === 200 || response.status === 0) {
                    return Promise.resolve(response.blob());
                } else {
                    console.log(response.statusText);
                    return Promise.reject(new Error(response.statusText));
                }
            })
            .then(JSZip.loadAsync)
            .then((zip) => {
                let promises: Promise<void>[] = [];
                zip.forEach((relativePath, zipEntry) => {
                    if (zipEntry.dir) {
                        mem_fs.mkdirSync("/" + relativePath);
                    } else {
                        promises.push(
                            zipEntry.async("uint8array")
                            .then((blob) => {
                                mem_fs.writeFileSync("/" + relativePath, Buffer.from(blob.buffer));
                            })
                        );
                    }
                });
                return promises;
            })
            .then(
                async function success(promises) {
                    await Promise.all(promises);
                    resolve(new AssetStorage(mem_fs))
                },
                function error(e) {
                    reject(e);
                }
            );
        })
    }

    static async from_local(path: string): Promise<AssetStorage> {
        return new Promise<AssetStorage>((resolve, reject) => {
            const mem_fs = memfs().fs;

            new JSZip.external.Promise(function (resolve, reject) {
                fs.readFile(path, function(err, data) {
                    if (err) {
                        reject(err);
                    } else {
                        resolve(data);
                    }
                });
            })
            .then((data) => {
                return JSZip.loadAsync(data as Buffer);
            })
            .then((zip) => {
                let promises: Promise<void>[] = [];
                zip.forEach((relativePath, zipEntry) => {
                    if (zipEntry.dir) {
                        mem_fs.mkdirSync("/" + relativePath);
                    } else {
                        promises.push(
                            zipEntry.async("uint8array")
                            .then((blob) => {
                                mem_fs.writeFileSync("/" + relativePath, Buffer.from(blob.buffer));
                            })
                        );
                    }
                });
                return promises;
            })
            .then(
                async function success(promises) {
                    await Promise.all(promises);
                    resolve(new AssetStorage(mem_fs))
                },
                function error(e) {
                    reject(e);
                }
            );
        });
    }
}