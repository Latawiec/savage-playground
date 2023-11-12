import { mat4 } from "gl-matrix";
import { CommitedResourceStorage } from "./gl_resource/CommitedResourceStorage";
import { DrawCommand } from "./pipeline/DrawCommand";
import { BackBufferTarget } from "./pipeline/render_target/BackBufferTarget";
import { MainTarget } from "./pipeline/render_target/MainTarget";

class GameRenderer {
    private _gl: WebGLRenderingContext;
    private _game_canvas: HTMLCanvasElement;
    private _commited_resource_storage: CommitedResourceStorage;
    private _back_buffer_render_target: BackBufferTarget;
    private _main_render_target: MainTarget;

    private _active_camera: {
      view: mat4,
      proj: mat4
    } | undefined;

    constructor (
      game_canvas: HTMLCanvasElement,
      commited_resource_storage: CommitedResourceStorage,
    ) {
      this._gl = game_canvas.getContext('webgl', 
      {
          alpha: false
      })!;
      this._game_canvas = game_canvas
      this._commited_resource_storage = commited_resource_storage;

      this._back_buffer_render_target = new BackBufferTarget(this._gl, game_canvas.width, game_canvas.height);
      this._main_render_target = new MainTarget(this._gl, game_canvas.width, game_canvas.height);

      game_canvas.addEventListener('resize', this.on_game_canvas_resize)
    }

    set_camera(view: mat4, proj: mat4) {
      this._active_camera = {
        view: view,
        proj: proj
      };
    }

    execute_draw_commands(draw_commands: DrawCommand[]) {
      if (this._active_camera === undefined) {
        throw new Error("No camera is defined. Will not render the scene.");
      }
      const camera = this._active_camera!;
      const gl = this._gl;

      this._back_buffer_render_target.bind();
      gl.clearColor(0.0, 0.0, 0.0, 1.0);
      gl.clearDepth(1.0);
      gl.disable(gl.DEPTH_TEST);
      gl.enable(gl.BLEND);
      
      gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

      for(const draw_command of draw_commands) {
        draw_command.draw(this._gl, camera.view, camera.proj)
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
