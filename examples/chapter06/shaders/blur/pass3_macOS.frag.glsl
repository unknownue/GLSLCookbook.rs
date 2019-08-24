
#version 410

uniform sampler2D Texture0;

// The `Weight` failed to set on macOS.
uniform WeightBlock {
    float Weight[5];
};

// Here we use pre-calculate constants to ease this problem
const float Pre_Weights[5] = float[5](0.158435, 0.148836, 0.123389, 0.0902733, 0.0582848);


layout(location = 0) out vec4 FragColor;

void main() {

    ivec2 pix = ivec2(gl_FragCoord.xy);

    vec4 sum = texelFetch(Texture0, pix, 0) * Pre_Weights[0];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2( 1, 0)) * Pre_Weights[1];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(-1, 0)) * Pre_Weights[1];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2( 2, 0)) * Pre_Weights[2];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(-2, 0)) * Pre_Weights[2];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2( 3, 0)) * Pre_Weights[3];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(-3, 0)) * Pre_Weights[3];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2( 4, 0)) * Pre_Weights[4];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(-4, 0)) * Pre_Weights[4];

    // vec4 sum = texelFetch(Texture0, pix, 0) * Weight[0];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2( 1, 0)) * Weight[1];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2(-1, 0)) * Weight[1];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2( 2, 0)) * Weight[2];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2(-2, 0)) * Weight[2];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2( 3, 0)) * Weight[3];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2(-3, 0)) * Weight[3];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2( 4, 0)) * Weight[4];
        // sum += texelFetchOffset(Texture0, pix, 0, ivec2(-4, 0)) * Weight[4];

    FragColor = sum;
}
