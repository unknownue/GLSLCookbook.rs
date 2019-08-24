
#version 410

layout (location = 2) in vec2 TexCoord;

layout (location = 0) out vec4 FragColor;

uniform sampler2D HdrTex;
uniform sampler2D BlurTex1;

uniform float AveLum;

// XYZ/RGB conversion matrices from:
// https://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html

uniform mat3 rgb2xyz = mat3( 
    0.4124564, 0.2126729, 0.0193339,
    0.3575761, 0.7151522, 0.1191920,
    0.1804375, 0.0721750, 0.9503041
);

uniform mat3 xyz2rgb = mat3(
    3.2404542, -0.9692660, 0.0556434,
    -1.5371385, 1.8760108, -0.2040259,
    -0.4985314, 0.0415560, 1.0572252
);

uniform float Exposure = 0.35;
uniform float White    = 0.928;
 

// Composite pass, apply tone map to HDR image, then combine with the blurred bright-pass filter.
// (Read from BlurTex1 and HdrTex, write to default buffer).
void main() {

    /////////////// Tone mapping ///////////////
    // Retrieve high-res color from texture
    vec4 color = texture(HdrTex, TexCoord);

    // Convert to XYZ
    vec3 xyzCol = rgb2xyz * vec3(color);

    // Convert to xyY
    float xyzSum = xyzCol.x + xyzCol.y + xyzCol.z;
    vec3 xyYCol = vec3(xyzCol.x / xyzSum, xyzCol.y / xyzSum, xyzCol.y);

    // Apply the tone mapping operation to be the luminance (xyYcol.z or xyzCol.y)
    float L = (Exposure * xyYCol.z) / AveLum;
    L = (L * (1 + L / (White * White))) / (1 + L);

    // Using the new luminance, convert back to XYZ
    xyzCol.x = (L * xyYCol.x) / (xyYCol.y);
    xyzCol.y = L;
    xyzCol.z = (L * (1 - xyYCol.x - xyYCol.y)) / xyYCol.y;

    // Convert back to RGB and send to output buffer
    vec4 toneMapColor = vec4(xyz2rgb * xyzCol, 1.0);

    ///////////// Combine with blurred texture /////////////
    // We want linear filtering on this texture access so that
    // we get additional blurring.
    vec4 blurTex = texture(BlurTex1, TexCoord);

    // ivec2 blurSize = textureSize(BlurTex1, 0);
    // if(gl_FragCoord.x < blurSize.x && gl_FragCoord.y < blurSize.y)
    //     FragColor = texture(BlurTex1, vec2(gl_FragCoord.x / blurSize.x,
    //   gl_FragCoord.y / blurSize.y));
    // else
    FragColor = toneMapColor + blurTex;
}
