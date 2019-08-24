
#version 410

layout (location = 0) in vec2 TexCoord;

layout (location = 0) out vec4 FragColor;

uniform sampler2D BlurTex2;


uniform float PixOffset[10] = float[](0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
uniform float Pre_Weight[10] = float[](0.084612, 0.08293, 0.07810, 0.070674, 0.061441, 0.051320, 0.04118, 0.03175, 0.023525, 0.016744);

uniform WeightBlock {
    float Weight[10];
};

// Second blur (read from BlurTex2, write to BlurTex1) 
void main() {
    float dx = 1.0 / (textureSize(BlurTex2, 0)).x;

    vec4 sum = texture(BlurTex2, TexCoord) * Pre_Weight[0];
    for(int i = 1; i < 10; i++) {
       sum += texture(BlurTex2, TexCoord + vec2(PixOffset[i], 0.0) * dx) * Pre_Weight[i];
       sum += texture(BlurTex2, TexCoord - vec2(PixOffset[i], 0.0) * dx) * Pre_Weight[i];
    }

    FragColor = sum;
}
