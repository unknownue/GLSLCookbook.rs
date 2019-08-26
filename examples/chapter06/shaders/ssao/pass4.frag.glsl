
#version 410

layout (location = 0) in vec2 TexCoord;

layout (location = 0) out vec4 FragColor;


uniform LightInfo {
    vec4 LightPosition;  // Light position in eye coords.
    vec3 L;   // D,S intensity
    vec3 La;  // Ambient
};

uniform sampler2D PositionTex;
uniform sampler2D NormalTex;
uniform sampler2D ColorTex;
uniform sampler2D AoTex;


vec3 ambAndDiffuse(vec3 pos, vec3 norm, vec3 diff, float ao) {

    ao = pow(ao, 4);
    vec3 ambient = La * diff * ao;
    vec3 s = normalize(vec3(LightPosition) - pos);
    float sDotN = max(dot(s, norm), 0.0);
    return ambient + L * diff * sDotN;
}

// Final color pass
void main() {

    // Retrieve position and normal information from textures
    vec3 position  = texture(PositionTex, TexCoord).xyz;
    vec3 normal    = texture(NormalTex, TexCoord).xyz;
    vec3 diffColor = texture(ColorTex, TexCoord).rgb;
    float aoVal    = texture(AoTex, TexCoord).r;

    vec3 color = ambAndDiffuse(position, normal, diffColor, aoVal);
    color = pow(color, vec3(1.0 / 2.2));

    FragColor = vec4(color, 1.0);
}
