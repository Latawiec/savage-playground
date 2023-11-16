import { IAssetStorage } from "./IAssetStorage";

export class NullAssetStorage implements IAssetStorage {
    get source(): string {
        return "null";
    }
    read_file(asset_path: string): Promise<string | Buffer> {
        return Promise.reject("Null storage.");
    }

}