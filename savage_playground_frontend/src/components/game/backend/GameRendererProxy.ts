import { ShaderValueType } from "./common/GLTypes";
import { hasValues } from "./common/Objects";
import { Renderer } from "./communication/GameMessage";
import { GameRenderer } from "./renderer/GameRenderer";
import { Texture } from "./renderer/gl_resource/TextureStorage";
import { ShaderProgram } from "./renderer/gl_resource/ProgramStorage";
import { CommitedResourceStorage } from "./renderer/gl_resource/CommitedResourceStorage";
import { Mesh } from "./renderer/gl_resource/MeshStorage";
import { IDrawCommand } from "./renderer/pipeline/command/IDrawCommand";
import { GeneralDrawCommand } from "./renderer/pipeline/command/GeneralDrawCommand";
import { ZipAssetStorage } from "./assets/ZipAssetStorage";

interface GlobalUniformValues {
    view: number[],
    proj: number[],
    camera_forward: number[],
}

export class GameRendererProxy {
    private _game_canvas: HTMLCanvasElement;
    private _game_renderer: GameRenderer;
    private _saved_snapshot: Renderer.Snapshot;

    constructor(game_canvas: HTMLCanvasElement) {
        const gl = game_canvas.getContext('webgl',
            {
                alpha: false
            })!;

        this._game_canvas = game_canvas;
        this._saved_snapshot = {};
        this._game_renderer = new GameRenderer(gl);
        this._game_renderer.resize_buffers(game_canvas.width, game_canvas.height);

        game_canvas.addEventListener('resize', this.on_game_canvas_resize);
    }

    load_assets_package(source: string): Promise<void> {
        return new Promise(async(resolve) => {
            const asset_storage = await ZipAssetStorage.from_file(source);
            this._game_renderer.external_asset_storage = asset_storage;
            resolve();
        });
    }

    render_snapshot(renderer_snapshot: Renderer.Snapshot): Promise<void> {
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

            const global_uniforms: GlobalUniformValues = {
                view: camera.view_transform!,
                proj: camera.proj_transform!,
                camera_forward: [0, 0, 0],
            };

            const draw_commands: IDrawCommand[] = [];

            for (const entity_id in entities) {
                const entity = entities[entity_id];

                try {
                    const program = await this.fetch_program(entity);
                    const mesh = await this.fetch_mesh(entity);
                    const textures = await this.fetch_textures(entity);
                    const uniforms = await this.fetch_uniform_attributes(entity, global_uniforms);
                    const vertex_attrs = await this.fetch_vertex_attributes(entity);
                    const layer = await this.fetch_layer(entity);
                    const billboard = await this.fetch_billboard(entity);

                    // Build Draw Command
                    const draw_command = new GeneralDrawCommand(
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

            await this._game_renderer.execute_draw_commands(draw_commands);
            await this._game_renderer.present();

            resolve();
        });
    }

    private get gl_resources(): CommitedResourceStorage {
        return this._game_renderer.external_resource_storage;
    }

    private on_game_canvas_resize(e: UIEvent) {
        this._game_renderer.resize_buffers(this._game_canvas.width, this._game_canvas.height);
    }

    // Fetch steps:
    private async fetch_program(drawable: Renderer.Drawable): Promise<ShaderProgram> {
        return new Promise(async (resolve, reject) => {
            const vertex_shader_src = drawable.assets?.vertex_shader;
            const pixel_shader_src = drawable.assets?.pixel_shader;
            if (vertex_shader_src === undefined || pixel_shader_src === undefined) {
                reject('Not all shaders provided.');
            }
            this.gl_resources.programs.read(vertex_shader_src!, pixel_shader_src!).then(
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
        return new Promise(async (resolve, reject) => {
            const mesh_src = drawable.assets?.mesh;
            if (mesh_src === undefined) {
                reject('No mesh provided.');
            }
            this.gl_resources.meshes.read(mesh_src!).then(
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
        return new Promise(async (resolve, reject) => {
            const textures_opt = drawable.assets?.textures;
            let textures_map: Map<number, Texture> | undefined = undefined;
            // Textures are not mandatory.
            if (textures_opt !== undefined && hasValues(textures_opt)) {
                const textures = textures_opt!;
                textures_map = new Map();
                for (const texture_id in textures) {
                    const texture_src = textures[texture_id];
                    this.gl_resources.textures.read(texture_src).then(
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

    private async fetch_uniform_attributes(drawable: Renderer.Drawable, global_uniforms: GlobalUniformValues): Promise<Map<ShaderValueType, Map<string, number | number[]>> | undefined> {
        return new Promise(async (resolve, reject) => {
            const local_uniforms_opt = drawable.local_uniform_attributes;
            const global_uniforms_names_opt = drawable.global_uniform_attributes;

            if (local_uniforms_opt === undefined && global_uniforms_names_opt === undefined) {
                resolve(undefined);
            }

            let uniforms_map: Map<ShaderValueType, Map<string, number | number[]>> = new Map();

            // Uniforms are not mandatory.
            if (local_uniforms_opt !== undefined && hasValues(local_uniforms_opt)) {
                let uniforms = local_uniforms_opt!;
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

            // Check global uniforms
            if (global_uniforms_names_opt !== undefined && hasValues(global_uniforms_names_opt)) {
                let uniform_names = global_uniforms_names_opt!;
                if (uniform_names.mat4 !== undefined) {
                    if (!uniforms_map.has('mat4')) { uniforms_map.set('mat4', new Map()); }

                    const camera_view_name = uniform_names.mat4.camera_view;
                    const camera_proj_name = uniform_names.mat4.camera_proj;

                    if (camera_view_name) {
                        uniforms_map.get('mat4')!.set(camera_view_name, global_uniforms.view);
                    }

                    if (camera_proj_name) {
                        uniforms_map.get('mat4')!.set(camera_proj_name, global_uniforms.proj);
                    }
                }

                if (uniform_names.vec3 !== undefined) {
                    if (!uniforms_map.has('vec3')) { uniforms_map.set('vec3', new Map()); }

                    const camera_forward_name = uniform_names.vec3.camera_forward;

                    if (camera_forward_name !== undefined) {
                        uniforms_map.get('vec3')!.set(camera_forward_name, global_uniforms.camera_forward);
                    }
                }
            }

            resolve(uniforms_map);
        });
    }

    private async fetch_vertex_attributes(drawable: Renderer.Drawable): Promise<Map<string, string>> {
        return new Promise(async (resolve, reject) => {
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
        return new Promise(async (resolve, reject) => {
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
        return new Promise(async (resolve, reject) => {
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