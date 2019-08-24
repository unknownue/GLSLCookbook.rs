
#version 410

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 TexCoord;


uniform float Gamma;

uniform LightInfo {
    vec4 LightPosition;  // Light position in eye coords.
    vec3 Intensity;      // A, D, S intensity
};

uniform MaterialInfo {
    vec3 Ka;            // Ambient reflectivity
    vec3 Kd;            // Diffuse reflectivity
    vec3 Ks;            // Specular reflectivity
    float Shininess;    // Specular shininess factor
};

layout(location = 0) out vec4 FragColor;


vec3 phongModel(vec3 position, vec3 n) {

    vec3 ambient = Intensity * Ka;
    vec3 s = normalize(LightPosition.xyz - position);

    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = Kd * sDotN;

    vec3 spec = vec3(0.0);
    if(sDotN > 0.0) {
        vec3 v = normalize(-position.xyz);
        vec3 r = reflect(-s, n);
        spec = Ks * pow(max(dot(v, r), 0.0), Shininess);
    }
    return ambient + Intensity * (diffuse + spec);
}

void main() {

    vec3 color = phongModel(Position, Normal);
    FragColor = vec4(pow(color, vec3(1.0 / Gamma)), 1.0);
}
