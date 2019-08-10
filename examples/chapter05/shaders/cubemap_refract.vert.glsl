
#version 410

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexNormal;
layout (location = 2) in vec2 VertexTexCoord;

layout (location = 0) out vec3 ReflectDir;
layout (location = 1) out vec3 RefractDir;

uniform MaterialInfo {
    float Eta;              // Index of refraction
    float ReflectionFactor; // Percentage of reflected light
};


uniform vec3 WorldCameraPosition;
uniform mat4 ModelMatrix;
uniform mat4 MVP;

void main() {

    vec3 worldPos  = vec3(ModelMatrix * vec4(VertexPosition, 1.0));
    // Set 0.0 to w component to avoid translation component of the model matrix affecting the normal.
    // And the model matrix must not contain any non-uniform scaling component.
    vec3 worldNorm = vec3(ModelMatrix * vec4(VertexNormal, 0.0));
    vec3 worldView = normalize(WorldCameraPosition - worldPos);

    ReflectDir = reflect(-worldView, worldNorm);
    RefractDir = refract(-worldView, worldNorm, Eta);
    gl_Position = MVP * vec4(VertexPosition, 1.0);
}
