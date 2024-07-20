import { ShaderProgram } from '../commited_storage/ProgramStorage'
import { Mesh } from '../commited_storage/MeshStorage'
import { Texture } from '../commited_storage/TextureStorage'
import { ShaderValueType } from '../common/GLTypes'
import { IDrawCommand } from './IDrawCommand'

export class GeneralDrawCommand implements IDrawCommand {
    private program: ShaderProgram;
    private mesh: Mesh;
    private textures: Map<number, Texture> | undefined;
    private uniformAttrs: Map<ShaderValueType, Map<string, number | number[]>> | undefined;
    private vertexAttrs: Map<string, string>;
    // private layer: number;
    // private billboard: boolean;

    constructor (
      program: ShaderProgram,
      mesh: Mesh,
      textures: Map<number, Texture> | undefined,
      uniformAttrs: Map<ShaderValueType, Map<string, number | number[]>> | undefined,
      vertexAttrs: Map<string, string>,
      _layer: number,
      _billboard: boolean
    ) {
      this.program = program
      this.mesh = mesh
      this.textures = textures
      this.uniformAttrs = uniformAttrs
      this.vertexAttrs = vertexAttrs
      // this.layer = layer
      // this.billboard = billboard
    }

    // impl IDrawRequest
    draw (gl: WebGLRenderingContext): void {
      this.prepareProgram(gl)
      this.prepareVertexAttributes(gl)
      this.prepareUniformAttributes(gl)
      this.prepareTextures(gl)
      this.prepareBlending(gl)
      this.finalizeDraw(gl)
    }

    // private
    private finalizeDraw (gl: WebGLRenderingContext): void {
      // Assume that we always have triangles.
      // Assume that indices are always short ints.
      // Assume index offset is always 0.
      gl.drawElements(gl.TRIANGLES, this.mesh.elementsCount, gl.UNSIGNED_SHORT, 0)
    }

    private prepareProgram (gl: WebGLRenderingContext): void {
      gl.useProgram(this.program.glShaderProgram)
    }

    private prepareVertexAttributes (gl: WebGLRenderingContext): void {
      // Required attributes (vertices, indices)
      const verticesAttributeName = this.vertexAttrs.get('vertices')
      if (!verticesAttributeName) {
        throw new Error('Vertices attribute not found.')
      }

      const vertexPositionAttribLoc = gl.getAttribLocation(this.program.glShaderProgram, verticesAttributeName)
      gl.bindBuffer(gl.ARRAY_BUFFER, this.mesh.glVertexBuffer)
      gl.vertexAttribPointer(vertexPositionAttribLoc, 3, gl.FLOAT, false, 0, 0)
      gl.enableVertexAttribArray(vertexPositionAttribLoc)
      gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.mesh.glIndexBuffer)

      // Optional attributes
      for (const [bufferName, attribName] of this.vertexAttrs) {
        if (bufferName === 'vertices') {
          // Special. For now.
          continue
        }

        const meshNamedBuffer = this.mesh.getNamedGlBuffer(bufferName)
        const attribLoc = gl.getAttribLocation(this.program.glShaderProgram, attribName)

        if (!meshNamedBuffer) {
          console.warn(`Mesh doesn't have required named buffer: ${bufferName}. Vertex attribute couldn't be set.`)
          continue
        }

        if (attribLoc < 0) {
          console.warn(`Couldn't locate ${attribName} in shader program. Vertex attribute couldn't be set.`)
          continue
        }

        const glType = meshNamedBuffer.glType
        const glBuffer = meshNamedBuffer.glBuffer
        const size = meshNamedBuffer.size
        const normalize = meshNamedBuffer.normalize

        gl.bindBuffer(gl.ARRAY_BUFFER, glBuffer)
        gl.enableVertexAttribArray(attribLoc)
        gl.vertexAttribPointer(attribLoc, size, glType, normalize, 0, 0)
      }
    }

    private prepareUniformAttributes (gl: WebGLRenderingContext): void {
      if (this.uniformAttrs === undefined) {
        return
      }

      for (const [type, attributes] of this.uniformAttrs) {
        for (const [uniformName, uniformValues] of attributes) {
          const uniformLocation = gl.getUniformLocation(this.program.glShaderProgram, uniformName)
          if (uniformLocation === null) {
            console.error(`Uniform attribute ${uniformName} not found in shader program.`)
            continue
          }

          switch (type) {
            case 'mat4': gl.uniformMatrix4fv(uniformLocation, false, uniformValues as Array<number>); break

            case 'float': gl.uniform1f(uniformLocation, uniformValues as number); break
            case 'vec2': gl.uniform2fv(uniformLocation, uniformValues as Array<number>); break
            case 'vec3': gl.uniform3fv(uniformLocation, uniformValues as Array<number>); break
            case 'vec4': gl.uniform4fv(uniformLocation, uniformValues as Array<number>); break

            case 'int': gl.uniform1i(uniformLocation, uniformValues as number); break
            case 'ivec2': gl.uniform2iv(uniformLocation, uniformValues as Array<number>); break
            case 'ivec3': gl.uniform3iv(uniformLocation, uniformValues as Array<number>); break
            case 'ivec4': gl.uniform4iv(uniformLocation, uniformValues as Array<number>); break

            default: console.error(`Uniform type unknown: ${type}`)
          }
        }
      }
    }

    private prepareTextures (gl: WebGLRenderingContext): void {
      if (this.textures === undefined) {
        return
      }

      for (const [textureOffset, texture] of this.textures) {
        gl.activeTexture(gl.TEXTURE0 + textureOffset)
        gl.bindTexture(gl.TEXTURE_2D, texture.glTexture)
      }
    }

    private prepareBlending (_gl: WebGLRenderingContext): void {
      // Just use standard for now.
      // canvas.glContext.blendFunc(canvas.glContext.SRC_ALPHA, canvas.glContext.ONE); // Additive blending.
      // gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA); // real transparency
    }
}
