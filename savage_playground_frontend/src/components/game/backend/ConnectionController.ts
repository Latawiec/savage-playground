import { EventEmitter } from 'events'
import { GameMessage, Renderer, Settings } from './communication/GameMessage';
import { InputMessage } from './communication/ClientMessage';


interface ConnectionControllerEventMap {
    "connected": any;
    "disconnected": any;
    "error": any;
    "renderer_update": Renderer.Snapshot,
    "settings_update": Settings.Snapshot,
    "ui_update": any,
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
        this._websocket.addEventListener('open', (e) => { this.on_open(e) } );
        this._websocket.addEventListener('error', (e) => { this.on_error(e) });
        this._websocket.addEventListener('close', (e) => { this.on_close(e) });
        this._websocket.addEventListener('message', (e) => { this.on_message(e) });
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
        console.log("open: ", event);
        this.emit('connected');
    }

    private on_close(event: CloseEvent) {
        console.log("close: ", event);
        this.emit('disconnected');
    }

    private on_error(event: Event) {
        console.log("error: ", event);
        this.emit('error', {});
    }

    private on_message(event: MessageEvent) {
        console.log("message: ", event);
        try {
            let game_message = JSON.parse(event.data as string) as GameMessage;
            
            if (game_message.renderer !== undefined) {
                this.emit('renderer_update', game_message.renderer!);
            }

            if (game_message.settings !== undefined) {
                this.emit('settings_update', game_message.settings!);
            }

            if (game_message.ui !== undefined) {
                this.emit('ui_update', game_message.ui!);
            }
        } catch(e) {
            console.log(`Error reading message: ${e}`);
        }
    }
}
