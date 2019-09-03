
#version 410

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 TexCoord;

layout (location = 0) out vec4 FragColor;

uniform LightInfo {
    vec4 LightPosition;
    vec3 Intensity;
};

uniform sampler2D AOTex;
uniform sampler2D DiffTex;

vec3 phongModelDiffuse() {

    vec3 n = Normal;
    vec3 s = normalize(vec3(LightPosition) - Position);
    float sDotN = max(dot(s,n), 0.0);
    vec3 diffColor = texture(DiffTex, TexCoord).rgb;

    return Intensity * diffColor * sDotN;
}

void main() {

    vec3 diffuse = phongModelDiffuse();

    vec4 aoFactor = texture(AOTex, TexCoord);

    FragColor = vec4(diffuse * aoFactor.r, 1.0);
}
