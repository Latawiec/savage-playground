import { ConnectionController } from "./ConnectionController";
import { GameRendererProxy } from "./GameRendererProxy";
import { Renderer, Settings } from "./communication/GameMessage";

export interface GameCanvasInterface {
  get_html_canvas(): HTMLCanvasElement,
}

export interface GameOverlayInterface {

}

export class GameRuntime {
    private _canvas_interface: GameCanvasInterface;
    private _overlay_interface: GameOverlayInterface;

    private _connection_controller: ConnectionController;
    private _game_renderer: GameRendererProxy;

    constructor(game_host_address: URL, canvas_interface: GameCanvasInterface, overlay_interface: GameOverlayInterface) {
      this._canvas_interface = canvas_interface;
      this._overlay_interface = overlay_interface;

      this._connection_controller = new ConnectionController(game_host_address);
      this._game_renderer = new GameRendererProxy(canvas_interface.get_html_canvas());

      this._connection_controller.on('connected',       (e) => this.on_connected());
      this._connection_controller.on('disconnected',    (e) => this.on_disconnected());
      this._connection_controller.on('error',           (e) => this.on_error());
      this._connection_controller.on('renderer_update', (e) => this.on_renderer_update(e));
      this._connection_controller.on('settings_update', (e) => this.on_settings_update(e));
      this._connection_controller.on('ui_update',       (e) => this.on_ui_update(e));
    }

    private async on_connected() {

    }

    private async on_disconnected() {

    }

    private async on_error() {

    }

    private async on_renderer_update(snaposhot: Renderer.Snapshot) {
      await this._game_renderer.render_snapshot(snaposhot);
    }

    private async on_settings_update(snapshot: Settings.Snapshot) {

    }

    private async on_ui_update(snapshot: any) {
      
    }
}
