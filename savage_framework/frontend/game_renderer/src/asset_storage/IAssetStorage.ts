
export interface IAssetStorage {
    readFile(assetPath: string): Promise<string | Buffer>
}