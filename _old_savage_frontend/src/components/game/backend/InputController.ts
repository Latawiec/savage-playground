
export class InputController {
    private _inputState: Map<number, boolean>
    private _keyBinding: Map<string, number>

    constructor (document: Document, keyBinding: Map<string, number>) {
      this._inputState = new Map()
      this._keyBinding = keyBinding

      document.addEventListener('keyup', this.keyPressCallback)
      document.addEventListener('keydown', this.keyReleaseCallback)
    }

    set keyBinding (keyBinding: Map<string, number>) {
      this._keyBinding = keyBinding
    }

    get keyBinding (): Readonly<Map<string, number>> {
      return this._keyBinding
    }

    get inputState (): Readonly<Map<number, boolean>> {
      return this._inputState
    }

    private keyPressCallback (e: KeyboardEvent) {
      const keyCode = this._keyBinding.get(e.code)
      if (!keyCode) {
        // Unknown binding.
        return
      }
      this._inputState.set(keyCode, true)
    }

    private keyReleaseCallback (e: KeyboardEvent) {
      const keyCode = this._keyBinding.get(e.code)
      if (!keyCode) {
        // Unknown binding.
        return
      }
      this._inputState.set(keyCode, false)
    }
}
