
#version 410

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexNormal;
layout (location = 2) in vec2 VertexTexCoord;

// layout (location = 0) centroid out vec2 TexCoord;
layout (location = 0) out vec2 TexCoord;

uniform mat4 MVP;

void main() {

    TexCoord = VertexTexCoord;
    gl_Position = MVP * vec4(VertexPosition, 1.0);
}
