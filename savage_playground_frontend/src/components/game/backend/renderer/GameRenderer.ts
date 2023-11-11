
class GameRenderer {
    private _game_canvas: HTMLCanvasElement;

    constructor (
      game_canvas: HTMLCanvasElement
    ) {
      this._game_canvas = game_canvas
    }

    private gl_context_setup(gl: WebGLRenderingContext){
      gl.enable(gl.DEPTH_TEST);
      gl.enable(gl.BLEND);
      gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    }
}
