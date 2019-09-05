
#version 410

layout (location = 0) in vec3 ReflectDir;
layout (location = 1) in vec2 TexCoord;

layout (location = 0) out vec4 FragColor;

uniform samplerCube CubeMapTex;
uniform sampler2D NoiseTex;

uniform vec4 MaterialColor;
uniform float ReflectFactor;

uniform float Threshold = 0.58;


void main() {

    // Access the noise texture
    float noise = texture(NoiseTex, TexCoord).a;
    float scale = floor(noise + (1.0 - Threshold));

    // Access the cube map texture
    vec3 cubeMapColor = texture(CubeMapTex, ReflectDir).rgb;

    // Gamma correct
    cubeMapColor = pow(cubeMapColor, vec3(1.0 / 2.2));

    vec3 rustColor = mix(MaterialColor.rgb, vec3(0.01), noise);

    FragColor = vec4(mix(cubeMapColor, rustColor, scale), 1.0);
}
