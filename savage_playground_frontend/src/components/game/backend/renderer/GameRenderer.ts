import { mat4 } from "gl-matrix";
import { CommitedResourceStorage } from "./gl_resource/CommitedResourceStorage";
import { DrawCommand } from "./pipeline/DrawCommand";
import { BackBufferTarget } from "./pipeline/render_target/BackBufferTarget";
import { MainTarget } from "./pipeline/render_target/MainTarget";
import { AssetStorage } from "../AssetStorage";
import { LocalAssets } from "./base_assets/LocalAssets";

export class GameRenderer {
    private _gl: WebGLRenderingContext;
    private _game_canvas: HTMLCanvasElement;
    private _commited_resource_storage: CommitedResourceStorage;
    private _back_buffer_render_target: BackBufferTarget;
    private _main_render_target: MainTarget;

    constructor (
      game_canvas: HTMLCanvasElement,
    ) {
      this._gl = game_canvas.getContext('webgl', 
      {
          alpha: false
      })!;
      this._game_canvas = game_canvas
      this._commited_resource_storage = new CommitedResourceStorage(this._gl, AssetStorage.empty());

      this._back_buffer_render_target = new BackBufferTarget(this._gl, game_canvas.width, game_canvas.height);
      this._main_render_target = new MainTarget(this._gl, game_canvas.width, game_canvas.height);

      game_canvas.addEventListener('resize', this.on_game_canvas_resize)
    }

    set_asset_storage(asset_storage: AssetStorage) {
      this._commited_resource_storage = new CommitedResourceStorage(this._gl, asset_storage);
      // Add renderer required assets.
      LocalAssets.store_local_meshes(this._commited_resource_storage.meshes);
      LocalAssets.store_local_shaders(this._commited_resource_storage.programs);
    }

    get resource_storage(): CommitedResourceStorage {
      return this._commited_resource_storage;
    }

    execute_draw_commands(draw_commands: DrawCommand[]) {
      const gl = this._gl;

      this._back_buffer_render_target.bind();
      gl.clearColor(0.0, 0.0, 0.0, 1.0);
      gl.clearDepth(1.0);
      gl.disable(gl.DEPTH_TEST);
      gl.enable(gl.BLEND);
      
      gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

      for(const draw_command of draw_commands) {
        draw_command.draw(this._gl)
      }
    }

    present() {
      const gl = this._gl;
      this._main_render_target.bind();
      gl.disable(gl.DEPTH_TEST);
      gl.disable(gl.BLEND);
      
    }
  
    private on_game_canvas_resize(event: UIEvent) {
      this._back_buffer_render_target = new BackBufferTarget(this._gl, this._game_canvas.width, this._game_canvas.height);
      this._main_render_target = new MainTarget(this._gl, this._game_canvas.width, this._game_canvas.height);
    }

    private gl_context_setup(gl: WebGLRenderingContext){
      gl.enable(gl.DEPTH_TEST);
      gl.enable(gl.BLEND);
      gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    }
}
