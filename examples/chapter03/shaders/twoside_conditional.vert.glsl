
#version 410

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexNormal;

layout (location = 0) out vec3 Color;

uniform LightInfo {
    vec4 LightPosition; // Light position in eye coords.
    vec3 La;       // Ambient light intensity
    vec3 Ld;       // Diffuse light intensity
    vec3 Ls;       // Specular light intensity
};

uniform MaterialInfo {
    vec3 Ka;            // Ambient reflectivity
    vec3 Kd;            // Diffuse reflectivity
    vec3 Ks;            // Specular reflectivity
    float Shininess;    // Specular shininess factor
};

uniform mat4 ModelViewMatrix;
uniform mat3 NormalMatrix;
uniform mat4 ProjectionMatrix;
uniform mat4 MVP;

vec3 phongModel(vec3 position, vec3 n) {

    vec3 ambient = La * Ka;
    vec3 s = normalize(LightPosition.xyz - position);
    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = Ld * Kd * sDotN;
    vec3 spec = vec3(0.0);

    if(sDotN > 0.0) {
        vec3 v = normalize(-position.xyz);
        vec3 r = reflect(-s, n);
        spec = Ls * Ks * pow(max(dot(r, v), 0.0), Shininess);
    }

    return ambient + diffuse + spec;
}

void main() {

    vec3 tnorm = normalize(NormalMatrix * VertexNormal);
    vec3 camCoords = (ModelViewMatrix * vec4(VertexPosition, 1.0)).xyz;

    vec3 v = normalize(-camCoords.xyz);
    float vDotN = dot(v, tnorm);

    if (vDotN >= 0.0) {
        Color = phongModel(camCoords, tnorm);
    } else {
        Color = phongModel(camCoords, -tnorm);
    }

    gl_Position = MVP * vec4(VertexPosition, 1.0);
}
