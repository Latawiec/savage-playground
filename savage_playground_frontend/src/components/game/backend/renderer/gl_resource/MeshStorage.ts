import { IAssetStorage } from "../../assets/IAssetStorage";

type NamedBufferElementType = 'u8' | 'u16' | 'u32' | 'i8' | 'i16' | 'i32' | 'f32';
type NamedBufferElementSize = 1 | 2 | 3 | 4;

export interface MeshNamedBuffer {
    size: NamedBufferElementSize;
    normalize: boolean,
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

    static from_json(
        gl: WebGLRenderingContext,
        mesh_data: json_mesh.MeshJSON
    ): Mesh {
        const mesh = new Mesh(gl, Float32Array.from(mesh_data.vertices), Uint16Array.from(mesh_data.indices));
        if (mesh_data.named_buffers !== undefined) {
            for (const buffer_name in mesh_data.named_buffers) {
                const buffer_content = mesh_data.named_buffers[buffer_name];
                let buffer_data = json_mesh.match_buffer_by_type(buffer_content.type, buffer_content.data);
                let buffer_gl_type = json_mesh.match_gl_type_by_type(buffer_content.type);
                let normalize = buffer_content.normalize !== undefined ? buffer_content.normalize! : false;

                mesh.set_named_gl_buffer(
                    gl,
                    buffer_name,
                    buffer_gl_type,
                    buffer_content.size,
                    normalize,
                    buffer_data
                );
            }
        }
        return mesh;
    }

    set_named_gl_buffer(gl: WebGLRenderingContext, buffer_name: string, gl_type: GLint, size: NamedBufferElementSize, normalize: boolean, data: ArrayBufferView) {
        let named_buffer = gl.createBuffer()!;

        gl.bindBuffer(gl.ARRAY_BUFFER, named_buffer);
        gl.bufferData(gl.ARRAY_BUFFER, data, gl.STATIC_DRAW);

        const mesh_named_buffer: MeshNamedBuffer = {
            size: size,
            normalize: normalize,
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

export class MeshStorage {
    private _meshCache = new Map<string, Mesh>();

    private _assetStorage: IAssetStorage;
    private _gl: WebGLRenderingContext;

    constructor(gl: WebGLRenderingContext, assetStorage: IAssetStorage) {
        this._assetStorage = assetStorage;
        this._gl = gl;
    }

    write(asset_path:string, json_mesh: json_mesh.MeshJSON): Mesh {
        if (this._meshCache.has(asset_path)) {
            throw new Error(`Asset with path ${asset_path} already exists in ${MeshStorage.name}`);
        }
        const mesh = Mesh.from_json(this._gl, json_mesh);
        this._meshCache.set(asset_path, mesh);
        return mesh;
    }

    read(assetPath: string): Promise<Mesh> {
        return new Promise(async (resolve, reject) => {
            if (!this._meshCache.has(assetPath)) {
                try {
                    const json_mesh = JSON.parse(await this._assetStorage.read_file(assetPath).toString()) as json_mesh.MeshJSON;
                    this.write(assetPath, json_mesh);
                } catch (e) {
                    reject(e);
                }
            }
            resolve(this._meshCache.get(assetPath)!);
        })
    }
}

module json_mesh {
    export interface NamedBufferJSON {
        type: NamedBufferElementType,
        size: NamedBufferElementSize,
        normalize?: boolean,
        data: Array<number>,
    }

    export interface MeshJSON {
        vertices: Float32Array;
        indices: Uint16Array;
        named_buffers?: Record<string, NamedBufferJSON>;
    }

    export function match_buffer_by_type(type: NamedBufferElementType, buffer: Array<number>): ArrayBufferView {
        let buffer_data: ArrayBufferView;
        switch (type) {
            case 'u8': return Uint8Array.from(buffer);
            case 'u16': return Uint16Array.from(buffer);
            case 'u32': 
                throw Error(`${type} only available in WebGL2`);
                return Uint32Array.from(buffer);

            case 'i8': return Int8Array.from(buffer);
            case 'i16': return Int16Array.from(buffer);
            case 'i32':
                throw Error(`${type} only available in WebGL2`);
                return Int32Array.from(buffer);

            case 'f32': return Float32Array.from(buffer);

            default:
                throw new Error(`Unknown named buffer type: ${type}`);
        }
        return buffer_data;
    }

    export function match_gl_type_by_type(type: NamedBufferElementType): GLint {
        switch (type) {
            case 'u8': return WebGLRenderingContext.UNSIGNED_BYTE;
            case 'u16': return WebGLRenderingContext.UNSIGNED_SHORT;
            case 'u32': 
                throw Error(`${type} only available in WebGL2`);
                return WebGLRenderingContext.UNSIGNED_INT;  // Only works in WebGL2

            case 'i8': return WebGLRenderingContext.BYTE;
            case 'i16': return WebGLRenderingContext.SHORT;
            case 'i32': 
                throw Error(`${type} only available in WebGL2`);
                return WebGLRenderingContext.INT;   // Only works in WebGL2

            case 'f32': return WebGLRenderingContext.FLOAT;
            default:
                throw new Error(`Unknown named buffer type: ${type}`);
        }
    }
}