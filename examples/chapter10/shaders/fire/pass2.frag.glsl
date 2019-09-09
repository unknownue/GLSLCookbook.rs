
#version 410

layout (location = 0) in float Transp;
layout (location = 1) in vec2 TexCoord;

layout (location = 0) out vec4 FragColor;

uniform sampler2D ParticleTex;

void main() {
    
    FragColor = texture(ParticleTex, TexCoord);
    // Mix with black as it gets older, to simulate a bit of somke
    FragColor = vec4(mix(vec3(0.0), FragColor.xyz, Transp), FragColor.a);
    FragColor.a *= Transp;
}
