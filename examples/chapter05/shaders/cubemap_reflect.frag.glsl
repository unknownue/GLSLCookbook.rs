
#version 410

layout (location = 0) in vec3 ReflectDir;

layout (location = 0) out vec4 FragColor;

uniform samplerCube CubeMapTex;

uniform MaterialInfo {
    vec4 MaterialColor;
    float ReflectFactor;
};


void main() {

    // Access the cube map texture
    vec3 cubeMapColor = texture(CubeMapTex, ReflectDir).rgb;
    // Gamma correct
    cubeMapColor = pow(cubeMapColor, vec3(1.0 / 2.2));
    FragColor = vec4(mix(MaterialColor.rgb, cubeMapColor, ReflectFactor), 1.0);
}
