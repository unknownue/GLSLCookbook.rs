
#version 410

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 TexCoord;

layout(location = 0) out vec4 FragColor;

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

vec3 phongModel(vec3 pos, vec3 n) {

    vec3 ambient = Ka;

    vec3 s = normalize(LightPosition.xyz - pos);
    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = Kd * sDotN;

    vec3 spec = vec3(0.0);
    if(sDotN > 0.0) {
        vec3 v = normalize(-pos);
        vec3 r = reflect(-s, n);
        spec = Ks * pow(max(dot(r, v), 0.0), Shininess);
    }

    return Intensity * (ambient + diffuse + spec);
}

void main() {

    FragColor = vec4(phongModel(Position, normalize(Normal)), 1.0);
}
