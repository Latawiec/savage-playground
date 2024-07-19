import { RenderTarget } from './RenderTarget'

export class MainTarget implements RenderTarget {
    private _gl: WebGLRenderingContext;

    private _width: number;
    private _height: number;

    constructor(gl: WebGLRenderingContext, width: number, height: number) {
        this._gl = gl
        this._width = width
        this._height = height
    }

    bind() {
        this._gl.bindFramebuffer(this._gl.FRAMEBUFFER, null)
        this._gl.viewport(0, 0, this._width, this._height)
    }

    resize(width: number, height: number): void {
        this._width = width;
        this._height = height;
    }

    get width(): number { return this._width }
    get height(): number { return this._height }
}
