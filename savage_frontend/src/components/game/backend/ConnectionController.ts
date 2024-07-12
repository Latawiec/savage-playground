import { EventEmitter } from 'events'
import { GameMessage, Renderer, Settings } from './communication/GameMessage'
import { InputMessage } from './communication/ClientMessage'

interface ConnectionControllerEventMap {
    'connected': unknown;
    'disconnected': unknown;
    'error': unknown;
    'renderer_update': Renderer.Snapshot,
    'settings_update': Settings.Snapshot,
    'ui_update': unknown,
}

interface ConnectionControllerMessageMap {
    'input': InputMessage,
}

export class ConnectionController extends EventEmitter {
    private _websocket: WebSocket;

    constructor (
      gameHostAddress: URL
    ) {
      super()
      this._websocket = new WebSocket(gameHostAddress)
      this._websocket.addEventListener('open', (e) => { this.onOpen(e) })
      this._websocket.addEventListener('error', (e) => { this.onError(e) })
      this._websocket.addEventListener('close', (e) => { this.onClose(e) })
      this._websocket.addEventListener('message', (e) => { this.onMessage(e) })
    }

    send<K extends keyof ConnectionControllerMessageMap> (type: K, message: ConnectionControllerMessageMap[K]) {
      const jsonMessage = JSON.stringify(message)
      this._websocket.send(jsonMessage)
    }

    // Events
    on<K extends keyof ConnectionControllerEventMap> (event: K, listener: (ev: ConnectionControllerEventMap[K]) => unknown): this {
      return super.addListener(event, listener)
    }

    addListener<K extends keyof ConnectionControllerEventMap> (event: K, listener: (ev: ConnectionControllerEventMap[K]) => unknown): this {
      return super.addListener(event, listener)
    }

    // WebSocket Callbacks
    private onOpen (event: Event) {
      console.log('open: ', event)
      this.emit('connected')
    }

    private onClose (event: CloseEvent) {
      console.log('close: ', event)
      this.emit('disconnected')
    }

    private onError (event: Event) {
      console.log('error: ', event)
      this.emit('error', {})
    }

    private onMessage (event: MessageEvent) {
      console.log('message: ', event)
      try {
        const gameMessage = JSON.parse(event.data as string) as GameMessage

        if (gameMessage.renderer) {
          this.emit('renderer_update', gameMessage.renderer)
        }

        if (gameMessage.settings) {
          this.emit('settings_update', gameMessage.settings)
        }

        if (gameMessage.ui) {
          this.emit('ui_update', gameMessage.ui)
        }
      } catch (e) {
        console.log(`Error reading message: ${e}`)
      }
    }
}
