
#version 410

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexNormal;
layout (location = 2) in vec2 VertexTexCoord;

layout (location = 0) out vec3 LightIntensity;

uniform vec4 LightPosition; // Light position in eye coords.
uniform vec3 Kd;            // Diffuse reflectivity.
uniform vec3 Ld;            // Diffuse light intensity.

uniform mat4 ModelViewMatrix;
uniform mat3 NormalMatrix;
uniform mat4 MVP;

void main() {

    // Transforming the normal vector using the normal matrix.
    // The normal matrix is the inverse transpose of the upper-left 3x3 portion of the model-view matrix.
    vec3 tnorm = normalize(NormalMatrix * VertexNormal);

    vec4 eyeCoords = ModelViewMatrix * vec4(VertexPosition, 1.0);
    vec3 s = normalize(vec3(LightPosition - eyeCoords));

    LightIntensity = Ld * Kd * max(dot(s, tnorm), 0.0);

    // The subsequent stage of the OpenGL pipeline expects that
    // the vertex position will be provided in clip space coordinates in the output variable gl_Position.
    gl_Position = MVP * vec4(VertexPosition, 1.0);
}
