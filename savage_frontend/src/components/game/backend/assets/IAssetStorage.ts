
export interface IAssetStorage {
    get source(): string;
    readFile(assetPath: string): Promise<string | Buffer>;
}
