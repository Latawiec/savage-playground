import { EventEmitter } from 'events'
import { GameMessage } from './communication/GameMessage';
import { InputMessage } from './communication/ClientMessage';


interface ConnectionControllerEventMap {
    "connected": any;
    "disconnected": any;
    "error": any;
    "game_message": GameMessage;
}

interface ConnectionControllerMessageMap {
    "input": InputMessage,
}

export class ConnectionController extends EventEmitter {
    private _websocket: WebSocket;

    constructor(
        game_host_address: URL
    ) {
        super();
        this._websocket = new WebSocket(game_host_address);
        this._websocket.addEventListener('open', this.on_open);
        this._websocket.addEventListener('error', this.on_error);
        this._websocket.addEventListener('close', this.on_close);
        this._websocket.addEventListener('message', this.on_message);
    }

    send<K extends keyof ConnectionControllerMessageMap>(type: K, message: ConnectionControllerMessageMap[K]) {
        const json_message = JSON.stringify(message);
        this._websocket.send(json_message);
    }

    // Events
    on<K extends keyof ConnectionControllerEventMap>(event: K, listener: (ev: ConnectionControllerEventMap[K]) => any): this {
        return super.addListener(event, listener);
    }
    addListener<K extends keyof ConnectionControllerEventMap>(event: K, listener: (ev: ConnectionControllerEventMap[K]) => any): this {
        return super.addListener(event, listener);
    }

    // WebSocket Callbacks
    private on_open(event: Event) {
        this.emit('connected');
    }

    private on_close(event: CloseEvent) {
        this.emit('disconnected');
    }

    private on_error(event: Event) {
        this.emit('error', {});
    }

    private on_message(event: MessageEvent) {
        // todo parse message
        let game_message = JSON.parse(event.data as string) as GameMessage;
        this.emit('world_snapshot', game_message);
    }
}
