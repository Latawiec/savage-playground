import { AssetStorage } from "../AssetStorage";

type NamedBufferElementType = 'u8' | 'u16' | 'u32' | 'i8' | 'i16' | 'i32' | 'f32';
type NamedBufferElementSize = 1 | 2 | 3 | 4;

export interface MeshNamedBuffer {
    size: NamedBufferElementSize;
    gl_type: GLint,
    gl_buffer: WebGLBuffer,
}

export class Mesh {
    private _elementsCount: number;
    private _vertexBuffer: WebGLBuffer;
    private _indexBuffer: WebGLBuffer;

    private _named_buffers = new Map<string, MeshNamedBuffer>();

    constructor(
        gl: WebGLRenderingContext,
        vertices: Float32Array,
        indices: Uint16Array
    ) {
        this._vertexBuffer = gl.createBuffer()!;
        this._indexBuffer = gl.createBuffer()!;

        gl.bindBuffer(gl.ARRAY_BUFFER, this._vertexBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);

        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this._indexBuffer);
        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, indices, gl.STATIC_DRAW);

        this._elementsCount = indices.length;
    }

    set_named_gl_buffer(gl: WebGLRenderingContext, buffer_name: string, gl_type: GLint, size: NamedBufferElementSize, data: ArrayBufferView) {
        let named_buffer = gl.createBuffer()!;

        gl.bindBuffer(gl.ARRAY_BUFFER, named_buffer);
        gl.bufferData(gl.ARRAY_BUFFER, data, gl.STATIC_DRAW);

        const mesh_named_buffer: MeshNamedBuffer = {
            size: size,
            gl_type: gl_type,
            gl_buffer: named_buffer
        };

        this._named_buffers.set(buffer_name, mesh_named_buffer);
    }

    get_named_gl_buffer(buffer_name: string): Readonly<MeshNamedBuffer> | undefined {
        return this._named_buffers.get(buffer_name);
    }

    get glVertexBuffer(): Readonly<WebGLBuffer> { return this._vertexBuffer; }
    get glIndexBuffer(): Readonly<WebGLBuffer> { return this._indexBuffer; }
    get elementsCount(): Readonly<number> { return this._elementsCount; }
}

// Json mesh 
interface NamedBufferJSON {
    type: NamedBufferElementType,
    size: NamedBufferElementSize,
    data: Array<number>,
}

interface MeshJSON {
    vertices: Float32Array;
    indices: Uint16Array;
    named_buffers?: Map<string, NamedBufferJSON>;
}

export class MeshStorage {
    private _meshCache = new Map<string, Mesh>;

    private _assetStorage: AssetStorage;
    private _gl: WebGLRenderingContext;

    constructor(gl: WebGLRenderingContext, assetStorage: AssetStorage) {
        this._assetStorage = assetStorage;
        this._gl = gl;
    }

    read(assetPath: string): Promise<Mesh> {
        return new Promise(async (resolve, reject) => {
            if (!this._meshCache.has(assetPath)) {
                try {
                    const json_mesh = JSON.parse(this._assetStorage.fs.readFileSync(assetPath).toString()) as MeshJSON;
                    const mesh = new Mesh(this._gl, Float32Array.from(json_mesh.vertices), Uint16Array.from(json_mesh.indices));

                    if (json_mesh.named_buffers !== undefined) {
                        for (const [buffer_name, buffer_content] of json_mesh.named_buffers) {
                            let buffer_data = MeshStorage.match_buffer_by_type(buffer_content.type, buffer_content.data);
                            let buffer_gl_type = MeshStorage.match_gl_type_by_type(buffer_content.type);

                            mesh.set_named_gl_buffer(this._gl, buffer_name, buffer_gl_type, buffer_content.size, buffer_data);
                        }
                    }

                    this._meshCache.set(assetPath, mesh);
                } catch (e) {
                    reject(e);
                }
            }
            resolve(this._meshCache.get(assetPath)!);
        })
    }

    private static match_buffer_by_type(type: NamedBufferElementType, buffer: Array<number>): ArrayBufferView {
        let buffer_data: ArrayBufferView;
        switch (type) {
            case 'u8': return Uint8Array.from(buffer);
            case 'u16': return Uint16Array.from(buffer);
            case 'u32': return Uint32Array.from(buffer);

            case 'i8': return Int8Array.from(buffer);
            case 'i16': return Int16Array.from(buffer);
            case 'i32': return Int32Array.from(buffer);

            case 'f32': return Float32Array.from(buffer);

            default:
                throw new Error(`Unknown named buffer type: ${type}`);
        }
        return buffer_data;
    }

    private static match_gl_type_by_type(type: NamedBufferElementType): GLint {
        switch (type) {
            case 'u8': return WebGLRenderingContext.UNSIGNED_BYTE;
            case 'u16': return WebGLRenderingContext.UNSIGNED_SHORT;
            case 'u32': return WebGLRenderingContext.UNSIGNED_INT;

            case 'i8': return WebGLRenderingContext.BYTE;
            case 'i16': return WebGLRenderingContext.SHORT;
            case 'i32': return WebGLRenderingContext.INT;

            case 'f32': return WebGLRenderingContext.FLOAT;
            default:
                throw new Error(`Unknown named buffer type: ${type}`);
        }
    }
}