
#version 410

layout (location = 0) in vec3 VertexPosition;

layout (location = 0) out vec3 Vec;

uniform mat4 MVP;

void main() {

    Vec = VertexPosition;
    gl_Position = MVP * vec4(VertexPosition, 1.0);
}
