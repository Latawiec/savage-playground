import { DrawBundle } from "../.gen/proto/draw_bundle";
import { SceneUpdate, UpdateType } from "../.gen/proto/scene_update";
import { FloatArray } from "../.gen/proto/types";
import { UniformAttributes } from "../.gen/proto/uniform_attributes";
import { IAssetStorage } from "../asset_storage/IAssetStorage";
import { CommitedResourceStorage } from "./commited_storage/CommitedResourceStorage";
import { Mesh } from "./commited_storage/MeshStorage";
import { ShaderProgram } from "./commited_storage/ProgramStorage";
import { Texture } from "./commited_storage/TextureStorage";
import { ShaderValueType } from "./common/GLTypes";
import { BackBufferTarget } from "./render_target/BackBufferTarget";
import { MainTarget } from "./render_target/MainTarget";
import { RenderTarget } from "./render_target/RenderTarget";


export class Renderer {
    private _gl: WebGLRenderingContext;
    private _resourcesStorage: CommitedResourceStorage;
    private _backBufferRenderTarget: RenderTarget;
    private _mainRenderTarget: RenderTarget;
    private _sceneCache: Map<string, DrawBundle>;

    constructor(canvas: HTMLCanvasElement, assets: IAssetStorage) {
        const gl = canvas.getContext('webgl', {
            alpha: false
        });

        if (!gl) {
            throw new Error("Couldn't get WebGL context");
        }

        this._gl = gl;
        this._resourcesStorage = new CommitedResourceStorage(gl, assets);
        this._backBufferRenderTarget = new BackBufferTarget(gl, canvas.width, canvas.height);
        this._mainRenderTarget = new MainTarget(gl, canvas.width, canvas.height);
        this._sceneCache = new Map();
    }

    async update(scene_update: SceneUpdate) {
        if (scene_update.type === UpdateType.Increment) {
            throw new Error('Increment rendering not yet implemented');
        }

        if (scene_update.type === UpdateType.Full) {
            this._sceneCache.clear();
        }

        const drawCommands = this.commands;

        for (const element of scene_update.elements) {
            const id = element.id;
            const bundle = element.drawBundle;
            if (!bundle) {
                // Make sure it's deleted from the caches.
                continue;
            }

            try {
                const program = await this.fetchProgram(element.drawBundle!);
                const mesh = await this.fetchMesh(element.drawBundle!);

            }
        }
    }

    // Fetch steps:
    private async fetchProgram (drawable: DrawBundle): Promise<ShaderProgram> {
        const vertexShaderSrc = drawable.vertexShaderAsset;
        const pixelShaderSrc = drawable.pixelShaderAsset;
  
        if (!vertexShaderSrc || !pixelShaderSrc) { throw new Error('Not all shaders provided.') }
  
        try {
          const program = await this._resourcesStorage.programs.read(vertexShaderSrc, pixelShaderSrc)
          return program
        } catch (e) {
          throw new Error(`Couldn't prepare program [${vertexShaderSrc} + ${pixelShaderSrc}]: ${e}`)
        }
      }
  
      private async fetchMesh (drawable: DrawBundle): Promise<Mesh> {
        const meshSrc = drawable.meshAsset;
  
        if (meshSrc === undefined) { throw new Error('No mesh provided.') }
  
        try {
          const mesh = await this._resourcesStorage.meshes.read(meshSrc)
          return mesh
        } catch (e) {
          throw new Error(`Couldn't prepare mesh ${meshSrc}: ${e}`)
        }
      }
  
      private async fetchTextures (drawable: DrawBundle): Promise<Map<number, Texture> | undefined> {
        const texturesOpt = drawable.textures;
  
        // Textures are not mandatory.
        if (texturesOpt) {
          const textures = texturesOpt
          const texturesMap = new Map()
          for (const texture of textures) {
            const texture_offset = texture.bindOffset;
            const texture_asset = texture.asset;

            this._resourcesStorage.textures.read(texture_asset).then(
              (texture) => {
                texturesMap.set(texture_offset, texture)
              },
              (error) => {
                console.error(`Couldn't prepare texture ${texture_asset}: ${error}`)
              }
            )
          }
          return texturesMap
        }
  
        return undefined
      }
  
      private async fetchUniformAttributes (drawable: DrawBundle): Promise<Map<ShaderValueType, Map<string, number[]>> | undefined> {
        const localUniformsOpt = drawable.uniformAttributes;
  
        if (localUniformsOpt === undefined) {
          return undefined
        }
  
        const uniformsMap: Map<ShaderValueType, Map<string, number[]>> = new Map()
  
        // Uniforms are not mandatory.
        if (localUniformsOpt) {
          const uniforms = localUniformsOpt
  
          for (const valueType in uniforms) {
            const type = valueType as ShaderValueType;
            const typedUniformsOpt = uniforms[valueType as keyof UniformAttributes]

            if (typedUniformsOpt) {
              const typedUniforms = typedUniformsOpt;
              const uniformValues = new Map<string, number[]>();

              for (const uniformName in typedUniforms) {
                const uniformValue = typedUniforms[uniformName]
                uniformValues.set(uniformName, uniformValue.values)
              }

              uniformsMap.set(type, uniformValues)
            }
          }
        }
  
        return uniformsMap
      }
  
      private async fetchVertexAttributes (drawable: DrawBundle): Promise<Map<string, string>> {
        const vertexAttrs = drawable.vertexAttributes
        const vertexAttrsMap: Map<string, string> = new Map()
  
        // Vertices are mandatory. Other named attributes aren't.
        if (!vertexAttrs || !vertexAttrs.vertices) {
          throw new Error('Vertex attribute to bind positions to not found.')
        }
  
        vertexAttrsMap.set('vertices', vertexAttrs.vertices)
  
        // Vertex attributes for named buffers are not mandatory.
        if (vertexAttrs.namedBuffers) {
          const namedBufferAttributes = vertexAttrs.namedBuffers
  
          for (const bufferName in namedBufferAttributes) {
            const attributeName = namedBufferAttributes[bufferName]
            vertexAttrsMap.set(bufferName, attributeName)
          }
        }
        return vertexAttrsMap;
      }
  
      private async fetchLayer (drawable: DrawBundle): Promise<number> {
        const DEFAULT_LAYER = 0
        return drawable.layer ? drawable.layer : DEFAULT_LAYER
      }
  
      private async fetchBillboard (drawable: DrawBundle): Promise<boolean> {
        const DEFAULT_BILLBOARD = false
        return drawable.billboard ? drawable.billboard : DEFAULT_BILLBOARD
      }
}