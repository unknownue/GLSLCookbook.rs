
#version 410

layout (location = 0) in vec3 GPosition;
layout (location = 1) in vec3 GNormal;

noperspective in vec3 GEdgeDistance;

layout (location = 0) out vec4 FragColor;

uniform LightInfo {
    vec4 LightPosition;  // Light position in eye coords.
    vec3 Intensity; // A,D,S intensity
};

uniform MaterialInfo {
    vec3 Ka;            // Ambient reflectivity
    vec3 Kd;            // Diffuse reflectivity
    vec3 Ks;            // Specular reflectivity
    float Shininess;    // Specular shininess factor
};

uniform LineInfo {
    vec4 LineColor;
    float LineWidth;
};

vec3 phongModel(vec3 pos, vec3 norm) {

    vec3 s = normalize(LightPosition.xyz - pos);
    vec3 v = normalize(-pos.xyz);
    vec3 r = reflect(-s, norm);
    vec3 ambient = Intensity * Ka;
    float sDotN = max(dot(s, norm), 0.0);
    vec3 diffuse = Intensity * Kd * sDotN;
    vec3 spec = vec3(0.0);
    if(sDotN > 0.0)
        spec = Intensity * Ks * pow(max(dot(r, v), 0.0), Shininess);

    return ambient + diffuse + spec;
}

void main() {

    vec4 color = vec4(phongModel(GPosition, GNormal), 1.0);

    // Find the smallest distance
    float d = min(GEdgeDistance.x, GEdgeDistance.y);
    d = min(d, GEdgeDistance.z);

    float mixVal;
    if(d < LineWidth - 1) {
        mixVal = 1.0;
    } else if(d > LineWidth + 1) {
        mixVal = 0.0;
    } else {
        float x = d - (LineWidth - 1);
        mixVal = exp2(-2.0 * (x * x));
    }
    FragColor = mix(color, LineColor, mixVal);
}
