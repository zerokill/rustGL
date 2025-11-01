#version 330 core
in vec3 ourColor;
in vec2 ourTexCoord;
out vec4 FragColor;

uniform sampler2D textureSampler;
uniform bool useTexture;

void main() {
    if (useTexture) {
        // Sample texture and multiply by vertex color for tinting
        vec4 texColor = texture(textureSampler, ourTexCoord);
        FragColor = texColor;
    } else {
        // Use vertex color only (current behavior)
        FragColor = vec4(ourColor, 1.0);
    }
}
