
#version 410

layout (location = 0) in vec4 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 TexCoord;

layout(location = 0) out vec4 FragColor;

uniform sampler2D NoiseTex;

uniform float LowThreshold;
uniform float HighThreshold;

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

vec3 phongModel(vec4 position, vec3 n) {

    vec3 s = normalize(LightPosition.xyz - position.xyz);

    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = Kd * sDotN;

    vec3 spec = vec3(0.0);
    if(sDotN > 0.0) {
        vec3 v = normalize(-position.xyz);
        vec3 r = reflect(-s, n);
        spec = Ks * pow(max(dot(r, v), 0.0), Shininess);
    }

    return Intensity * (diffuse + spec);
}

void main() {

    vec4 noise = texture(NoiseTex, TexCoord);

    if (noise.a < LowThreshold)
        discard;
    if (noise.a > HighThreshold)
        discard;

    FragColor = vec4(phongModel(Position, normalize(Normal)), 1.0);
}
