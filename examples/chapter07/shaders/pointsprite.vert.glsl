
#version 410

layout (location = 0) in vec3 VertexPosition;

uniform mat4 ModelViewMatrix;

void main() {

    // Convert the position to camera coordinates, not the clip coordinates
    gl_Position = ModelViewMatrix * vec4(VertexPosition, 1.0);
}
