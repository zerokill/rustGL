#version 330 core
in vec3 ourColor;
in vec3 ourNormal;
in vec2 ourTexCoord;
in vec3 fragPos;

out vec4 FragColor;

uniform sampler2D textureSampler;
uniform bool useTexture;

uniform vec3 viewPos;

// Material properties
uniform vec3 material_ambient;
uniform vec3 material_diffuse;
uniform vec3 material_specular;
uniform float material_shininess;

#define MAX_LIGHTS 4
uniform int numLights;

struct Light {
    vec3 position;
    vec3 color;
    float constant;
    float linear;
    float quadratic;
};

uniform Light lights[MAX_LIGHTS];

vec3 calculatePointLight(Light light, vec3 normal, vec3 fragPos, vec3 viewDir, vec3 objectColor) {
    vec3 lightDir = normalize(light.position - fragPos);

    float diff = max(dot(normal, lightDir), 0.0);

    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material_shininess);

    float distance = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * distance * distance);

    vec3 ambient = material_ambient * light.color;
    vec3 diffuse = diff * material_diffuse * light.color;
    vec3 specular = spec * material_specular * light.color;

    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;

    return (ambient + diffuse + specular) * objectColor;
}

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
    vec3 viewDir = normalize(viewPos - fragPos);

    vec3 result = vec3(0.0);

    for (int i = 0; i < MAX_LIGHTS; i++) {
        result += calculatePointLight(lights[i], norm, fragPos, viewDir, objectColor);
    }

    FragColor = vec4(result, 1.0);
}
