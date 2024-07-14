import { IAssetStorage } from '../../assets/IAssetStorage'
import { UuidGenerator } from '../../common/UuidGenerator'

export enum ShaderType {
    PIXEL = WebGLRenderingContext.FRAGMENT_SHADER,
    VERTEX = WebGL2RenderingContext.VERTEX_SHADER
}

export class Shader {
    private _gl: WebGLRenderingContext;
    private _id: number;
    private _type: ShaderType;
    private _shader: WebGLShader;

    constructor (gl: WebGLRenderingContext, id: number, type: ShaderType, source: Readonly<string>) {
      this._gl = gl
      this._id = id
      this._type = type
      this._shader = this.compileShader(this._gl, source)
    }

    release () {
      this._gl.deleteShader(this._shader)
    }

    get type () {
      return this._type
    }

    get id () {
      return this._id
    }

    get glShader () {
      return this._shader
    }

    private compileShader (gl: WebGLRenderingContext, source: Readonly<string>): WebGLShader {
      const shader = gl.createShader(this._type)
      if (!shader) {
        throw new Error('Failed to create shader.')
      }

      gl.shaderSource(shader, source)
      gl.compileShader(shader)

      if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        gl.deleteShader(shader)
        throw new Error(`Failed to compile shader: ${gl.getShaderInfoLog(shader)}`)
      }
      return shader
    }
}

export class ShaderStorage {
    private _shaderCache = new Map<string, Shader>();
    private _shaderIdGenerator = new UuidGenerator();

    private _assetStorage: IAssetStorage;
    private _gl: WebGLRenderingContext;

    constructor (gl: WebGLRenderingContext, assetStorage: IAssetStorage) {
      this._assetStorage = assetStorage
      this._gl = gl
    }

    write (assetPath: string, type: ShaderType, source: Readonly<string>): Shader {
      if (this._shaderCache.has(assetPath)) {
        throw new Error(`Asset with path ${assetPath} already exists in ${ShaderStorage.name}`)
      }
      const shader = new Shader(this._gl, this._shaderIdGenerator.getNext(), type, source)
      this._shaderCache.set(assetPath, shader)

      return shader
    }

    async read (assetPath: string, type: ShaderType): Promise<Shader> {
      const shader = this._shaderCache.get(assetPath)
      if (shader) {
        return shader
      }

      try {
        const shaderSource = (await this._assetStorage.readFile(assetPath)).toString()
        const shader = this.write(assetPath, type, shaderSource)
        return shader
      } catch (e) {
        throw new Error(`Couldn't retrieve shader ${assetPath}: ${e}`)
      }
    }
}
