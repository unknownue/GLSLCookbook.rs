
#version 410

layout(location = 0) out vec4 FragColor;

uniform sampler2D RenderTex;
uniform float EdgeThreshold;

const vec3 lum = vec3(0.2126, 0.7152, 0.0722);

float luminance(vec3 color) {
    return dot(lum, color);
}


void main() {

    ivec2 pix = ivec2(gl_FragCoord.xy);

    float s00 = luminance(texelFetchOffset(RenderTex, pix, 0, ivec2(-1,  1)).rgb);
    float s10 = luminance(texelFetchOffset(RenderTex, pix, 0, ivec2(-1,  0)).rgb);
    float s20 = luminance(texelFetchOffset(RenderTex, pix, 0, ivec2(-1, -1)).rgb);
    float s01 = luminance(texelFetchOffset(RenderTex, pix, 0, ivec2( 0,  1)).rgb);
    float s21 = luminance(texelFetchOffset(RenderTex, pix, 0, ivec2( 0, -1)).rgb);
    float s02 = luminance(texelFetchOffset(RenderTex, pix, 0, ivec2( 1,  1)).rgb);
    float s12 = luminance(texelFetchOffset(RenderTex, pix, 0, ivec2( 1,  0)).rgb);
    float s22 = luminance(texelFetchOffset(RenderTex, pix, 0, ivec2( 1, -1)).rgb);

    float sx = s00 + 2 * s10 + s20 - (s02 + 2 * s12 + s22);
    float sy = s00 + 2 * s01 + s02 - (s20 + 2 * s21 + s22);

    float g = sx * sx + sy * sy;

    if(g > EdgeThreshold) {
        FragColor = vec4(1.0);
    } else {
        FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
}
