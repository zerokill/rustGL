#version 410 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D sceneTexture;
uniform vec3 lightColor;        // Color of the light source
uniform float luminanceThreshold; // Threshold to detect light

void main()
{
    vec3 color = texture(sceneTexture, TexCoords).rgb;

    // Calculate luminance
    float luminance = dot(color, vec3(0.2126, 0.7152, 0.0722));

    // If pixel is bright enough (likely the light source), output white
    // Otherwise, output black (occlusion)
    if (luminance > luminanceThreshold) {
        FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    } else {
        FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
}
