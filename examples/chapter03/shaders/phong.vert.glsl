
#version 410

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexNormal;
layout (location = 2) in vec2 VertexTexCoord;

layout (location = 0) out vec3 LightIntensity;

uniform LightInfo {
    vec4 LightPosition; // Light position in eye coords
    vec3 La;            // Ambient light intensity
    vec3 Ld;            // Diffuse light intensity
    vec3 Ls;            // Specular light intensity
};

uniform MaterialInfo {
    vec3 Ka;            // Ambient reflectivity
    vec3 Kd;            // Diffuse reflectivity
    vec3 Ks;            // Specular reflectivity
    float Shininess;    // Specular shininess factor
};

uniform mat4 ModelViewMatrix;
uniform mat3 NormalMatrix;
uniform mat4 MVP;

void main() {

    vec3 n = normalize(NormalMatrix * VertexNormal);
    // camCoords is the vertex position of object in camera space.
    vec4 camCoords = ModelViewMatrix * vec4(VertexPosition, 1.0);

    vec3 ambient = La * Ka;
    vec3 s = normalize(vec3(LightPosition - camCoords));
    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = Ld * Kd * sDotN;
    vec3 spec = vec3(0.0);

    if(sDotN > 0.0) {
		// In camera space, the camera is at origin (0, 0, 0)
        vec3 v = normalize(-camCoords.xyz);
        vec3 r = reflect(-s, n);
        spec = Ls * Ks * pow(max(dot(r, v), 0.0), Shininess);
    }

    LightIntensity = ambient + diffuse + spec;
    gl_Position = MVP * vec4(VertexPosition, 1.0);
}
