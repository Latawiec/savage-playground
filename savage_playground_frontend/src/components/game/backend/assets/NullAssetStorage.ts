import { IAssetStorage } from './IAssetStorage'

export class NullAssetStorage implements IAssetStorage {
  get source (): string {
    return 'null'
  }

  readFile (_assetPath: string): Promise<string | Buffer> {
    return Promise.reject(new Error('Null storage.'))
  }
}
