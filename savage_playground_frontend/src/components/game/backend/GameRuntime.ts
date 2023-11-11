
export class GameRuntime {
    private _game_canvas: HTMLElement;
    private _game_overlay: HTMLElement;

    constructor (
      game_canvas: HTMLElement,
      game_overlay: HTMLElement
    ) {
      this._game_canvas = game_canvas
      this._game_overlay = game_overlay
    }
}
