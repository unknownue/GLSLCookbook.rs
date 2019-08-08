
#version 410

layout (location = 0) in vec3 LightDir;
layout (location = 1) in vec2 TexCoord;
layout (location = 2) in vec3 ViewDir;

uniform sampler2D ColorTex;
uniform sampler2D NormalMapTex;
uniform sampler2D HeightMapTex;

uniform LightInfo {
    vec4 LightPosition;  // Light position in cam. coords.
    vec3 L;              // D,S intensity
    vec3 La;             // Amb intensity
};

uniform MaterialInfo {
    vec3 Ks;            // Specular reflectivity
    float Shininess;    // Specular shininess factor
};

layout (location = 0) out vec4 FragColor;


vec3 blinnPhong() {

    // After interpolation, the input attributes are probably not normalized.
    vec3 v = normalize(ViewDir);
    vec3 s = normalize(LightDir);

    const float bumpFactor = 0.009;
    float height = 1 - texture(HeightMapTex, TexCoord).r;
    vec2 delta = vec2(v.x, v.y) * height * bumpFactor / v.z;
    vec2 tc = TexCoord.xy - delta;
    //tc = TexCoord.xy;

    vec3 n = texture(NormalMapTex, tc).xyz;
    n.xy = 2.0 * n.xy - 1.0;
    n  = normalize(n);

    float sDotN = max(dot(s, n), 0.0);

    vec3 texColor = texture(ColorTex, tc).rgb;
    vec3 ambient = La * texColor;
    vec3 diffuse = texColor * sDotN;
    vec3 spec = vec3(0.0);

    if(sDotN > 0.0) {
        vec3 h = normalize(v + s);
        spec = Ks * pow(max(dot(h, n), 0.0), Shininess);
    }
    return ambient + L * (diffuse + spec);
}

void main() {

    vec3 c = blinnPhong();
    c = pow(c, vec3(1.0 / 2.2));
    FragColor = vec4(c, 1.0);
}
