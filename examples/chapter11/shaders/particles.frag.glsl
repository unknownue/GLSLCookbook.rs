
#version 430

layout (location = 0) in vec3 Position;

layout (location = 0) out vec4 FragColor;

uniform vec4 Color;

void main() {

    FragColor = Color;
}
