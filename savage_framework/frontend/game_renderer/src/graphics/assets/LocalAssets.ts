import { CommitedResourceStorage } from '../commited_storage/CommitedResourceStorage';

import MESH_SCREEN_SPACE_RECT_URL from './mesh/screen_space_rect.json';
import SHADER_BLIT_TEXTURE_PS_URL from './shader/blit_texture.ps.glsl';
import SHADER_BLIT_TEXTURE_VS_URL from './shader/blit_texture.vs.glsl';

const LOCAL_ASSET_PREFIX = "/L0C4L/";

const MESH_SCREEN_SPACE_RECT_RESOURCE_PATH = LOCAL_ASSET_PREFIX + MESH_SCREEN_SPACE_RECT_URL;
const SHADER_BLIT_TEXTURE_PS_RESOURCE_PATH = LOCAL_ASSET_PREFIX + SHADER_BLIT_TEXTURE_PS_URL;
const SHADER_BLIT_TEXTURE_VS_RESOURCE_PATH = LOCAL_ASSET_PREFIX + SHADER_BLIT_TEXTURE_VS_URL;

async function loadLocalAssets(commited_storage: CommitedResourceStorage): Promise<void> {
    // Mesh
    const mesh_screen_space_rect = await fetch(MESH_SCREEN_SPACE_RECT_URL);
    commited_storage.meshes.write(MESH_SCREEN_SPACE_RECT_RESOURCE_PATH, await mesh_screen_space_rect.json());

    // Shader
    const shader_blit_texture_vs = await fetch(SHADER_BLIT_TEXTURE_VS_URL);
    const shader_blit_texture_ps = await fetch(SHADER_BLIT_TEXTURE_PS_URL);
    commited_storage.programs.write(
        SHADER_BLIT_TEXTURE_VS_RESOURCE_PATH,
        await shader_blit_texture_vs.text(),
        SHADER_BLIT_TEXTURE_PS_RESOURCE_PATH,
        await shader_blit_texture_ps.text()
    );
}

export {
    MESH_SCREEN_SPACE_RECT_RESOURCE_PATH,
    SHADER_BLIT_TEXTURE_PS_RESOURCE_PATH,
    SHADER_BLIT_TEXTURE_VS_RESOURCE_PATH,
    loadLocalAssets
}