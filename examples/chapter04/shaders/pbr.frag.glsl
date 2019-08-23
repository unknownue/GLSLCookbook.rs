
#version 410

const float PI = 3.14159265358979323846;

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;

struct LightInfo {
    vec4 Position;  // Light position in cam. coords.
    vec3 L;         // Intensity
};

uniform LightsBlock {
    // Due to the limit of glium, 5 is specified here, but acutally 3 is used.
    LightInfo Light[5];
};

uniform MaterialInfo {
    vec3 MaterialColor;   // Diffuse color for dielectrics, f0 for metallic
    float MaterialRough;  // Roughness
    bool IsMetal;         // Metallic (true) or dielectric (false)
};

layout(location = 0) out vec4 FragColor;

float ggxDistribution(float nDotH) {

    float alpha2 = MaterialRough * MaterialRough * MaterialRough * MaterialRough;
    float d = (nDotH * nDotH) * (alpha2 - 1) + 1;
    return alpha2 / (PI * d * d);
}

float geomSmith(float dotProd) {
    float k = (MaterialRough + 1.0) * (MaterialRough + 1.0) / 8.0;
    float denom = dotProd * (1 - k) + k;
    return 1.0 / denom;
}

vec3 schlickFresnel(float lDotH) {
    vec3 f0 = vec3(0.04);
    if(IsMetal) {
        f0 = MaterialColor;
    }
    return f0 + (1 - f0) * pow(1.0 - lDotH, 5);
}

vec3 microfacetModel(int lightIdx, vec3 position, vec3 n) {

    vec3 diffuseBrdf = vec3(0.0);  // Metallic
    if(!IsMetal) {
        diffuseBrdf = MaterialColor;
    }

    vec3 l = vec3(0.0),
    lightI = Light[lightIdx].L;
    if(Light[lightIdx].Position.w == 0.0) {
        // Directional light
        l = normalize(Light[lightIdx].Position.xyz);
    } else {
        // Positional light
        l = Light[lightIdx].Position.xyz - position;
        float dist = length(l);
        l = normalize(l);
        lightI /= (dist * dist);
    }

    vec3 v = normalize(-position);
    vec3 h = normalize(v + l);
    float nDotH = dot(n, h);
    float lDotH = dot(l, h);
    float nDotL = max(dot(n, l), 0.0);
    float nDotV = dot(n, v);
    vec3 specBrdf = 0.25 * ggxDistribution(nDotH) * schlickFresnel(lDotH) * geomSmith(nDotL) * geomSmith(nDotV);

    return (diffuseBrdf + PI * specBrdf) * lightI * nDotL;
}

void main() {

    vec3 sum = vec3(0);
    vec3 n = normalize(Normal);

    for(int i = 0; i < 3; i++) {
        sum += microfacetModel(i, Position, n);
    }

    // Gamma
    sum = pow(sum, vec3(1.0 / 2.2));

    FragColor = vec4(sum, 1);
}
