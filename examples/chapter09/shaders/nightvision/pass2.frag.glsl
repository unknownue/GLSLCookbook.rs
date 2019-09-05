
#version 410

layout (location = 0) in vec2 TexCoord;

layout (location = 0) out vec4 FragColor;

uniform int Width;
uniform int Height;
uniform float Radius;
uniform sampler2D RenderTex;
uniform sampler2D NoiseTex;

float luminance(vec3 color) {
    return dot(color.rgb, vec3(0.2126, 0.7152, 0.0722));
}

void main() {

    vec4 noise = texture(NoiseTex,  TexCoord);
    vec4 color = texture(RenderTex, TexCoord);
    float green = luminance(color.rgb);

    float dist1 = length(gl_FragCoord.xy - vec2(      Width / 4.0, Height / 2.0));
    float dist2 = length(gl_FragCoord.xy - vec2(3.0 * Width / 4.0, Height / 2.0));

    if (dist1 > Radius && dist2 > Radius)
        FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    else
        FragColor = vec4(0.0, green * clamp(noise.a, 0.0, 1.0), 0.0, 1.0);
}
