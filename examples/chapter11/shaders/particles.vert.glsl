
#version 430

layout (location = 0) in vec4 VertexPosition;

layout (location = 0) out vec3 Position;

uniform mat4 ModelViewMatrix;
uniform mat4 NormalMatrix;
uniform mat4 MVP;

void main() {

    Position = (ModelViewMatrix * VertexPosition).xyz;
    gl_Position = MVP * VertexPosition;
}
