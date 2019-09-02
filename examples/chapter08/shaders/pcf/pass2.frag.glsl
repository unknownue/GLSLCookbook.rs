
#version 410

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec4 ShadowCoord;

layout(location = 0) out vec4 FragColor;

uniform sampler2DShadow ShadowMap;


uniform LightInfo {
    vec4 LightPosition;
    vec3 Intensity;
};

uniform MaterialInfo {
    vec3 Ka;            // Ambient reflectivity
    vec3 Kd;            // Diffuse reflectivity
    vec3 Ks;            // Specular reflectivity
    float Shininess;    // Specular shininess factor
};


vec3 phongModelDiffAndSpec() {

    vec3 n = Normal;
    vec3 s = normalize(LightPosition.xyz - Position);

    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = Intensity * Kd * sDotN;

    vec3 spec = vec3(0.0);
    if(sDotN > 0.0) {
        vec3 v = normalize(-Position.xyz);
        vec3 r = reflect(-s, n);
        spec = Intensity * Ks * pow(max(dot(r, v), 0.0), Shininess);
    }

    return diffuse + spec;
}

void shadeWithShadow() {
    
    vec3 ambient = Intensity * Ka;
    vec3 diffAndSpec = phongModelDiffAndSpec();

    // Lookup the texels nearby
    float sum = 0;
    float shadow = 1.0;

    // Dont' text points behind the light source.
    if(ShadowCoord.z >= 0) {
        // Sum contributions from 4 texels around ShadowCoord
        sum += textureProjOffset(ShadowMap, ShadowCoord, ivec2(-1, -1));
        sum += textureProjOffset(ShadowMap, ShadowCoord, ivec2(-1,  1));
        sum += textureProjOffset(ShadowMap, ShadowCoord, ivec2( 1,  1));
        sum += textureProjOffset(ShadowMap, ShadowCoord, ivec2( 1, -1));
        shadow = sum * 0.25;
    }

    FragColor = vec4(ambient + diffAndSpec * shadow, 1.0);

    // Gamma correct
    FragColor = pow(FragColor, vec4(1.0 / 2.2));
}

void main() {

    shadeWithShadow();
}
