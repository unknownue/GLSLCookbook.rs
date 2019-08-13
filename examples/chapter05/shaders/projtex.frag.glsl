
#version 410

layout (location = 0) in vec3 EyeNormal;
layout (location = 1) in vec4 EyePosition;
layout (location = 2) in vec4 ProjTexCoord;

uniform sampler2D ProjectorTex;

uniform LightInfo {
    vec4 LightPosition;  // Light position in cam. coords.
    vec3 L;              // D,S intensity
    vec3 La;             // Ambient intensity
};

uniform MaterialInfo {
    vec3 Ka;            // Ambient reflectivity
    vec3 Kd;            // Diffuse reflectivity
    vec3 Ks;            // Specular reflectivity
    float Shininess;    // Specular shininess factor
};

layout(location = 0) out vec4 FragColor;


vec3 blinnPhong(vec3 position, vec3 n) {

    vec3 ambient = La * Ka;
    vec3 s = normalize(LightPosition.xyz - position);
    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = Kd * sDotN;

    vec3 spec = vec3(0.0);
    if(sDotN > 0.0) {
        vec3 v = normalize(-position.xyz);
        vec3 h = normalize(v + s);
        spec = Ks * pow(max(dot(h, n), 0.0), Shininess);
    }
    return ambient + L * (diffuse + spec);
}

void main() {

    vec3 color = blinnPhong(EyePosition.xyz, normalize(EyeNormal));

    vec3 projTexColor = vec3(0.0);
    if(ProjTexCoord.z > 0.0) {
        projTexColor = textureProj(ProjectorTex, ProjTexCoord).rgb;
    }

    FragColor = vec4(color + projTexColor * 0.5, 1.0);
}
