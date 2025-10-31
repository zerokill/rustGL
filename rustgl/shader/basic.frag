#version 330 core
in vec3 ourColor;
in vec2 ourTexCoord;
out vec4 FragColor;

void main() {
    // For now, just use the color. UV coordinates are available as ourTexCoord
    // They can be used later for texturing
    FragColor = vec4(ourColor, 1.0);
}
