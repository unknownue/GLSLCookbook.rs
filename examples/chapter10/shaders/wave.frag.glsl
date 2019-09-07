
#version 410

layout (location = 0) in vec4 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 TexCoord;

layout (location = 0) out vec4 FragColor;

uniform LightInfo {
    vec4 LightPosition;
    vec3 Intensity;
};

uniform MaterialInfo {
    vec3 Ka;
    vec3 Kd;
    vec3 Ks;
    float Shininess;
};


vec3 phongModel(vec3 kd) {

    vec3 ambient = Ka * Intensity;

    vec3 n = Normal;
    vec3 s = normalize(LightPosition.xyz - Position.xyz);
    vec3 r = reflect(-s, n);

    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = Intensity * kd * sDotN;

    vec3 spec = vec3(0.0);
    if (sDotN > 0.0) {
        vec3 v = normalize(-Position.xyz);
        spec = Intensity * Ks * pow(max(dot(r, v), 0.0), Shininess);
    }

    return ambient + diffuse + spec;
}

void main() {

    FragColor = vec4(phongModel(Kd) , 1.0);
}
