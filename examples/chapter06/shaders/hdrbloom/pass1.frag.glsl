
#version 410

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;

layout (location = 0) out vec4 FragColor;

struct LightInfo {
    vec4 Position;  // Light position in cam. coords.
    vec3 L;         // D,S intensity
    vec3 La;        // Amb intensity
};
uniform LightBlock {
    LightInfo Lights[5];
};

uniform MaterialInfo {
    vec3 Ka;            // Ambient reflectivity
    vec3 Kd;            // Diffuse reflectivity
    vec3 Ks;            // Specular reflectivity
    float Shininess;    // Specular shininess factor
};


vec3 blinnPhong(vec3 position, vec3 n, int idx) {

    vec3 ambient = Lights[idx].La * Ka;
    vec3 s = normalize(Lights[idx].Position.xyz - position);
    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = Kd * sDotN;

    vec3 spec = vec3(0.0);
    if(sDotN > 0.0) {
        vec3 v = normalize(-position.xyz);
        vec3 h = normalize(v + s);
        spec = Ks * pow(max(dot(h, n), 0.0), Shininess);
    }
    return ambient + Lights[idx].L * (diffuse + spec);
}


void main() {

    vec3 n = normalize(Normal);
    vec3 color = vec3(0.0);
    for(int i = 0; i < 3; i++) {
        color += blinnPhong(Position, n, i);
    }

    FragColor = vec4(color, 1.0);
}
