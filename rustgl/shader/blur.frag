#version 410 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D image;
uniform bool horizontal;  // true = horizontal blur, false = vertical blur

// Gaussian blur weights (5-tap)
float weights[5] = float[] (0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);

void main()
{
    vec2 tex_offset = 1.0 / textureSize(image, 0);  // Size of single texel
    vec3 result = texture(image, TexCoords).rgb * weights[0];  // Current fragment

    if (horizontal) {
        // Horizontal blur (sample left and right)
        for (int i = 1; i < 5; ++i) {
            result += texture(image, TexCoords + vec2(tex_offset.x * i, 0.0)).rgb * weights[i];
            result += texture(image, TexCoords - vec2(tex_offset.x * i, 0.0)).rgb * weights[i];
        }
    } else {
        // Vertical blur (sample up and down)
        for (int i = 1; i < 5; ++i) {
            result += texture(image, TexCoords + vec2(0.0, tex_offset.y * i)).rgb * weights[i];
            result += texture(image, TexCoords - vec2(0.0, tex_offset.y * i)).rgb * weights[i];
        }
    }

    FragColor = vec4(result, 1.0);
}
