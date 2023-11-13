import { MeshStorage } from '../gl_resource/MeshStorage'
import { ProgramStorage } from '../gl_resource/ProgramStorage'

// Meshes
import screen_space_rect_src from './mesh/screen_space_rect.json'

// Shaders
import blit_texture_vs_src from './shader/blit_texture.vs.glsl'
import blit_texture_ps_src from './shader/blit_texture.ps.glsl'

const LOCAL_ASSETS_PATH = process.env.VUE_APP_LOCAL_ASSETS_PATH!;

export class LocalAssets {
    static async store_local_meshes(mesh_storage: MeshStorage): Promise<void> {
        return new Promise(async (resolve, reject) => {
            try {
                const screen_space_rect_json_mesh = await fetch(LOCAL_ASSETS_PATH + screen_space_rect_src).then(response => response.json());
                mesh_storage.write(screen_space_rect_src, screen_space_rect_json_mesh);
            } catch(e) {
                reject(e);
            }
            resolve();
        })
    }

    static async store_local_shaders(program_storage: ProgramStorage): Promise<void> {
        return new Promise(async (resolve, reject) => {
            try {
                const blit_texture_vs = await fetch(LOCAL_ASSETS_PATH + blit_texture_vs_src).then(response => response.text());
                const blit_texture_ps = await fetch(LOCAL_ASSETS_PATH + blit_texture_ps_src).then(response => response.text());

                program_storage.write(
                    blit_texture_vs_src, blit_texture_vs,
                    blit_texture_ps_src, blit_texture_ps
                );
            } catch(e) {
                reject(e);
            }
            resolve();
        });
    }
}