attribute vec4 aVertexPosition;
attribute vec2 aUvCoord;

varying lowp vec2 vUvCoord;

void main(void) {
    gl_Position = vec4(aVertexPosition.xyz, 1);
    vUvCoord = aUvCoord.xy;
}