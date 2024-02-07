import { ShaderValueType } from './common/GLTypes'
import { hasValues } from './common/Objects'
import { Renderer } from './communication/GameMessage'
import { GameRenderer } from './renderer/GameRenderer'
import { Texture } from './renderer/gl_resource/TextureStorage'
import { ShaderProgram } from './renderer/gl_resource/ProgramStorage'
import { CommitedResourceStorage } from './renderer/gl_resource/CommitedResourceStorage'
import { Mesh } from './renderer/gl_resource/MeshStorage'
import { IDrawCommand } from './renderer/pipeline/command/IDrawCommand'
import { GeneralDrawCommand } from './renderer/pipeline/command/GeneralDrawCommand'
import { ZipAssetStorage } from './assets/ZipAssetStorage'

interface GlobalUniformValues {
    view?: number[],
    proj?: number[],
    cameraForward?: number[],
}

export class GameRendererProxy {
    private _gameCanvas: HTMLCanvasElement;
    private _gameRenderer: GameRenderer;
    private _savedSnapshot: Renderer.Snapshot;

    constructor (gameCanvas: HTMLCanvasElement) {
      const gl = gameCanvas.getContext('webgl',
        {
          alpha: false
        })

      if (!gl) {
        throw new Error('Couldn\'t get WebGL context')
      }

      this._gameCanvas = gameCanvas
      this._savedSnapshot = {}
      this._gameRenderer = new GameRenderer(gl)
      this._gameRenderer.resizeBuffers(gameCanvas.width, gameCanvas.height)

      gameCanvas.addEventListener('resize', this.onGameCanvasResize)
    }

    async loadAssetsPackage (source: string) {
      try {
        const assetStorage = await ZipAssetStorage.fromFile(source)
        this._gameRenderer.setExternalAssetStorage(assetStorage)
      } catch (e) {
        throw new Error(`Couldn't load asset package: ${e}`)
      }
    }

    async renderSnapshot (rendererSnapshot: Renderer.Snapshot) {
      if (rendererSnapshot.type === 'increment') {
        throw new Error('Increment rendering not yet implemented')
      }

      if (rendererSnapshot.type === 'reset' || rendererSnapshot.type === undefined) {
        // Reset.
        this._savedSnapshot = rendererSnapshot
      }

      if (!this._savedSnapshot.entities || !hasValues(this._savedSnapshot.entities)) {
        // No entities to draw.
        throw new Error('No entities defined. Nothing to draw...')
      }

      const camera = rendererSnapshot.camera
      const entities = rendererSnapshot.entities

      const globalUniforms: GlobalUniformValues = {
        view: camera?.viewTransform,
        proj: camera?.projTransform,
        cameraForward: [0, 0, 0]
      }

      const drawCommands: IDrawCommand[] = []

      for (const entityId in entities) {
        const entity = entities[entityId]

        try {
          const program = await this.fetchProgram(entity)
          const mesh = await this.fetchMesh(entity)
          const textures = await this.fetchTextures(entity)
          const uniforms = await this.fetchUniformAttributes(entity, globalUniforms)
          const vertexAttrs = await this.fetchVertexAttributes(entity)
          const layer = await this.fetchLayer(entity)
          const billboard = await this.fetchBillboard(entity)

          // Build Draw Command
          const drawCommand = new GeneralDrawCommand(
            program,
            mesh,
            textures,
            uniforms,
            vertexAttrs,
            layer,
            billboard
          )

          drawCommands.push(drawCommand)
        } catch (rejection) {
          console.error(`Couldn't render ${entityId}: ${rejection}`)
        }
      }

      try {
        await this._gameRenderer.clear()
        await this._gameRenderer.executeDrawCommands(drawCommands)
        await this._gameRenderer.present()
      } catch (e) {
        console.log(`Render has failed: ${e}`)
      }
    }

    private get glResources (): CommitedResourceStorage {
      return this._gameRenderer.externalResourceStorage
    }

    private onGameCanvasResize () {
      this._gameRenderer.resizeBuffers(this._gameCanvas.width, this._gameCanvas.height)
    }

    // Fetch steps:
    private async fetchProgram (drawable: Renderer.Drawable): Promise<ShaderProgram> {
      const vertexShaderSrc = drawable.assets?.vertexShader
      const pixelShaderSrc = drawable.assets?.pixelShader

      if (!vertexShaderSrc || !pixelShaderSrc) { throw new Error('Not all shaders provided.') }

      try {
        const program = await this.glResources.programs.read(vertexShaderSrc, pixelShaderSrc)
        return program
      } catch (e) {
        throw new Error(`Couldn't prepare program [${vertexShaderSrc} + ${pixelShaderSrc}]: ${e}`)
      }
    }

    private async fetchMesh (drawable: Renderer.Drawable): Promise<Mesh> {
      const meshSrc = drawable.assets?.mesh

      if (meshSrc === undefined) { throw new Error('No mesh provided.') }

      try {
        const mesh = await this.glResources.meshes.read(meshSrc)
        return mesh
      } catch (e) {
        throw new Error(`Couldn't prepare mesh ${meshSrc}: ${e}`)
      }
    }

    private async fetchTextures (drawable: Renderer.Drawable): Promise<Map<number, Texture> | undefined> {
      const texturesOpt = drawable.assets?.textures

      // Textures are not mandatory.
      if (texturesOpt && hasValues(texturesOpt)) {
        const textures = texturesOpt
        const texturesMap = new Map()
        for (const textureId in textures) {
          const textureSrc = textures[textureId]
          this.glResources.textures.read(textureSrc).then(
            (texture) => {
              texturesMap.set(parseInt(textureId), texture)
            },
            (error) => {
              console.error(`Couldn't prepare texture ${textureSrc}: ${error}`)
            }
          )
        }
        return texturesMap
      }

      return undefined
    }

    private async fetchUniformAttributes (drawable: Renderer.Drawable, globalUniforms: GlobalUniformValues): Promise<Map<ShaderValueType, Map<string, number | number[]>> | undefined> {
      const localUniformsOpt = drawable.localUniformAttributes
      const globalUniformsNamesOpt = drawable.globalUniformAttributes

      if (localUniformsOpt === undefined && globalUniformsNamesOpt === undefined) {
        return undefined
      }

      const uniformsMap: Map<ShaderValueType, Map<string, number | number[]>> = new Map()

      // Uniforms are not mandatory.
      if (localUniformsOpt && hasValues(localUniformsOpt)) {
        const uniforms = localUniformsOpt

        for (const valueType in uniforms) {
          const type = valueType as ShaderValueType
          const typedUniformsOpt = uniforms[valueType as keyof Renderer.UniformAttributes]
          if (typedUniformsOpt && hasValues(typedUniformsOpt)) {
            const typedUniforms = typedUniformsOpt
            const uniformValues = new Map<string, number | number[]>()
            for (const uniformName in typedUniforms) {
              const uniformValue = typedUniforms[uniformName]
              uniformValues.set(uniformName, uniformValue)
            }
            uniformsMap.set(type, uniformValues)
          }
        }
      }

      // Check global uniforms
      if (globalUniformsNamesOpt && hasValues(globalUniformsNamesOpt)) {
        const uniformNames = globalUniformsNamesOpt

        if (uniformNames.mat4 !== undefined) {
          if (!uniformsMap.has('mat4')) { uniformsMap.set('mat4', new Map()) }

          const cameraViewName = uniformNames.mat4.cameraView
          const cameraProjName = uniformNames.mat4.cameraProj

          if (cameraViewName && globalUniforms.view) {
            uniformsMap.get('mat4')?.set(cameraViewName, globalUniforms.view)
          }

          if (cameraProjName && globalUniforms.proj) {
            uniformsMap.get('mat4')?.set(cameraProjName, globalUniforms.proj)
          }
        }

        if (uniformNames.vec3 !== undefined) {
          if (!uniformsMap.has('vec3')) { uniformsMap.set('vec3', new Map()) }

          const cameraForwardName = uniformNames.vec3.cameraForward

          if (cameraForwardName && globalUniforms.cameraForward) {
            uniformsMap.get('vec3')?.set(cameraForwardName, globalUniforms.cameraForward)
          }
        }
      }

      return uniformsMap
    }

    private async fetchVertexAttributes (drawable: Renderer.Drawable): Promise<Map<string, string>> {
      const vertexAttrs = drawable.vertexAttributes
      const vertexAttrsMap: Map<string, string> = new Map()

      // Vertices are mandatory. Other named attributes aren't.
      if (!vertexAttrs || !vertexAttrs.vertices) {
        throw new Error('Vertex attribute to bind positions to not found.')
      }

      vertexAttrsMap.set('vertices', vertexAttrs.vertices)

      // Vertex attributes for named buffers are not mandatory.
      if (vertexAttrs.namedBuffers !== undefined && hasValues(vertexAttrs.namedBuffers)) {
        const namedBufferAttributes = vertexAttrs.namedBuffers

        for (const bufferName in namedBufferAttributes) {
          const attributeName = namedBufferAttributes[bufferName]
          vertexAttrsMap.set(bufferName, attributeName)
        }
      }
      return vertexAttrsMap
    }

    private async fetchLayer (drawable: Renderer.Drawable): Promise<number> {
      const DEFAULT_LAYER = 0
      return drawable.layer ? drawable.layer : DEFAULT_LAYER
    }

    private async fetchBillboard (drawable: Renderer.Drawable): Promise<boolean> {
      const DEFAULT_BILLBOARD = false
      return drawable.billboard ? drawable.billboard : DEFAULT_BILLBOARD
    }
}
