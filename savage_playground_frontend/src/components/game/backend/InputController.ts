

class InputController {
    private _input_state: Map<number, boolean>
    private _key_binding: Map<string, number>

    constructor(document: Document, key_binding: Map<string, number>) {
        this._input_state = new Map();
        this._key_binding = key_binding;

        document.addEventListener('keyup', this.keyPressCallback);
        document.addEventListener('keydown', this.keyReleaseCallback);
    }

    set keyBinding(key_binding: Map<string, number>) {
        this._key_binding = key_binding
    }

    get inputState(): Readonly<Map<number, boolean>> {
        return this._input_state
    }

    private keyPressCallback(e: KeyboardEvent) {
        if (!this._key_binding.has(e.code)) {
            // Unknown binding.
            return;
        }
        let keyCode = this._key_binding.get(e.code)!;
        this._input_state.set(keyCode, true);
    }

    private keyReleaseCallback(e: KeyboardEvent) {
        if (!this._key_binding.has(e.code)) {
            // Unknown binding.
            return;
        }
        let keyCode = this._key_binding.get(e.code)!;
        this._input_state.set(keyCode, false);
    }
}