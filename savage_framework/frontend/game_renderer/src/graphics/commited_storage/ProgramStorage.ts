import { IAssetStorage } from '../../asset_storage/IAssetStorage'
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
        this._shaderProgram = this.linkProgram(pixelShader.glShader, vertexShader.glShader)
    }

    get glShaderProgram() {
        return this._shaderProgram
    }

    private linkProgram(ps: WebGLShader, vs: WebGLShader): WebGLProgram {
        const shaderProgram = this._gl.createProgram()

        if (!shaderProgram) {
            throw new Error('Failed to create program')
        }

        this._gl.attachShader(shaderProgram, vs)
        this._gl.attachShader(shaderProgram, ps)
        this._gl.linkProgram(shaderProgram)

        if (!this._gl.getProgramParameter(shaderProgram, this._gl.LINK_STATUS)) {
            throw new Error(`Failed to link program: ${this._gl.getProgramInfoLog(shaderProgram)}`)
        }

        return shaderProgram
    }
}

export class ProgramStorage {
    private _programCache = new Map<string, ShaderProgram>();

    private _shaderStorage: ShaderStorage;
    private _gl: WebGLRenderingContext;

    constructor(gl: WebGLRenderingContext, assetStorage: IAssetStorage) {
        this._shaderStorage = new ShaderStorage(gl, assetStorage)
        this._gl = gl
    }

    write(vsAssetPath: string, vsSourceCode: string, psAssetPath: string, psSourceCode: string): ShaderProgram {
        const vertexShader = this._shaderStorage.write(vsAssetPath, ShaderType.VERTEX, vsSourceCode)
        const pixelShader = this._shaderStorage.write(psAssetPath, ShaderType.PIXEL, psSourceCode)

        const pairKey = this.programPairKey(vertexShader, pixelShader)
        if (this._programCache.has(pairKey)) {
            throw new Error(`Such program combo already exists: ${vsAssetPath} and ${psAssetPath}`)
        }

        const shaderProgram = new ShaderProgram(this._gl, pixelShader, vertexShader)
        this._programCache.set(pairKey, shaderProgram)

        return shaderProgram
    }

    async read(vsAssetPath: string, psAssetPath: string): Promise<ShaderProgram> {
        try {
            const vertexShader = await this._shaderStorage.read(vsAssetPath, ShaderType.VERTEX)
            const pixelShader = await this._shaderStorage.read(psAssetPath, ShaderType.PIXEL)
            const pairKey = this.programPairKey(vertexShader, pixelShader)

            const program = this._programCache.get(pairKey)
            if (!program) {
                const shaderProgram = new ShaderProgram(this._gl, pixelShader, vertexShader)
                this._programCache.set(pairKey, shaderProgram)
                return shaderProgram
            }
            return program
        } catch (e) {
            throw new Error(`Couldn't retrieve program [${vsAssetPath}, ${psAssetPath}]: ${e}`)
        }
    }

    private programPairKey(vertexShader: Shader, pixelShader: Shader) {
        return vertexShader.id.toString() + ' ' + pixelShader.id.toString()
    }
}
