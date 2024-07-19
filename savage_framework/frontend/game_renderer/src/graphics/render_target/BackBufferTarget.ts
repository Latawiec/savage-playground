import { RenderTarget } from './RenderTarget'

export class BackBufferTarget implements RenderTarget {
    private _gl: WebGLRenderingContext;

    private _framebuffer: WebGLFramebuffer = 0;
    private _colorTexture: WebGLTexture = 0;
    private _depthBuffer: WebGLRenderbuffer = 0;

    private _width: number;
    private _height: number;

    constructor (gl: WebGLRenderingContext, width: number, height: number) {
      this._gl = gl
      this._width = width
      this._height = height

      this.recreateBuffer(width, height);
    }

    get width (): number { return this._width }
    get height (): number { return this._height }
    get colorTexture (): Readonly<WebGLTexture> { return this._colorTexture }

    bind () {
      this._gl.bindFramebuffer(this._gl.FRAMEBUFFER, this._framebuffer)
      this._gl.viewport(0, 0, this._width, this._height)
    }

    resize(width: number, height: number): void {
      this.recreateBuffer(width, height);
    }

    // private
    recreateBuffer(new_width: number, new_height: number) {
      const gl = this._gl;

      if (this._framebuffer) gl.deleteFramebuffer(this._framebuffer);
      if (this._depthBuffer) gl.deleteRenderbuffer(this._depthBuffer);
      if (this._colorTexture) gl.deleteTexture(this._colorTexture);

      const texture = gl.createTexture();
      const depthBuffer = gl.createRenderbuffer();
      const frameBuffer = gl.createFramebuffer();

      if (!texture) { throw new Error('Couldn\'t create texture') }
      if (!depthBuffer) { throw new Error('Couldn\'t create render buffer') }
      if (!frameBuffer) { throw new Error('Couldn\'t create frame buffer') }

      this._colorTexture = texture
      gl.bindTexture(gl.TEXTURE_2D, this.colorTexture)
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, new_width, new_height, 0, gl.RGBA, gl.UNSIGNED_BYTE, null)
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST)
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST)
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)

      this._depthBuffer = depthBuffer
      gl.bindRenderbuffer(gl.RENDERBUFFER, this._depthBuffer)
      gl.renderbufferStorage(gl.RENDERBUFFER, gl.DEPTH_COMPONENT16, new_width, new_height)

      this._framebuffer = frameBuffer
      gl.bindFramebuffer(gl.FRAMEBUFFER, this._framebuffer)
      gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, this.colorTexture, 0)
      gl.framebufferRenderbuffer(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.RENDERBUFFER, this._depthBuffer)

      gl.bindFramebuffer(gl.FRAMEBUFFER, null)

      this._width = new_width;
      this._height = new_height;
    }
}
