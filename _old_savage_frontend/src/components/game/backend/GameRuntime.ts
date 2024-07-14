import { ConnectionController } from './ConnectionController'
import { GameRendererProxy } from './GameRendererProxy'
import { Renderer, Settings } from './communication/GameMessage'

export interface GameCanvasInterface {
  getHtmlCanvas(): HTMLCanvasElement,
}

export interface GameOverlayInterface {
  data(): object, // Just placeholder.
}

export class GameRuntime {
    private _canvasInterface: GameCanvasInterface;
    private _overlayInterface: GameOverlayInterface;

    private _connectionController: ConnectionController;
    private _gameRenderer: GameRendererProxy;

    constructor (gameHostAddress: URL, canvasInterface: GameCanvasInterface, overlayInterface: GameOverlayInterface) {
      this._canvasInterface = canvasInterface
      this._overlayInterface = overlayInterface

      this._connectionController = new ConnectionController(gameHostAddress)
      this._gameRenderer = new GameRendererProxy(canvasInterface.getHtmlCanvas())

      this._connectionController.on('connected', (e) => this.onConnected(e))
      this._connectionController.on('disconnected', (e) => this.onDisconnected(e))
      this._connectionController.on('error', (e) => this.onError(e))
      this._connectionController.on('renderer_update', (e) => this.onRendererUpdate(e))
      this._connectionController.on('settings_update', (e) => this.onSettingsUpdate(e))
      this._connectionController.on('ui_update', (e) => this.onUiUpdate(e))
    }

    private async onConnected (_: unknown) {
      console.log('Connected')
    }

    private async onDisconnected (_: unknown) {
      console.log('Disconnected')
    }

    private async onError (_: unknown) {
      console.log('Error')
    }

    private async onRendererUpdate (snaposhot: Renderer.Snapshot) {
      await this._gameRenderer.renderSnapshot(snaposhot)
    }

    private async onSettingsUpdate (snapshot: Settings.Snapshot) {
      if (snapshot.assets && snapshot.assets.assetsPackagePath) {
        await this._gameRenderer.loadAssetsPackage(snapshot.assets.assetsPackagePath)
      }
    }

    private async onUiUpdate (_snapshot: unknown) {
      console.log('UI Update')
    }
}
