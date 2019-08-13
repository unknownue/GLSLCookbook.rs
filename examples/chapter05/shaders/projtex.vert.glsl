
#version 410

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexNormal;

layout (location = 0) out vec3 EyeNormal;  // Normal in eye coordinates
layout (location = 1) out vec4 EyePosition; // Position in eye coordinates
layout (location = 2) out vec4 ProjTexCoord;

uniform mat4 ProjectorMatrix;

uniform mat4 ModelViewMatrix;
uniform mat4 ModelMatrix;
uniform mat3 NormalMatrix;
uniform mat4 MVP;

void main() {

    vec4 pos4 = vec4(VertexPosition, 1.0);

    EyeNormal    = normalize(NormalMatrix * VertexNormal);
    EyePosition  = ModelViewMatrix * pos4;
    ProjTexCoord = ProjectorMatrix * (ModelMatrix * pos4);

    gl_Position = MVP * pos4;
}
