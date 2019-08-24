
#version 410

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 TexCoord;

out vec3 PositionData;
out vec3 NormalData;
out vec3 ColorData;

uniform MaterialInfo {
    vec3 Kd; // Diffuse reflectivity
};

void main() {
    // Store position, normal, and diffuse color in textures
    PositionData = Position;
    NormalData   = normalize(Normal);
    ColorData    = Kd;
}
