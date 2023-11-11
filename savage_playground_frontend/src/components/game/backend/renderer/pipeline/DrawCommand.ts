import { mat4 } from "gl-matrix";
import { ShaderProgram } from "../gl_resource/ProgramStorage";
import { Mesh } from "../gl_resource/MeshStorage";
import { Texture } from "../gl_resource/TextureStorage";

interface IDrawCommand {
    draw(view: mat4, projection: mat4): void;
    get zorder(): number,
}


class DrawCommand implements IDrawCommand {

    private gl: WebGLRenderingContext;
    private program: ShaderProgram;
    private mesh: Mesh;
    private textures: Map<number, Texture>;
    private uniform_attrs: Record<string, any>;
    private vertex_attrs: Record<string, string>;
    private layer: number;
    private billboard: boolean;

    constructor(
        gl: WebGLRenderingContext,
        program: ShaderProgram,
        mesh: Mesh,
        textures: Map<number, Texture>,
        uniform_attrs: Record<string, any>,
        vertex_attrs: Record<string, any>,
        layer: number,
        billboard: boolean
    ) {
        this.gl = gl;
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

    draw(view: mat4, projection: mat4): void {
        throw new Error("Method not implemented.");
    }

// private

    private prepare_vertex_attributes(): void {
        const gl = this.gl;

        // Required attributes (vertices, indices)
        const vertex_position_attrib_loc = gl.getAttribLocation(this.program.glShaderProgram, this.vertex_attrs.vertices);
        gl.bindBuffer(gl.ARRAY_BUFFER, this.mesh.glVertexBuffer);
        gl.vertexAttribPointer(vertex_position_attrib_loc, 3, gl.FLOAT, false, 0, 0);
        gl.enableVertexAttribArray(vertex_position_attrib_loc);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.mesh.glIndexBuffer);

        // Optional attributes
        for (const attrib in this.vertex_attrs) {
            // So I think this works as pairs of [mesh_defined_name, shader_defined_name]
            const mesh_named_buffer = this.mesh.get_named_gl_buffer(attrib);
            const vertex_position_attrib_loc = gl.getAttribLocation(this.program.glShaderProgram, this.vertex_attrs[attrib]);

            if (mesh_named_buffer === undefined) {
                console.error(`Mesh doesn't have required named buffer: ${attrib}. Vertex attribute couldn't be set.`);
                continue;
            }

            if (vertex_position_attrib_loc === null) {
                console.error(`Couldn't locate ${this.vertex_attrs[attrib]} in shader program. Vertex attribute couldn't be set.`);
                continue;
            }

            const gl_type = mesh_named_buffer!.gl_type;
            const gl_buffer = mesh_named_buffer!.gl_buffer;
            const size = mesh_named_buffer!.size;

            gl.bindBuffer(gl.ARRAY_BUFFER, gl_buffer);
            gl.vertexAttribPointer(vertex_position_attrib_loc, size, gl_type, false, 0, 0);
        }
    }


}