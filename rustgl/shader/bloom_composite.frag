#version 410 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D scene;          // Original scene
uniform sampler2D bloomBlur;      // Blurred bright areas
uniform float bloomStrength;      // How much bloom to add (default: 1.0)

void main()
{
    vec3 sceneColor = texture(scene, TexCoords).rgb;
    vec3 bloomColor = texture(bloomBlur, TexCoords).rgb;

    // Additive blending with strength control
    vec3 result = sceneColor + bloomColor * bloomStrength;

    FragColor = vec4(result, 1.0);
}
