import { EventEmitter } from "stream";

export class GameCommunication extends EventEmitter {
    private _websocket: WebSocket;

    constructor(gameServerAddress: string) {
        super();
        this._websocket = new WebSocket(gameServerAddress);
    }

    // !!! OLD CODE. Remove. But first find replacement for this.
    // Client Events:
    // send(event: 'input', data: ClientInputEvent): this;
    // send(event: string | symbol, data: ClientEvent): this {
    //     this._websocket.send(JSON.stringify(data));
    //     return this;
    // }

    // // Server Events:
    // on(event: 'gameConfig', cb: (data: GameConfigEvent) => void): this;
    // on(event: 'gameUpdate', cb: (data: GameUpdateEvent) => void): this;
    // on(event: string | symbol, cb: (...args: any[])=>void): this {
    //     return super.on(event, cb);
    // }

    // addListener(event: 'gameConfig', cb: (data: GameConfigEvent) => void): this;
    // addListener(event: 'gameUpdate', cb: (data: GameUpdateEvent) => void): this;
    // addListener(event: string | symbol, cb: (...args: any[]) => void): this {
    //     return super.on(event, cb);
    // }
}