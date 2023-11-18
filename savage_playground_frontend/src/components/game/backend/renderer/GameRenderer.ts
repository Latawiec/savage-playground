import { CommitedResourceStorage } from './gl_resource/CommitedResourceStorage'
import { BackBufferTarget } from './pipeline/render_target/BackBufferTarget'
import { MainTarget } from './pipeline/render_target/MainTarget'
import { IDrawCommand } from './pipeline/command/IDrawCommand'
import { ZipAssetStorage } from '../assets/ZipAssetStorage'
import { LocalAssetStorage } from '../assets/LocalAssetStorage'
import { IAssetStorage } from '../assets/IAssetStorage'

export class GameRenderer {
    private _gl: WebGLRenderingContext;
    private _externalResourcesStorage: CommitedResourceStorage;
    private _localResourcesStorage: CommitedResourceStorage;
    private _backBufferRenderTarget: BackBufferTarget;
    private _mainRenderTarget: MainTarget;

    constructor (
      gl: WebGLRenderingContext
    ) {
      this._gl = gl
      this._externalResourcesStorage = new CommitedResourceStorage(this._gl, ZipAssetStorage.fromEmpty())
      this._localResourcesStorage = new CommitedResourceStorage(this._gl, new LocalAssetStorage())

      // Just some default values. I expect it to be overwritten anyways.
      this._backBufferRenderTarget = new BackBufferTarget(this._gl, 100, 100)
      this._mainRenderTarget = new MainTarget(this._gl, 100, 100)
    }

    setExternalAssetStorage (assetStorage: IAssetStorage) {
      this._externalResourcesStorage = new CommitedResourceStorage(this._gl, assetStorage)
    }

    get externalResourceStorage (): CommitedResourceStorage {
      return this._externalResourcesStorage
    }

    get localResourceStorage (): CommitedResourceStorage {
      return this._localResourcesStorage
    }

    async clear () {
      const gl = this._gl

      this._mainRenderTarget.bind()
      gl.clearColor(0.0, 1.0, 0.0, 1.0)
      gl.clearDepth(1.0)
      gl.disable(gl.DEPTH_TEST)
      gl.disable(gl.BLEND)

      gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)
    }

    async executeDrawCommands (drawCommands: IDrawCommand[]) {
      this._mainRenderTarget.bind()
      for (const drawCommand of drawCommands) {
        drawCommand.draw(this._gl)
      }
    }

    async present () {
      // const gl = this._gl;
      // this._main_render_target.bind();

      // const draw_command = await PresentDrawCommand.from_resources(this._local_resources_storage, this._back_buffer_render_target.color_texture);
      // draw_command.draw(gl);
      console.log(this._gl.getError())
    }

    resizeBuffers (width: number, height: number) {
      this._backBufferRenderTarget = new BackBufferTarget(this._gl, width, height)
      this._mainRenderTarget = new MainTarget(this._gl, width, height)
    }
}
