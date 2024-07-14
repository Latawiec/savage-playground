import { IAssetStorage } from './IAssetStorage'
import path from 'path'

// Meshes
import screenSpaceRectSrc from './local_assets/mesh/screen_space_rect.json'

// Shaders
import blitTextureVsSrc from './local_assets/shader/blit_texture.vs.glsl'
import blitTexturePsSrc from './local_assets/shader/blit_texture.ps.glsl'

const LOCAL_ASSETS_PATH = process.env.VUE_APP_LOCAL_ASSETS_PATH

export class LocalAssetStorage implements IAssetStorage {
    private _availableAssets = new Set([
      screenSpaceRectSrc,
      blitTextureVsSrc,
      blitTexturePsSrc
    ]);

    // Impl IAssetStorage
    get source (): string {
      return 'local'
    }

    async readFile (assetPath: string): Promise<string | Buffer> {
      if (!LOCAL_ASSETS_PATH) {
        throw new Error('LOCAL_ASSETS_PATH not defined')
      }

      if (this._availableAssets.has(assetPath)) {
        const result = await fetch(path.join(LOCAL_ASSETS_PATH, assetPath)).then(response => response.arrayBuffer())
        // TODO: Avoid Buffer.from() since it allocates. Can I just forward a Blob instead?
        return Buffer.from(result)
      } else {
        throw new Error(`Local asset ${assetPath} not found.`)
      }
    }
}
