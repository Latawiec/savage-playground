import { CommitedResourceStorage } from "../../gl_resource/CommitedResourceStorage";
import { ShaderProgram } from "../../gl_resource/ProgramStorage";
import { IDrawCommand } from "./IDrawCommand";
import { Mesh } from "../../gl_resource/MeshStorage";

import screen_space_rect_src from '../../../assets/local_assets/mesh/screen_space_rect.json'
import blit_texture_vs_src from '../../../assets/local_assets/shader/blit_texture.vs.glsl'
import blit_texture_ps_src from '../../../assets/local_assets/shader/blit_texture.ps.glsl'

export class PresentDrawCommand implements IDrawCommand {

    private _program: ShaderProgram;
    private _mesh: Mesh;
    private _texture: WebGLTexture;

    private constructor(
        program: ShaderProgram,
        mesh: Mesh,
        texture: WebGLTexture,
    ) {
        this._program = program;
        this._mesh = mesh;
        this._texture = texture;
    }

    static from_resources(resources: CommitedResourceStorage, texture: WebGLTexture): Promise<PresentDrawCommand> {
        return new Promise(async (resolve, reject) => {
            const program = await resources.programs.read(blit_texture_vs_src, blit_texture_ps_src);
            const mesh = await resources.meshes.read(screen_space_rect_src);

            resolve(
                new PresentDrawCommand(
                    program,
                    mesh,
                    texture
                )
            );
        });
    }

    draw(gl: WebGLRenderingContext): void {
        this.prepare_program(gl);
        this.prepare_vertex_attributes(gl);
        this.prepare_textures(gl);
        this.prepare_blending(gl);
        this.final_draw(gl);
    }

    // private
    private final_draw(gl: WebGLRenderingContext): void {
        const elements_count = this._mesh.elementsCount;
        gl.drawElements(gl.TRIANGLES, elements_count, gl.UNSIGNED_SHORT, 0);
    }

    private prepare_program(gl: WebGLRenderingContext,): void {
        gl.useProgram(this._program.glShaderProgram);
    }

    private prepare_vertex_attributes(gl: WebGLRenderingContext): void {
        const vertex_position_attrib_loc = gl.getAttribLocation(this._program.glShaderProgram, 'aVertexPosition');
        gl.bindBuffer(gl.ARRAY_BUFFER, this._mesh.glVertexBuffer);
        gl.vertexAttribPointer(vertex_position_attrib_loc, 3, gl.FLOAT, false, 0, 0);
        gl.enableVertexAttribArray(vertex_position_attrib_loc);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this._mesh.glIndexBuffer);

        const uv_buffer = this._mesh.get_named_gl_buffer('uv')!;
        const uv_attrib_loc = gl.getAttribLocation(this._program.glShaderProgram, 'aUvCoord');
        gl.bindBuffer(gl.ARRAY_BUFFER, uv_buffer.gl_buffer);
        gl.vertexAttribPointer(uv_attrib_loc, uv_buffer.size, uv_buffer.gl_type, uv_buffer.normalize, 0, 0);
        gl.enableVertexAttribArray(uv_attrib_loc);
    }

    private prepare_textures(gl: WebGLRenderingContext): void {
        gl.activeTexture(gl.TEXTURE0);
        gl.bindTexture(gl.TEXTURE_2D, this._texture);
    }

    private prepare_blending(gl: WebGLRenderingContext): void {
        gl.disable(gl.DEPTH_TEST);
        gl.disable(gl.BLEND);
    }
}