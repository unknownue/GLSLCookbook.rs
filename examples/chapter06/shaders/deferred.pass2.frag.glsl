
#version 410

layout (location = 0) out vec4 FragColor;

layout (location = 0) in vec2 TexCoord;


uniform sampler2D PositionTex;
uniform sampler2D NormalTex;
uniform sampler2D ColorTex;

uniform LightInfo {
    vec4 LightPosition;  // Light position in cam. coords.
    vec3 Intensity;      // Diffuse intensity
};

vec3 diffuseModel(vec3 pos, vec3 norm, vec3 diff) {

    vec3 s = normalize(LightPosition.xyz - pos);
    float sDotN = max(dot(s, norm), 0.0);

    return Intensity * diff * sDotN;
}

void main() {

    // Retrieve position and normal information from textures
    vec3 pos  = vec3(texture(PositionTex, TexCoord));
    vec3 norm = vec3(texture(NormalTex, TexCoord));
    vec3 diffColor = vec3(texture(ColorTex, TexCoord));

    FragColor = vec4(diffuseModel(pos, norm, diffColor), 1.0);
}
