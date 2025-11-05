#version 410 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D occlusionTexture;
uniform vec2 lightScreenPos;    // Light position in screen space [0-1]
uniform float exposure;         // Overall intensity
uniform float decay;            // Light decay factor (0.95-0.99)
uniform float density;          // Sample density (0.5-1.0)
uniform float weight;           // Sample weight (0.1-0.5)
uniform int numSamples;         // Number of samples (typically 100)

void main()
{
    // Create a local copy of TexCoords that we can modify
    vec2 texCoord = TexCoords;

    // Calculate vector from current position to light position
    vec2 deltaTexCoord = texCoord - lightScreenPos;

    // Divide by number of samples and multiply by density
    deltaTexCoord *= 1.0 / float(numSamples) * density;

    // Store initial sample
    vec3 color = texture(occlusionTexture, texCoord).rgb;

    // Set up illumination decay factor
    float illuminationDecay = 1.0;

    // Accumulate samples along ray from pixel to light
    for (int i = 0; i < numSamples; i++) {
        // Step towards light
        texCoord -= deltaTexCoord;

        // Sample occlusion texture
        vec3 sampleData = texture(occlusionTexture, texCoord).rgb;

        // Apply decay and weight
        sampleData *= illuminationDecay * weight;

        // Accumulate
        color += sampleData;

        // Decay illumination
        illuminationDecay *= decay;
    }

    // Apply exposure
    FragColor = vec4(color * exposure, 1.0);
}

