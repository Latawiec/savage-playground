#version 300 es

precision highp float;

in vec3 aPos;
in vec3 aNormal;
in vec2 aUV;

out vec3 FragPos;
out vec3 Normal;
out vec2 UV;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    vec4 worldPosition = model * vec4(aPos, 1.0);
    FragPos = worldPosition.xyz;
    Normal = mat3(transpose(inverse(model))) * aNormal;
    UV = aUV;
    gl_Position = projection * view * worldPosition;
}