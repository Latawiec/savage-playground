#version 300 es

precision mediump float;

in vec2 vUvCoord;
uniform sampler2D uTexture;

out vec4 FragColor;

void main(void) {
    FragColor = vec4(texture(uTexture, vUvCoord).rgb, 1.0);
}