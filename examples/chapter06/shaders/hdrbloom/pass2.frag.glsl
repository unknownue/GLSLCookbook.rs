
#version 410

layout (location = 0) in vec2 TexCoord;

layout (location = 0) out vec4 FragColor;

uniform sampler2D HdrTex;

uniform float LumThresh;

float luminance(vec3 color) {
    return 0.2126 * color.r + 0.7152 * color.g + 0.0722 * color.b;
}

// Bright-pass filter (write to BlurTex1)
void main() {

    vec4 val = texture(HdrTex, TexCoord);
    if(luminance(val.rgb) > LumThresh) {
        FragColor = val;
    } else {
        FragColor = vec4(0.0);
    }
}
