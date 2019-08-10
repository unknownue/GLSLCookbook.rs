
#version 410

layout (location = 0) in vec3 ReflectDir;
layout (location = 1) in vec3 RefractDir;

uniform samplerCube CubeMapTex;

uniform MaterialInfo {
    float Eta;              // Index of refraction
    float ReflectionFactor; // Percentage of reflected light
};

layout(location = 0) out vec4 FragColor;

void main() {

    // Access the cube map texture
    vec3 reflectColor = texture(CubeMapTex, ReflectDir).rgb;
    vec3 refractColor = texture(CubeMapTex, RefractDir).rgb;

    vec3 color = mix(refractColor, reflectColor, ReflectionFactor);
    // Gamma
    color = pow(color, vec3(1.0 / 2.2));

    FragColor = vec4(color, 1.0);
}
