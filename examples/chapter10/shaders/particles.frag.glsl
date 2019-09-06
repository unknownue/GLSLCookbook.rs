
#version 410

layout (location = 0) in float Transp;
layout (location = 1) in vec2 TexCoord;

layout (location = 0) out vec4 FragColor;

uniform sampler2D ParticleTex;

void main() {
    
    FragColor = texture(ParticleTex, TexCoord);
    FragColor.a *= Transp;
}
