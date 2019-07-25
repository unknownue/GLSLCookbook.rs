
#version 410

layout (location = 0) in vec3 FrontColor;
layout (location = 1) in vec3 BackColor;
layout (location = 2) in vec2 TexCoord;

layout (location = 0) out vec4 FragColor;

void main() {

    const float scale = 15.0;

    bvec2 toDiscard = greaterThan(fract(TexCoord * scale), vec2(0.2, 0.2));

    if(all(toDiscard))
        discard;
    else {
        if(gl_FrontFacing)
            FragColor = vec4(FrontColor, 1.0);
        else
            FragColor = vec4(BackColor, 1.0);
    }
}
