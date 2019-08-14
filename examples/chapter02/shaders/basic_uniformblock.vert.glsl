
#version 410

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec2 VertexTexCoord;

layout (location = 0) out vec2 TexCoord;

void main() {

    TexCoord = VertexTexCoord;
    gl_Position = vec4(VertexPosition, 1.0);
}
