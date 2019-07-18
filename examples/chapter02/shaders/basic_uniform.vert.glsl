
#version 410

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;

layout (location = 0) out vec3 Color;

//uniform struct {
//  mat4 RotationMatrix;
//  mat4 ViewMatrix;
//} MyMats;

uniform mat4 RotationMatrix;

//uniform mat4 Mats[2];

void main() {
    Color = color;
    gl_Position = RotationMatrix * vec4(position, 1.0);
}
