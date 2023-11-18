varying lowp vec2 vUvCoord;
uniform sampler2D uTexture;

void main(void) {
    gl_FragColor = vec4(texture2D(uTexture, vUvCoord).r, 1.0, 1.0, 1.0);
}