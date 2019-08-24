
#version 410

uniform sampler2D Texture0;

uniform WeightBlock {
    float Weight[5];
};

layout(location = 0) out vec4 FragColor;

void main() {

    ivec2 pix = ivec2(gl_FragCoord.xy);

    vec4 sum = texelFetch(Texture0, pix, 0) * Weight[0];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0,  1)) * Weight[1];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0, -1)) * Weight[1];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0,  2)) * Weight[2];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0, -2)) * Weight[2];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0,  3)) * Weight[3];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0, -3)) * Weight[3];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0,  4)) * Weight[4];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0, -4)) * Weight[4];

    FragColor = sum;
}
