
#version 410

layout (location = 0) in vec3 fPosition;
layout (location = 1) in vec3 fNormal;

layout (location = 0) out vec4 FragColor;

uniform LightInfo {
    vec4 LightPosition;  // Light position in eye coords.
    vec3 Intensity;      // A,D,S intensity
};

uniform MaterialInfo {
    vec3 Ka;            // Ambient reflectivity
    vec3 Kd;            // Diffuse reflectivity
    vec3 Ks;            // Specular reflectivity
    float Shininess;    // Specular shininess factor
};


vec3 phongModel(vec3 pos, vec3 norm) {

    vec3 s = normalize(vec3(LightPosition) - pos);

    vec3 ambient = Intensity * Ka;

    float sDotN = max(dot(s, norm), 0.0);
    vec3 diffuse = Intensity * Kd * sDotN;

    vec3 spec = vec3(0.0);
    if(sDotN > 0.0) {
        vec3 v = normalize(-pos.xyz);
        vec3 r = reflect( -s, norm );
        spec = Intensity * Ks * pow(max(dot(r, v), 0.0), Shininess);
    }

    return ambient + diffuse + spec;
}

void main() {

    FragColor = vec4(phongModel(fPosition, fNormal), 1.0);
}
