
#version 410

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;

uniform int Pass;
uniform sampler2D Texture0;

// The `Weight` failed to set on macOS.
uniform WeightBlock {
    float Weight[5];
};

// Here we use pre-calculate constants to ease this problem
const float Pre_Weights[5] = float[5](0.158435, 0.148836, 0.123389, 0.0902733, 0.0582848);


uniform LightInfo {
    vec4 LightPosition;  // Light position in cam. coords.
    vec3 L;              // D,S intensity
    vec3 La;             // Amb intensity
};

uniform MaterialInfo {
    vec3 Ka;            // Ambient reflectivity
    vec3 Kd;            // Diffuse reflectivity
    vec3 Ks;            // Specular reflectivity
    float Shininess;    // Specular shininess factor
};

layout(location = 0) out vec4 FragColor;


vec3 blinnPhong(vec3 position, vec3 n) {

    vec3 ambient = La * Ka;
    vec3 s = normalize(LightPosition.xyz - position);
    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = Kd * sDotN;

    vec3 spec = vec3(0.0);
    if(sDotN > 0.0) {
        vec3 v = normalize(-position.xyz);
        vec3 h = normalize(v + s);
        spec = Ks * pow(max(dot(h, n), 0.0), Shininess);
    }
    return ambient + L * (diffuse + spec);
}

vec4 pass1() {
    return vec4(blinnPhong(Position, normalize(Normal)), 1.0);
}

vec4 pass2() {
    ivec2 pix = ivec2(gl_FragCoord.xy);

    vec4 sum = texelFetch(Texture0, pix, 0) * Pre_Weights[0];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0,  1)) * Pre_Weights[1];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0, -1)) * Pre_Weights[1];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0,  2)) * Pre_Weights[2];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0, -2)) * Pre_Weights[2];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0,  3)) * Pre_Weights[3];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0, -3)) * Pre_Weights[3];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0,  4)) * Pre_Weights[4];
    sum += texelFetchOffset(Texture0, pix, 0, ivec2(0, -4)) * Pre_Weights[4];

    // vec4 sum = texelFetch(Texture0, pix, 0) * Weight[0];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2(0,  1)) * Weight[1];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2(0, -1)) * Weight[1];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2(0,  2)) * Weight[2];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2(0, -2)) * Weight[2];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2(0,  3)) * Weight[3];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2(0, -3)) * Weight[3];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2(0,  4)) * Weight[4];
    // sum += texelFetchOffset(Texture0, pix, 0, ivec2(0, -4)) * Weight[4];
    
    return sum;
}

vec4 pass3() {
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

    return sum;
}

void main() {

    FragColor = vec4(0.0);
    if(Pass == 1) FragColor = pass1();
    if(Pass == 2) FragColor = pass2();
    if(Pass == 3) FragColor = pass3();
}
