import { CommitedResourceStorage } from '../commited_storage/CommitedResourceStorage'
import { ShaderProgram } from '../commited_storage/ProgramStorage'
import { IDrawCommand } from './IDrawCommand'
import { Mesh } from '../commited_storage/MeshStorage'
import { MESH_SCREEN_SPACE_RECT_RESOURCE_PATH, SHADER_BLIT_TEXTURE_PS_RESOURCE_PATH, SHADER_BLIT_TEXTURE_VS_RESOURCE_PATH } from '../assets/LocalAssets';


export class PresentDrawCommand implements IDrawCommand {
    private _program: ShaderProgram;
    private _mesh: Mesh;
    private _texture: WebGLTexture;

    private constructor(
        program: ShaderProgram,
        mesh: Mesh,
        texture: WebGLTexture
    ) {
        this._program = program
        this._mesh = mesh
        this._texture = texture
    }

    static async fromResources(resources: CommitedResourceStorage, texture: WebGLTexture): Promise<PresentDrawCommand> {
        try {
            const program = await resources.programs.read(SHADER_BLIT_TEXTURE_VS_RESOURCE_PATH, SHADER_BLIT_TEXTURE_PS_RESOURCE_PATH)
            const mesh = await resources.meshes.read(MESH_SCREEN_SPACE_RECT_RESOURCE_PATH)

            return new PresentDrawCommand(
                program,
                mesh,
                texture
            )
        } catch (e) {
            throw new Error(`Couldn't create PresentDrawCommand: ${e}`)
        }
    }

    draw(gl: WebGLRenderingContext): void {
        this.prepareProgram(gl)
        this.prepareVertexAttributes(gl)
        this.prepareTextures(gl)
        this.prepareBlending(gl)
        this.finalizeDraw(gl)
    }

    // private
    private finalizeDraw(gl: WebGLRenderingContext): void {
        gl.drawElements(gl.TRIANGLES, this._mesh.elementsCount, gl.UNSIGNED_SHORT, 0)
    }

    private prepareProgram(gl: WebGLRenderingContext): void {
        gl.useProgram(this._program.glShaderProgram)
    }

    private prepareVertexAttributes(gl: WebGLRenderingContext): void {
        const vertexPositionAttribLoc = gl.getAttribLocation(this._program.glShaderProgram, 'aVertexPosition')
        gl.bindBuffer(gl.ARRAY_BUFFER, this._mesh.glVertexBuffer)
        gl.vertexAttribPointer(vertexPositionAttribLoc, 2, gl.FLOAT, false, 0, 0)
        gl.enableVertexAttribArray(vertexPositionAttribLoc)
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this._mesh.glIndexBuffer)

        const uvBuffer = this._mesh.getNamedGlBuffer('uv')
        const uvAttribLoc = gl.getAttribLocation(this._program.glShaderProgram, 'aUvCoord')

        if (uvBuffer && uvAttribLoc !== -1) {
            gl.bindBuffer(gl.ARRAY_BUFFER, uvBuffer.glBuffer)
            gl.vertexAttribPointer(uvAttribLoc, uvBuffer.size, uvBuffer.glType, uvBuffer.normalize, 0, 0)
            gl.enableVertexAttribArray(uvAttribLoc)
        } else {
            throw new Error('Couldn\'t prepare PresentDrawCommand vertex attributes')
        }
    }

    private prepareTextures(gl: WebGLRenderingContext): void {
        gl.activeTexture(gl.TEXTURE0)
        gl.bindTexture(gl.TEXTURE_2D, this._texture)
    }

    private prepareBlending(gl: WebGLRenderingContext): void {
        gl.disable(gl.DEPTH_TEST)
        gl.disable(gl.BLEND)
    }
}
