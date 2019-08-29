
#version 410

layout (location = 0) in vec3 GPosition;
layout (location = 1) in vec3 GNormal;
layout (location = 2) flat in int GIsEdge;

layout (location = 0) out vec4 FragColor;


uniform LightInfo {
    vec4 LightPosition;  // Light position in eye coords.
    vec3 Intensity; // A,D,S intensity
};

uniform MaterialInfo {
    vec3 Ka;  // Ambient reflectivity
    vec3 Kd;  // Diffuse reflectivity
};

uniform vec4 LineColor;

const int levels = 3;
const float scaleFactor = 1.0 / levels;


vec3 toonShade() {

    vec3 s = normalize(LightPosition.xyz - GPosition.xyz);
    vec3 ambient = Ka;
    float cosine = dot(s, GNormal);
    vec3 diffuse = Kd * ceil(cosine * levels) * scaleFactor;

    return Intensity * (ambient + diffuse);
}

void main() {

    if(GIsEdge == 1) {
        FragColor = LineColor;
    } else {
        FragColor = vec4(toonShade(), 1.0);
    }
}
