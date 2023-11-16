
export interface IAssetStorage {
    get source(): string;
    read_file(asset_path: string): Promise<string | Buffer>;
}