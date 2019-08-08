
#version 410

layout (location = 0) in vec3 LightDir;
layout (location = 1) in vec2 TexCoord;
layout (location = 2) in vec3 ViewDir;

uniform sampler2D ColorTex;
uniform sampler2D NormalMapTex;

uniform LightInfo {
    vec4 LightPosition;  // Light position in cam. coords.
    vec3 L;              // D,S intensity
    vec3 La;             // Amb intensity
};

uniform MaterialInfo {
    vec3 Ks;            // Specular reflectivity
    float Shininess;    // Specular shininess factor
};

layout(location = 0) out vec4 FragColor;

vec3 blinnPhong(vec3 n) {

    vec3 texColor = texture(ColorTex, TexCoord).rgb;

    vec3 ambient = La * texColor;
    vec3 s = normalize(LightDir);
    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = texColor * sDotN;

    vec3 spec = vec3(0.0);
    if(sDotN > 0.0) {
        vec3 v = normalize(ViewDir);
        vec3 h = normalize(v + s);
        spec = Ks * pow(max(dot(h, n), 0.0), Shininess);
    }
    return ambient + L * (diffuse + spec);
}

void main() {

    // Lookup the normal from the normal map
    // vec3 normal = 2.0 * texture( NormalMapTex, TexCoord ).xyz - 1.0;
    // FragColor = vec4( blinnPhong(normal), 1.0 );

    vec3 norm = texture(NormalMapTex, TexCoord).xyz;

    // Textures store values that range in [0, 1].
    // Normal vectors have components that range in [-1, 1], so we need to re-scale the value.
    //
    // For some normal maps, the zcoordinate is never negative because in tangent space that would correspond to a normal that points into the surface
    // Here we assume that z ranges in [0, 1], and use the the full resolution of the channel for that range.
    // There is no standard convention for z coordinate.
    norm.xy = 2.0 * norm.xy - 1.0;

    FragColor = vec4(blinnPhong(norm), 1.0);
}
