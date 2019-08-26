
#version 410

// out vec4 FragColor;
layout (location = 0) out float AoData;

uniform sampler2D AoTex;

// Blur pass
void main() {

    ivec2 pix = ivec2(gl_FragCoord.xy);
    float sum = 0.0;

    sum += texelFetchOffset(AoTex, pix, 0, ivec2(-1, -1)).r;
    sum += texelFetchOffset(AoTex, pix, 0, ivec2(-1,  0)).r;
    sum += texelFetchOffset(AoTex, pix, 0, ivec2(-1,  1)).r;
    sum += texelFetchOffset(AoTex, pix, 0, ivec2( 0, -1)).r;
    sum += texelFetchOffset(AoTex, pix, 0, ivec2( 0,  0)).r;
    sum += texelFetchOffset(AoTex, pix, 0, ivec2( 0,  1)).r;
    sum += texelFetchOffset(AoTex, pix, 0, ivec2( 1, -1)).r;
    sum += texelFetchOffset(AoTex, pix, 0, ivec2( 1,  0)).r;
    sum += texelFetchOffset(AoTex, pix, 0, ivec2( 1,  1)).r;

    // for(int x = -1; x <= 1; ++x) {
    //     for(int y = -1; y <= 1; y++) {
    //         sum += texelFetchOffset(AoTex, pix, 0, ivec2(x, y)).r;
    //     }
    // }

    float ao = sum * (1.0 / 9.0);
    AoData = ao;
    //FragColor = vec4(ao, ao, ao, 1);
}
