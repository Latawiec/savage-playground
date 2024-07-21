#version 300 es

precision highp float;

in vec3 FragPos;
in vec3 Normal;
in vec2 UV;

out vec4 FragColor;

// uniform vec3 lightPos;
// uniform vec3 viewPos;

void main() {
    vec3 norm = normalize(Normal);
    // vec3 lightDir = normalize(lightPos - FragPos);
    // float diff = max(dot(norm, lightDir), 0.0);
    // vec3 diffuse = vec3(1.0, 1.0, 1.0) * diff; // Assume white light

    // vec3 viewDir = normalize(viewPos - FragPos);
    // vec3 reflectDir = reflect(-lightDir, norm);
    // float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32.0); // Assume shininess of 32
    // vec3 specular = vec3(1.0, 1.0, 1.0) * spec; // Assume white light

    vec3 result = norm;//diffuse + specular;
    FragColor = vec4((norm + 1.0) / 2.0, 1.0);
}