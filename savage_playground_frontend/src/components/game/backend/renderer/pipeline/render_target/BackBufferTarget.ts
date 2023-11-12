import { RenderTarget } from "./RenderTarget";


export class BackBufferTarget implements RenderTarget {
    private _gl: WebGLRenderingContext;
    
    private _framebuffer: WebGLFramebuffer;
    private _color_texture: WebGLTexture;
    private _depth_buffer: WebGLRenderbuffer;

    private _width: number;
    private _height: number;

    constructor(gl: WebGLRenderingContext, width: number, height: number) {
        this._gl = gl;
        this._width = width;
        this._height = height;

        this._color_texture = gl.createTexture()!;
        gl.bindTexture(gl.TEXTURE_2D, this.color_texture);
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, width, height, 0, gl.RGBA, gl.UNSIGNED_BYTE, null)!;
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

        this._depth_buffer = gl.createRenderbuffer()!;
        gl.bindRenderbuffer(gl.RENDERBUFFER, this._depth_buffer);
        gl.renderbufferStorage(gl.RENDERBUFFER, gl.DEPTH_COMPONENT16, width, height);

        this._framebuffer = gl.createFramebuffer()!;
        gl.bindFramebuffer(gl.FRAMEBUFFER, this._framebuffer);
        gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, this.color_texture, 0);
        gl.framebufferRenderbuffer(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.RENDERBUFFER, this._depth_buffer);

        gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    }

    get width(): number { return this._width; }
    get height(): number { return this._height; }
    get color_texture(): Readonly<WebGLTexture> { return this._color_texture; }

    bind() {
        this._gl.bindFramebuffer(this._gl.FRAMEBUFFER, this._framebuffer);
        this._gl.viewport(0, 0, this._width, this._height);
    }
}