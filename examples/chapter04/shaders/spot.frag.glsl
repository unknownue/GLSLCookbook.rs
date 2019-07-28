
#version 410

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;

uniform SpotLightInfo {
    vec3 SpotPosition;   // Light position in eye coords.
    vec3 L;              // Diffuse/specular light intensity
    vec3 La;             // Ambient light intensity
    vec3 SpotDirection;  // Direction of the spotlight in eye coords.
    float Exponent;      // Angular attenuation exponent
    float Cutoff;        // Cutoff angle (between 0 and pi/2)
};

uniform MaterialInfo {
    vec3 Ka;            // Ambient reflectivity
    vec3 Kd;            // Diffuse reflectivity
    vec3 Ks;            // Specular reflectivity
    float Shininess;    // Specular shininess factor
};

layout(location = 0) out vec4 FragColor;


vec3 blinnphongSpot(vec3 position, vec3 n) {

    vec3 ambient = La * Ka;
    vec3 diffuse = vec3(0.0);
    vec3 spec = vec3(0.0);

    vec3 s = normalize(SpotPosition - position);

    float cos_angle = dot(-s, normalize(SpotDirection));
    float angle = acos(cos_angle);

    float spotScale = 0.0;
    if (angle >= 0.0 && angle < Cutoff) {
        spotScale = pow(cos_angle, Exponent);

        float sDotN = max(dot(s, n), 0.0);
        diffuse = Kd * sDotN;
        if (sDotN > 0.0) {
            vec3 v = normalize(-position.xyz);
            vec3 h = normalize(v + s);
            spec = Ks * pow(max(dot(h, n), 0.0), Shininess);
        }
    }

    return ambient + spotScale * L * (diffuse + spec);
}

void main() {

    FragColor = vec4(blinnphongSpot(Position, normalize(Normal)), 1.0);
}
