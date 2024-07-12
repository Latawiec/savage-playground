attribute vec4 aVertexPosition;
attribute vec2 aUvCoord;
attribute vec3 aColor;

varying lowp vec2 vUvCoord;
varying lowp vec3 vColor;

void main(void) {
    gl_Position = vec4(aVertexPosition.xyz, 1);
    vUvCoord = aUvCoord.xy;
    vColor = aColor.rgb;
}