

class GameRuntime {

    private _websocket: WebSocket;
    private _game_canvas: HTMLElement;
    private _game_overlay: HTMLElement;

    constructor(
        game_host_address: URL,
        game_canvas: HTMLElement,
        game_overlay: HTMLElement,
    ){
        this._websocket = new WebSocket(game_host_address);
        this._game_canvas = game_canvas;
        this._game_overlay = game_overlay;
    }

    
}