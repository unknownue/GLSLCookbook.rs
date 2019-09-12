
#version 410

uniform sampler2D ShadowTex;

layout (location = 0) in vec2 TexCoord;

layout (location = 0) out vec4 FragColor;

void main() {

    vec2 tex_coord = vec2(TexCoord.x, 1.0 - TexCoord.y);
    float d = texture(ShadowTex, tex_coord).r;
    FragColor = vec4(d);
}
