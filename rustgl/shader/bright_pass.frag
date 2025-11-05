#version 410 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D screenTexture;
uniform float threshold;  // Brightness threshold (default: 1.0)

void main()
{
    vec3 color = texture(screenTexture, TexCoords).rgb;

    // Calculate perceptual brightness (weighted RGB)
    float brightness = dot(color, vec3(0.2126, 0.7152, 0.0722));

    // Only output if above threshold
    if (brightness > threshold) {
        FragColor = vec4(color, 1.0);
    } else {
        FragColor = vec4(0.0, 0.0, 0.0, 1.0);  // Black
    }
}
