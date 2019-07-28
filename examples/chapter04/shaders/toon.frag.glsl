
#version 410

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;

uniform LightInfo {
    vec4 LightPosition;  // Light position in eye coords.
    vec3 La;             // Ambient light intensity
    vec3 L;              // Diffuse/specular light intensity
};

uniform MaterialInfo {
    vec3 Ka;            // Ambient reflectivity
    vec3 Kd;            // Diffuse reflectivity
};

layout(location = 0) out vec4 FragColor;

const int levels = 3;
const float scaleFactor = 1.0 / levels;


vec3 toonShade() {

    vec3 n = normalize(Normal);
    vec3 s = normalize(LightPosition.xyz - Position);

    vec3 ambient = La * Ka;
    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = Kd * floor(sDotN * levels) * scaleFactor;

    return ambient + L * diffuse;
}

void main() {

    FragColor = vec4(toonShade(), 1.0);
}
