
#version 410

uniform sampler2D Texture0;

uniform WeightBlock {
    float Weight[5];
};

layout(location = 0) out vec4 FragColor;

void main() {

ivec2 pix = ivec2(gl_FragCoord.xy);

    vec4 sum = texelFetch(Texture0, pix, 0) * Weight[0];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2( 1, 0)) * Weight[1];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(-1, 0)) * Weight[1];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2( 2, 0)) * Weight[2];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(-2, 0)) * Weight[2];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2( 3, 0)) * Weight[3];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(-3, 0)) * Weight[3];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2( 4, 0)) * Weight[4];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(-4, 0)) * Weight[4];

    FragColor = sum;
}
