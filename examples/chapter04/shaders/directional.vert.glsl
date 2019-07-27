
#version 410

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexNormal;

layout (location = 0) out vec3 Color;

uniform LightInfo {
    vec4 LightPosition; // Light position in eye coords.
    vec3 La;            // Ambient light intensity
    vec3 L;             // Diffuse/Specular light intensity
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

vec3 phongModel(vec3 position, vec3 n) {

    vec3 ambient = La * Ka;

    vec3 s;
    // LightPosition is used to determine whether or not the light is to be treated as a directional light.
    if (LightPosition.w == 0.0) {
        // LightPosition is normalized and used as the direction toward the light source.
        s = normalize(LightPosition.xyz);
    } else {
        // LightPosition is treated as a location in eye coordinates.
        s = normalize(LightPosition.xyz - position);
    }

    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = Kd * sDotN;
    vec3 spec = vec3(0.0);

    if(sDotN > 0.0) {
        vec3 v = normalize(-position.xyz);
        vec3 r = reflect(-s, n);
        spec = Ks * pow(max(dot(r, v), 0.0), Shininess);
    }

    return ambient + L * (diffuse + spec);
}

void main() {

    vec3 camNorm = normalize(NormalMatrix * VertexNormal);
    vec3 camPosition = (ModelViewMatrix * vec4(VertexPosition, 1.0)).xyz;

    // Evaluate the lighting equation
    Color  = phongModel(camPosition, camNorm);

    gl_Position = MVP * vec4(VertexPosition, 1.0);
}
