varying lowp vec2 vUvCoord;
uniform sampler2D uTexture;

void main(void) {
    gl_FragColor = texture2D(uTexture, vUvCoord);
}