
#version 410

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 TexCoord;

uniform sampler2D BaseTex;
uniform sampler2D AlphaTex;

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

vec3 blinnPhong(vec3 position, vec3 n) {

    vec3 texColor = texture(BaseTex, TexCoord).rgb;

    vec3 ambient = La * texColor;
    vec3 s = normalize(LightPosition.xyz - position);
    float sDotN = max(dot(s, n), 0.0);
    vec3 diffuse = texColor * sDotN;

    vec3 spec = vec3(0.0);
    if(sDotN > 0.0) {
        vec3 v = normalize(-position.xyz);
        vec3 h = normalize(v + s);
        spec = Ks * pow(max(dot(h, n), 0.0), Shininess);
    }
    return ambient + L * (diffuse + spec);
}

void main() {

    vec4 alphaMap = texture(AlphaTex, TexCoord);

    if(alphaMap.a < 0.15) {
        discard;
    } else {
        if(gl_FrontFacing) {
            FragColor = vec4(blinnPhong(Position, normalize(Normal)), 1.0);
        } else {
            FragColor = vec4(blinnPhong(Position, normalize(-Normal)), 1.0);
        }
    }
}
