
#version 410

layout (location = 0) in vec4 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) noperspective in vec3 EdgeDistance;

layout (location = 0) out vec4 FragColor;

uniform float LineWidth;
uniform vec4 LineColor;
uniform vec4 LightPosition;
uniform vec3 LightIntensity;
uniform vec3 Kd;


vec3 diffuseModel(vec3 pos, vec3 norm) {

    vec3 s = normalize(LightPosition.xyz - pos);
    float sDotN = max(dot(s, norm), 0.0);
    vec3 diffuse = LightIntensity * Kd * sDotN;

    return diffuse;
}

float edgeMix() {
    
    // Find the smallest distance
    float d = min(min(EdgeDistance.x, EdgeDistance.y), EdgeDistance.z);

    if(d < LineWidth - 1.0) {
        return 1.0;
    } else if(d > LineWidth + 1) {
        return 0.0;
    } else {
        float x = d - (LineWidth - 1.0);
        return exp2(-2.0 * (x * x));
    }
}

void main() {

    float mixVal = edgeMix();
    vec4 color = vec4(diffuseModel(Position.xyz, Normal), 1.0);
    color = pow(color, vec4(1.0 / 2.2));

    FragColor = mix(color, LineColor, mixVal);
}
