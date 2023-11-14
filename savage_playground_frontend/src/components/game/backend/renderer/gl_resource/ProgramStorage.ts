import { AssetStorage } from '../../AssetStorage'
import { Shader, ShaderStorage, ShaderType } from './ShaderStorage'

export class ShaderProgram {
  private _gl: WebGLRenderingContext;
  private _shaderProgram: WebGLProgram;

  constructor(gl: WebGLRenderingContext,
    pixelShader: Shader,
    vertexShader: Shader
  ) {
    // assert(pixelShader.type === ShaderType.PIXEL)
    // assert(vertexShader.type === ShaderType.VERTEX)

    this._gl = gl
    this._shaderProgram = this.linkProgram(pixelShader.glShader, vertexShader.glShader)!
  }

  get glShaderProgram() {
    return this._shaderProgram
  }

  private linkProgram(ps: WebGLShader, vs: WebGLShader): WebGLProgram | undefined {
    const shaderProgram = this._gl.createProgram()!
    this._gl.attachShader(shaderProgram, vs)
    this._gl.attachShader(shaderProgram, ps)
    this._gl.linkProgram(shaderProgram)

    if (!this._gl.getProgramParameter(shaderProgram, this._gl.LINK_STATUS)) {
      alert('Failed to link program: ' + this._gl.getProgramInfoLog(shaderProgram))
      return undefined
    }

    return shaderProgram
  }
}

export class ProgramStorage {
  private _programCache = new Map<string, ShaderProgram>();

  private _shaderStorage: ShaderStorage;
  private _assetStorage: AssetStorage;
  private _gl: WebGLRenderingContext;

  constructor(gl: WebGLRenderingContext, assetStorage: AssetStorage) {
    this._assetStorage = assetStorage
    this._shaderStorage = new ShaderStorage(gl, assetStorage)
    this._gl = gl
  }

  write(vs_asset_path: string, vs_source_code: string, ps_asset_path: string, ps_source_code: string): ShaderProgram {
    const vertex_shader = this._shaderStorage.write(vs_asset_path, ShaderType.VERTEX, vs_source_code);
    const pixel_shader = this._shaderStorage.write(ps_asset_path, ShaderType.PIXEL, ps_source_code);

    const pair_key = this.programPairKey(vertex_shader, pixel_shader);
    if (this._programCache.has(pair_key)) {
      throw new Error(`Such program combo already exists: ${vs_asset_path} and ${ps_asset_path}`);
    }

    const shader_program = new ShaderProgram(this._gl, pixel_shader, vertex_shader);
    this._programCache.set(pair_key, shader_program);

    return shader_program;
  }

  read(vs_asset_path: string, ps_asset_path: string): Promise<ShaderProgram> {
    return new Promise(async (resolve, reject) => {
      try {
        const vertexShader = await this._shaderStorage.read(vs_asset_path, ShaderType.VERTEX);
        const pixelShader = await this._shaderStorage.read(ps_asset_path, ShaderType.PIXEL);
        const pairKey = this.programPairKey(vertexShader, pixelShader);

        if (!this._programCache.has(pairKey)) {
          const shaderProgram = new ShaderProgram(this._gl, pixelShader, vertexShader);
          this._programCache.set(pairKey, shaderProgram);
        }

        resolve(this._programCache.get(pairKey)!)
      } catch (e) {
        reject(e)
      }
    })
  }

  private programPairKey(vertexShader: Shader, pixelShader: Shader) {
    return vertexShader.id.toString() + ' ' + pixelShader.id.toString()
  }
}
