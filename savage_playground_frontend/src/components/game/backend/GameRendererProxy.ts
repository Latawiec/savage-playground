import { ShaderValueType } from "./common/GLTypes";
import { hasValues } from "./common/Objects";
import { Renderer } from "./communication/GameMessage";
import { GameRenderer } from "./renderer/GameRenderer";
import { Texture } from "./renderer/gl_resource/TextureStorage";
import { DrawCommand } from "./renderer/pipeline/DrawCommand";
import { glm } from "./common/glm";
import { ShaderProgram } from "./renderer/gl_resource/ProgramStorage";
import { CommitedResourceStorage } from "./renderer/gl_resource/CommitedResourceStorage";
import { Mesh } from "./renderer/gl_resource/MeshStorage";


export class GameRendererProxy {
    private _game_renderer: GameRenderer;
    private _saved_snapshot: Renderer.Snapshot;

    constructor(game_renderer: GameRenderer) {
        this._game_renderer = game_renderer;
        this._saved_snapshot = {};
    }

    private get resources(): CommitedResourceStorage {
        return this._game_renderer.resource_storage;
    }

    async render(renderer_snapshot: Renderer.Snapshot): Promise<void> {
        if (renderer_snapshot.type === 'increment') {
            throw new Error("Increment rendering not yet implemented");
        }

        if (renderer_snapshot.type === 'reset' || renderer_snapshot.type === undefined) {
            // Reset.
            this._saved_snapshot = renderer_snapshot;
        }

        return new Promise(async (resolve, reject) => {

            if (this._saved_snapshot.camera === undefined || this._saved_snapshot.camera.proj_transform === undefined || this._saved_snapshot.camera.view_transform === undefined) {
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

                try {
                    const program = await this.fetch_program(entity);
                    const mesh = await this.fetch_mesh(entity);
                    const textures = await this.fetch_textures(entity);
                    const uniforms = await this.fetch_uniform_attributes(entity);
                    // TODO:
                    // const global_uniforms = await this.fetch_global_uniform_attributes(entity);
                    const vertex_attrs = await this.fetch_vertex_attributes(entity);
                    const layer = await this.fetch_layer(entity);
                    const billboard = await this.fetch_billboard(entity);

                    // Build Draw Command
                    const draw_command = new DrawCommand(
                        program,
                        mesh,
                        textures,
                        uniforms,
                        vertex_attrs,
                        layer,
                        billboard
                    );

                    draw_commands.push(draw_command);
                } catch (rejection) {
                    console.error(`Couldn't render ${entity_id}: ${rejection}`);
                }
            }
            
            this._game_renderer.execute_draw_commands(draw_commands);
            this._game_renderer.present();

            resolve();
        });
    }

// Fetch steps:
    private async fetch_program(drawable: Renderer.Drawable): Promise<ShaderProgram> {
        return new Promise(async (resolve, reject) => {
            const vertex_shader_src = drawable.assets?.vertex_shader;
            const pixel_shader_src = drawable.assets?.pixel_shader;
            if (vertex_shader_src === undefined || pixel_shader_src === undefined) {
                reject('Not all shaders provided.');
            }
            this.resources.programs.read(vertex_shader_src!, pixel_shader_src!).then(
                (program) => {
                    resolve(program);
                },
                (error) => {
                    reject(`Couldn't prepare program [${vertex_shader_src} + ${pixel_shader_src}]: ${error}`);
                }
            );
        });
    }

    private async fetch_mesh(drawable: Renderer.Drawable): Promise<Mesh> {
        return new Promise(async(resolve, reject) => {
            const mesh_src = drawable.assets?.mesh;
            if (mesh_src === undefined) {
                reject('No mesh provided.');
            }
            this.resources.meshes.read(mesh_src!).then(
                (mesh) => {
                    resolve(mesh)
                },
                (error) => {
                    reject(`Couldn't prepare mesh ${mesh_src}: ${error}`);
                }
            );
        });
    }

    private async fetch_textures(drawable: Renderer.Drawable): Promise<Map<number, Texture> | undefined> {
        return new Promise(async(resolve, reject) => {
            const textures_opt = drawable.assets?.textures;
            let textures_map: Map<number, Texture> | undefined = undefined;
            // Textures are not mandatory.
            if (textures_opt !== undefined && hasValues(textures_opt)) {
                const textures = textures_opt!;
                textures_map = new Map();
                for (const texture_id in textures) {
                    const texture_src = textures[texture_id];
                    this.resources.textures.read(texture_src).then(
                        (texture) => {
                            textures_map!.set(parseInt(texture_id), texture);
                        },
                        (error) => {
                            console.error(`Couldn't prepare texture ${texture_src}: ${error}`);
                        }
                    );
                }
            }
            resolve(textures_map);
        });
    }

    private async fetch_uniform_attributes(drawable: Renderer.Drawable): Promise<Map<ShaderValueType, Map<string, number | number[]>> | undefined> {
        return new Promise(async(resolve, reject) => {
            const uniforms_opt = drawable.uniform_attributes;
            let uniforms_map: Map<ShaderValueType, Map<string, number | number[]>> | undefined = undefined;

            // Uniforms are not mandatory.
            if (uniforms_opt !== undefined && hasValues(uniforms_opt)) {
                uniforms_map = new Map();
                let uniforms = uniforms_opt!;
                for (const value_type in uniforms) {
                    let type = value_type as ShaderValueType;
                    const typed_uniforms_opt = uniforms[value_type as keyof Renderer.UniformAttributes];
                    if (typed_uniforms_opt !== undefined && hasValues(typed_uniforms_opt)) {
                        const typed_uniforms = typed_uniforms_opt!;
                        const uniform_values = new Map<string, number | number[]>();
                        for (const uniform_name in typed_uniforms) {
                            const uniform_value = typed_uniforms[uniform_name];
                            uniform_values.set(uniform_name, uniform_value);
                        }
                        uniforms_map.set(type, uniform_values);
                    }
                }
            }

            resolve(uniforms_map);
        });
    }

    private async fetch_vertex_attributes(drawable: Renderer.Drawable): Promise<Map<string, string>> {
        return new Promise(async(resolve, reject) => {
            const vertex_attrs = drawable.vertex_attributes;
            let vertex_attrs_map: Map<string, string> = new Map();
            // Vertices are mandatory. Other named attributes aren't.
            if (vertex_attrs === undefined || vertex_attrs.vertices === undefined) {
                reject('Vertex attribute to bind positions to not found.');
            }
            vertex_attrs_map.set('vertices', vertex_attrs!.vertices!);

            // Vertex attributes for named buffers are not mandatory.
            if (vertex_attrs!.named_buffers !== undefined && hasValues(vertex_attrs!.named_buffers)) {
                const named_buffer_attributes = vertex_attrs!.named_buffers!;
                for (const buffer_name in named_buffer_attributes) {
                    const attribute_name = named_buffer_attributes[buffer_name];
                    vertex_attrs_map.set(buffer_name, attribute_name);
                }
            }

            resolve(vertex_attrs_map);
        });
    }

    private async fetch_layer(drawable: Renderer.Drawable): Promise<number> {
        const DEFAULT_LAYER = 0;
        return new Promise(async(resolve, reject) => {
            const layer_opt = drawable.layer;
            let layer = DEFAULT_LAYER;

            // Layer is not mandatory.
            if (layer_opt !== undefined) {
                layer = layer_opt!;
            }
            resolve(layer);
        });
    }

    private async fetch_billboard(drawable: Renderer.Drawable): Promise<boolean> {
        const DEFAULT_BILLBOARD = false;
        return new Promise(async(resolve, reject) => {
            const billboard_opt = drawable.billboard;
            let billboard = DEFAULT_BILLBOARD;

            // Billboard is not mandatory.
            if (billboard_opt !== undefined) {
                billboard = billboard_opt!;
            }
            resolve(billboard);
        });
    }
}