import { IAssetStorage } from "./IAssetStorage";

// Meshes
import screen_space_rect_src from './local_assets/mesh/screen_space_rect.json'

// Shaders
import blit_texture_vs_src from './local_assets/shader/blit_texture.vs.glsl'
import blit_texture_ps_src from './local_assets/shader/blit_texture.ps.glsl'

const LOCAL_ASSETS_PATH = process.env.VUE_APP_LOCAL_ASSETS_PATH!;

export class LocalAssetStorage implements IAssetStorage {
    private _available_assets = new Set([
        screen_space_rect_src,
        blit_texture_vs_src,
        blit_texture_ps_src
    ]);

    // Impl IAssetStorage
    get source(): string {
        return "local";
    }

    read_file(asset_path: string): Promise<string | Buffer> {
        return new Promise((resolve, reject) => {
            if (this._available_assets.has(asset_path)) {
                return fetch(LOCAL_ASSETS_PATH + asset_path).then(response => response.arrayBuffer())
            } else {
                reject(`Local asset ${asset_path} not found.`);
            }
        })
    }
}