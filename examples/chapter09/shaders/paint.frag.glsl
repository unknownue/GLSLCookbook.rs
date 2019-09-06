
#version 410

layout (location = 0) in vec4 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 TexCoord;

layout (location = 0) out vec4 FragColor;

uniform sampler2D NoiseTex;

uniform vec3 PaintColor = vec3(1.0);
uniform float Threshold = 0.62;

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

    vec3 n = normalize(Normal);
    vec3 s = normalize(LightPosition.xyz - Position.xyz);

    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = kd * sDotN;

    vec3 spec = vec3(0.0);
    if(sDotN > 0.0) {
        vec3 v = normalize(-Position.xyz);
        vec3 r = reflect(-s, n);
        spec = Ks * pow(max(dot(r, v), 0.0), Shininess);
    }

    return Intensity * (diffuse + spec);
}

void main() {

    vec4 noise = texture(NoiseTex, TexCoord);

    vec3 color = Kd;
    if (noise.a > Threshold)
        color = PaintColor;

    FragColor = vec4(phongModel(color), 1.0);
}
