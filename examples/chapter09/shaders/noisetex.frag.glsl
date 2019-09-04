
#version 410

layout (location = 0) in vec2 TexCoord;

layout (location = 0) out vec4 FragColor;

uniform sampler2D NoiseTex;

void main() {

    vec4 noise = texture(NoiseTex, TexCoord);
    FragColor = vec4(noise.aaa, 1.0);
}
