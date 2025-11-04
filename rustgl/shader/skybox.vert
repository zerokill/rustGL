#version 410 core

layout (location = 0) in vec3 aPos;

out vec3 TexCoords;

uniform mat4 projection;
uniform mat4 view;

void main()
{
    TexCoords = aPos;  // Use position as texture coordinates

    // Remove translation from view matrix
    // We only want rotation, not position changes
    mat4 viewNoTranslation = mat4(mat3(view));

    vec4 pos = projection * viewNoTranslation * vec4(aPos, 1.0);

    // Trick: Set z = w so that after perspective division, z/w = 1.0 (max depth)
    // This ensures skybox is always rendered behind everything
    gl_Position = pos.xyww;
}
