import { IAssetStorage } from '../../assets/IAssetStorage';
import { MeshStorage } from './MeshStorage'
import { ProgramStorage } from './ProgramStorage'
import { TextureStorage } from './TextureStorage'

export class CommitedResourceStorage {
    private textureStorage: TextureStorage;
    private meshStorage: MeshStorage;
    private programStorage: ProgramStorage;

    constructor (gl: WebGLRenderingContext, assetStorage: IAssetStorage) {
      this.textureStorage = new TextureStorage(gl, assetStorage)
      this.meshStorage = new MeshStorage(gl, assetStorage)
      this.programStorage = new ProgramStorage(gl, assetStorage)
    }

    get textures () {
      return this.textureStorage
    }

    get meshes () {
      return this.meshStorage
    }

    get programs () {
      return this.programStorage
    }
}
