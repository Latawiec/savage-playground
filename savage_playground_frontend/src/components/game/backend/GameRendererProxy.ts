import { ShaderValueType } from "./common/GLTypes";
import { hasValues } from "./common/Objects";
import { Renderer } from "./communication/GameMessage";
import { GameRenderer } from "./renderer/GameRenderer";
import { Texture } from "./renderer/gl_resource/TextureStorage";
import { DrawCommand } from "./renderer/pipeline/DrawCommand";


export class GameRendererProxy {
    private _game_renderer: GameRenderer;
    private _saved_snapshot: Renderer.Snapshot;

    constructor(game_renderer: GameRenderer) {
        this._game_renderer = game_renderer;
        this._saved_snapshot = {};
    }

    async render(renderer_snapshot: Renderer.Snapshot): Promise<void> {
        if (renderer_snapshot.type === 'increment') {
            throw new Error("Increment rendering not yet implemented");
        }

        if (renderer_snapshot.type === 'reset' || renderer_snapshot.type === undefined) {
            // Reset.
            this._saved_snapshot = renderer_snapshot;
        }

        new Promise(async (resolve, reject) => {

            if (this._saved_snapshot.camera === undefined) {
                // No camera defined. Nothing to draw.
                reject("No camera defined. No idea what to draw...");
            }

            if (this._saved_snapshot.entities === undefined || !hasValues(this._saved_snapshot.entities)) {
                // No entities to draw.
                reject("No entities defined. Nothing to draw...");
            }

            const camera = renderer_snapshot.camera!;
            const entities = renderer_snapshot.entities!;

            const draw_commands: DrawCommand[] = [];
            const resources = this._game_renderer.resource_storage;

            for (const entity_id in entities) {
                const entity = entities[entity_id];
                
                // Program
                const vertex_shader_src = entity.assets?.vertex_shader;
                const pixel_shader_src = entity.assets?.pixel_shader;
                if (vertex_shader_src === undefined || pixel_shader_src === undefined) {
                    console.error(`Cannot render entity ${entity_id}. Not all shaders provided.`);
                    continue;
                }
                const program = await resources.programs.read(vertex_shader_src!, pixel_shader_src!);

                // Mesh
                const mesh_src = entity.assets?.mesh;
                if (mesh_src === undefined) {
                    console.error(`Cannot render entity ${entity_id}. No mesh provided.`);
                    continue;
                }
                const mesh = await resources.meshes.read(mesh_src!);

                // Textures
                const textures_opt = entity.assets?.textures;
                let textures_map: Map<number, Texture> | undefined = undefined;
                // Textures are not mandatory.
                if (textures_opt !== undefined && hasValues(textures_opt)) {
                    const textures = textures_opt!;
                    textures_map = new Map();
                    for (const texture_id in textures) {
                        const texture_src = textures[texture_id];
                        const texture = await resources.textures.read(texture_src);
                        textures_map.set(parseInt(texture_id), texture);
                    }
                }

                // Uniform attributes
                const uniforms_opt = entity.uniform_attributes;
                let uniforms_map: Map<ShaderValueType, Map<string, number | number[]>> | undefined = undefined;
                // Uniforms are not mandatory.
                if (uniforms_opt !== undefined && hasValues(uniforms_opt)) {
                    let uniforms = uniforms_opt!;
                    for (const value_type in uniforms) {
                        // Continue tomorrow.
                        // const typed_uniforms = uniforms[value_type];
                    }
                }
            }
        })
    }
}