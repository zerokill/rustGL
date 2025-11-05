#version 410 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D scene;        // Original scene
uniform sampler2D godRays;      // Radial blur result
uniform float godRayStrength;   // Blending strength

void main()
{
    vec3 sceneColor = texture(scene, TexCoords).rgb;
    vec3 godRayColor = texture(godRays, TexCoords).rgb;

    // Additive blending
    vec3 result = sceneColor + godRayColor * godRayStrength;

    FragColor = vec4(result, 1.0);
}

