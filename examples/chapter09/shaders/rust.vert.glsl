
#version 410

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexNormal;
layout (location = 2) in vec2 VertexTexCoord;

layout (location = 0) out vec3 ReflectDir;
layout (location = 1) out vec2 TexCoord;

uniform vec3 WorldCameraPosition;
uniform mat4 ModelMatrix;
uniform mat4 MVP;

void main() {

    vec3 worldPos    = vec3(ModelMatrix * vec4(VertexPosition, 1.0));
    vec3 worldNormal = vec3(ModelMatrix * vec4(VertexNormal, 0.0));
    vec3 worldView   = normalize(WorldCameraPosition - worldPos);

    TexCoord = VertexTexCoord;
    ReflectDir = reflect(-worldView, worldNormal);

    gl_Position = MVP * vec4(VertexPosition, 1.0);
}
