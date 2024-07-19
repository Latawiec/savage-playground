import { IAssetStorage } from '../../asset_storage/IAssetStorage'

type NamedBufferElementType = 'u8' | 'u16' | 'u32' | 'i8' | 'i16' | 'i32' | 'f32';
type NamedBufferElementSize = 1 | 2 | 3 | 4;

export namespace JsonMesh {
    export interface NamedBufferJSON {
        type: NamedBufferElementType,
        size: NamedBufferElementSize,
        normalize?: boolean,
        data: Array<number>,
    }

    export interface MeshJSON {
        vertices: Float32Array;
        indices: Uint16Array;
        namedBuffers?: Record<string, NamedBufferJSON>;
    }

    export function matchBufferByType(type: NamedBufferElementType, buffer: Array<number>): ArrayBufferView {
        let bufferData: ArrayBufferView
        switch (type) {
            case 'u8': return Uint8Array.from(buffer)
            case 'u16': return Uint16Array.from(buffer)
            case 'u32':
                throw Error(`${type} only available in WebGL2`)
                return Uint32Array.from(buffer)

            case 'i8': return Int8Array.from(buffer)
            case 'i16': return Int16Array.from(buffer)
            case 'i32':
                throw Error(`${type} only available in WebGL2`)
                return Int32Array.from(buffer)

            case 'f32': return Float32Array.from(buffer)

            default:
                throw new Error(`Unknown named buffer type: ${type}`)
        }
        return bufferData
    }

    export function matchGlTypeByType(type: NamedBufferElementType): GLint {
        switch (type) {
            case 'u8': return WebGLRenderingContext.UNSIGNED_BYTE
            case 'u16': return WebGLRenderingContext.UNSIGNED_SHORT
            case 'u32':
                throw Error(`${type} only available in WebGL2`)
                return WebGLRenderingContext.UNSIGNED_INT // Only works in WebGL2

            case 'i8': return WebGLRenderingContext.BYTE
            case 'i16': return WebGLRenderingContext.SHORT
            case 'i32':
                throw Error(`${type} only available in WebGL2`)
                return WebGLRenderingContext.INT // Only works in WebGL2

            case 'f32': return WebGLRenderingContext.FLOAT
            default:
                throw new Error(`Unknown named buffer type: ${type}`)
        }
    }
}

export interface MeshNamedBuffer {
    size: NamedBufferElementSize;
    normalize: boolean,
    glType: GLint,
    glBuffer: WebGLBuffer,
}

export class Mesh {
    private _elementsCount: number;
    private _vertexBuffer: WebGLBuffer;
    private _indexBuffer: WebGLBuffer;

    private _namedBuffers = new Map<string, MeshNamedBuffer>();

    constructor(
        gl: WebGLRenderingContext,
        vertices: Float32Array,
        indices: Uint16Array
    ) {
        const vertexBuffer = gl.createBuffer()
        const indexBuffer = gl.createBuffer()

        if (!vertexBuffer || !indexBuffer) {
            throw new Error('Failed to create vertex and index buffers.')
        }

        this._vertexBuffer = vertexBuffer
        this._indexBuffer = indexBuffer

        gl.bindBuffer(gl.ARRAY_BUFFER, this._vertexBuffer)
        gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW)

        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this._indexBuffer)
        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, indices, gl.STATIC_DRAW)

        this._elementsCount = indices.length
    }

    static fromJson(
        gl: WebGLRenderingContext,
        meshData: JsonMesh.MeshJSON
    ): Mesh {
        const mesh = new Mesh(gl, Float32Array.from(meshData.vertices), Uint16Array.from(meshData.indices))
        if (meshData.namedBuffers !== undefined) {
            for (const bufferBame in meshData.namedBuffers) {
                const bufferContent = meshData.namedBuffers[bufferBame]
                const bufferData = JsonMesh.matchBufferByType(bufferContent.type, bufferContent.data)
                const bufferGlType = JsonMesh.matchGlTypeByType(bufferContent.type)
                const normalize = bufferContent.normalize ? bufferContent.normalize : false

                mesh.setNamedGlBuffer(
                    gl,
                    bufferBame,
                    bufferGlType,
                    bufferContent.size,
                    normalize,
                    bufferData
                )
            }
        }
        return mesh
    }

    setNamedGlBuffer(gl: WebGLRenderingContext, bufferName: string, glType: GLint, size: NamedBufferElementSize, normalize: boolean, data: ArrayBufferView) {
        const namedBuffer = gl.createBuffer()

        if (!namedBuffer) {
            throw new Error('Failed to create named buffer')
        }

        gl.bindBuffer(gl.ARRAY_BUFFER, namedBuffer)
        gl.bufferData(gl.ARRAY_BUFFER, data, gl.STATIC_DRAW)

        const meshNamedBuffer: MeshNamedBuffer = {
            size: size,
            normalize: normalize,
            glType: glType,
            glBuffer: namedBuffer
        }

        this._namedBuffers.set(bufferName, meshNamedBuffer)
    }

    getNamedGlBuffer(bufferName: string): Readonly<MeshNamedBuffer> | undefined {
        return this._namedBuffers.get(bufferName)
    }

    get glVertexBuffer(): Readonly<WebGLBuffer> { return this._vertexBuffer }
    get glIndexBuffer(): Readonly<WebGLBuffer> { return this._indexBuffer }
    get elementsCount(): Readonly<number> { return this._elementsCount }
}

export class MeshStorage {
    private _meshCache = new Map<string, Mesh>();

    private _assetStorage: IAssetStorage;
    private _gl: WebGLRenderingContext;

    constructor(gl: WebGLRenderingContext, assetStorage: IAssetStorage) {
        this._assetStorage = assetStorage
        this._gl = gl
    }

    write(assetPath: string, jsonMesh: JsonMesh.MeshJSON): Mesh {
        if (this._meshCache.has(assetPath)) {
            throw new Error(`Asset with path ${assetPath} already exists in ${MeshStorage.name}`)
        }
        const mesh = Mesh.fromJson(this._gl, jsonMesh)
        this._meshCache.set(assetPath, mesh)
        return mesh
    }

    async read(assetPath: string): Promise<Mesh> {
        if (!this._meshCache.has(assetPath)) {
            try {
                const jsonMesh = JSON.parse((await this._assetStorage.readFile(assetPath)).toString()) as JsonMesh.MeshJSON
                this.write(assetPath, jsonMesh)
            } catch (e) {
                throw new Error(`Couldn't retrieve mesh ${assetPath}: ${e}`)
            }
        }
        const mesh = this._meshCache.get(assetPath)
        if (!mesh) {
            throw new Error(`Failed to retrieve cached mesh: ${assetPath}`)
        }
        return mesh
    }
}
