
#version 410

layout(location = 0) in vec3 FrontColor;
layout(location = 1) in vec3 BackColor;

layout(location = 0) out vec4 FragColor;

void main() {

    if(gl_FrontFacing) {
        FragColor = vec4(FrontColor, 1.0);
    } else {
        //FragColor = mix(vec4(BackColor, 1.0), vec4(1.0,0.0,0.0,1.0), 0.7);
        FragColor = vec4(BackColor, 1.0);
    }
}
