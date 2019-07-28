
#version 410

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;

uniform LightInfo {
    vec4 LightPosition;  // Light position in eye coords.
    vec3 La;             // Ambient light intesity
    vec3 L;              // Diffuse and specular light intensity
};

uniform MaterialInfo {
    vec3 Ka;            // Ambient reflectivity
    vec3 Kd;            // Diffuse reflectivity
    vec3 Ks;            // Specular reflectivity
    float Shininess;    // Specular shininess factor
};

layout(location = 0) out vec4 FragColor;


vec3 phongModel(vec3 position, vec3 n) {

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

    FragColor = vec4(phongModel(Position, normalize(Normal)), 1.0);
}
