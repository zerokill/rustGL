#version 410 core

out vec4 FragColor;

uniform bool isOrb;  // true = render white (orb), false = render black (occluder)

void main()
{
    if (isOrb) {
        // Render orb as solid white
        FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    } else {
        // Render occluders as black
        FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
}
