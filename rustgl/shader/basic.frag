#version 330 core
in vec3 ourColor;
in vec3 ourNormal;
in vec2 ourTexCoord;
in vec3 fragPos;

out vec4 FragColor;

uniform sampler2D textureSampler;
uniform bool useTexture;

uniform vec3 lightPos;
uniform vec3 viewPos;
uniform vec3 lightColor;

void main() {
    vec3 objectColor;
    if (useTexture) {
        // Sample texture and multiply by vertex color for tinting
        objectColor = texture(textureSampler, ourTexCoord).rgb;
    } else {
        // Use vertex color only (current behavior)
        objectColor = ourColor;
    }

    vec3 norm = normalize(ourNormal);
    vec3 lightDir = normalize(lightPos - fragPos);

    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    float specularStrength = 0.5;
    vec3 viewDir = normalize(viewPos - fragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * lightColor;

    vec3 result = (ambient + diffuse + specular) * objectColor;

    FragColor = vec4(result, 1.0);
}
