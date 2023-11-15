import { ShaderProgram } from "../gl_resource/ProgramStorage";
import { Mesh } from "../gl_resource/MeshStorage";
import { Texture } from "../gl_resource/TextureStorage";
import { ShaderValueType } from "../../common/GLTypes";

export interface IDrawCommand {
    draw(gl: WebGLRenderingContext): void;
    get zorder(): number;
}

export class DrawCommand implements IDrawCommand {
    private program: ShaderProgram;
    private mesh: Mesh;
    private textures: Map<number, Texture> | undefined;
    private uniform_attrs: Map<ShaderValueType, Map<string, number | number[]>> | undefined;
    private vertex_attrs: Map<string, string>;
    private layer: number;
    private billboard: boolean;

    constructor(
        program: ShaderProgram,
        mesh: Mesh,
        textures: Map<number, Texture> | undefined,
        uniform_attrs: Map<ShaderValueType, Map<string, number | number[]>> | undefined,
        vertex_attrs: Map<string, string>,
        layer: number,
        billboard: boolean
    ) {
        this.program = program;
        this.mesh = mesh;
        this.textures = textures;
        this.uniform_attrs = uniform_attrs;
        this.vertex_attrs = vertex_attrs;
        this.layer = layer;
        this.billboard = billboard;
    }

// impl IDrawRequest
    get zorder(): number {
        return this.layer;
    }

    draw(gl: WebGLRenderingContext): void {
        this.prepare_program(gl);
    }

// private

    private prepare_program(gl: WebGLRenderingContext,): void {
        gl.useProgram(this.program.glShaderProgram);
    }

    private prepare_vertex_attributes(gl: WebGLRenderingContext): void {
        
        // Required attributes (vertices, indices)
        const vertex_position_attrib_loc = gl.getAttribLocation(this.program.glShaderProgram, this.vertex_attrs.get('vertices')!);
        gl.bindBuffer(gl.ARRAY_BUFFER, this.mesh.glVertexBuffer);
        gl.vertexAttribPointer(vertex_position_attrib_loc, 3, gl.FLOAT, false, 0, 0);
        gl.enableVertexAttribArray(vertex_position_attrib_loc);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.mesh.glIndexBuffer);

        // Optional attributes
        for (const [buffer_name, attrib_name] of this.vertex_attrs) {
            if (buffer_name === 'vertices') {
                // Special. For now.
                continue;
            }

            const mesh_named_buffer = this.mesh.get_named_gl_buffer(buffer_name);
            const vertex_position_attrib_loc = gl.getAttribLocation(this.program.glShaderProgram, attrib_name);

            if (mesh_named_buffer === undefined) {
                console.error(`Mesh doesn't have required named buffer: ${attrib_name}. Vertex attribute couldn't be set.`);
                continue;
            }

            if (vertex_position_attrib_loc === null) {
                console.error(`Couldn't locate ${mesh_named_buffer} in shader program. Vertex attribute couldn't be set.`);
                continue;
            }

            const gl_type = mesh_named_buffer!.gl_type;
            const gl_buffer = mesh_named_buffer!.gl_buffer;
            const size = mesh_named_buffer!.size;

            gl.bindBuffer(gl.ARRAY_BUFFER, gl_buffer);
            gl.vertexAttribPointer(vertex_position_attrib_loc, size, gl_type, false, 0, 0);
        }
    }
    
    private prepare_uniform_attributes(gl: WebGLRenderingContext): void {
        if (this.uniform_attrs === undefined) {
            return;
        }

        for (const [type, attributes] of this.uniform_attrs) {
            for (const [uniform_name, uniform_values] of attributes) {
                const uniform_location = gl.getUniformLocation(this.program.glShaderProgram, uniform_name);
                if (uniform_location === null) {
                    console.error(`Uniform attribute ${uniform_name} not found in shader program.`);
                    continue;
                }

                switch(type) {
                    case 'mat4': gl.uniformMatrix4fv(uniform_location, false, uniform_values as Array<number>); break;

                    case 'float': gl.uniform1f(uniform_location, uniform_values as number); break;
                    case 'vec2': gl.uniform2fv(uniform_location, uniform_values as Array<number>); break;
                    case 'vec3': gl.uniform3fv(uniform_location, uniform_values as Array<number>); break;
                    case 'vec4': gl.uniform4fv(uniform_location, uniform_values as Array<number>); break;

                    case 'int': gl.uniform1i(uniform_location, uniform_values as number); break;
                    case 'ivec2': gl.uniform2iv(uniform_location, uniform_values as Array<number>); break;
                    case 'ivec3': gl.uniform3iv(uniform_location, uniform_values as Array<number>); break;
                    case 'ivec4': gl.uniform4iv(uniform_location, uniform_values as Array<number>); break;

                    default: console.error(`Uniform type unknown: ${type}`);
                }
            }
        }
    }

    private prepare_textures(gl: WebGLRenderingContext): void {
        if (this.textures === undefined) {
            return;
        }

        for (const [texture_offset, texture] of this.textures) {
            gl.activeTexture(gl.TEXTURE0 + texture_offset);
            gl.bindTexture(gl.TEXTURE_2D, texture.glTexture);
        }
    }

    private prepare_blending(gl: WebGLRenderingContext): void {
        // Just use standard for now.
        //canvas.glContext.blendFunc(canvas.glContext.SRC_ALPHA, canvas.glContext.ONE); // Additive blending.
        gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA); // real transparency
    }

}