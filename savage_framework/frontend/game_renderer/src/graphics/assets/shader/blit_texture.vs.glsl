#version 300 es

in vec2 aVertexPosition;
in vec2 aUvCoord;

out vec2 vUvCoord;

void main(void) {
    gl_Position = vec4(aVertexPosition.xy, 0.0, 1.0);
    vUvCoord = aUvCoord.xy;
}