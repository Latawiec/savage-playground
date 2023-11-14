import { ConnectionController } from "./ConnectionController";
import { GameRenderer } from "./renderer/GameRenderer";


export class GameRuntime {
    private _connection_controller: ConnectionController;

    constructor (game_host_address: URL) {
      this._connection_controller = new ConnectionController(game_host_address);
      this._game_renderer = new GameRenderer()
    }
}
