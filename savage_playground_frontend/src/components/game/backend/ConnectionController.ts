
export class ConnectionController {
    private _websocket: WebSocket;

    constructor (
      game_host_address: URL
    ) {
      this._websocket = new WebSocket(game_host_address)
    }
}
