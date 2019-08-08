
#version 410

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexNormal;
layout (location = 2) in vec2 VertexTexCoord;
layout (location = 3) in vec4 VertexTangent;

uniform LightInfo {
    vec4 LightPosition;  // Light position in cam. coords.
    vec3 L;              // D,S intensity
    vec3 La;             // Amb intensity
};

layout (location = 0) out vec3 LightDir;
layout (location = 1) out vec2 TexCoord;
layout (location = 2) out vec3 ViewDir;

uniform mat4 ModelViewMatrix;
uniform mat3 NormalMatrix;
uniform mat4 MVP;

void main() {

    // Transform normal and tangent to eye space
    vec3 norm = normalize(NormalMatrix * VertexNormal);
    vec3 tang = normalize(NormalMatrix * vec3(VertexTangent));
    // Compute the binormal
    vec3 binormal = normalize(cross(norm, tang)) * VertexTangent.w;

    // Matrix for transformation to tangent space(also called TBN matrix)
    mat3 toObjectLocal = mat3(
        tang.x, binormal.x, norm.x,
        tang.y, binormal.y, norm.y,
        tang.z, binormal.z, norm.z);

    // Transform light direction and view direction(camera space) to tangent space
    vec3 pos = vec3(ModelViewMatrix * vec4(VertexPosition, 1.0));
    LightDir = toObjectLocal * (LightPosition.xyz - pos);

    ViewDir = toObjectLocal * normalize(-pos);

    TexCoord = VertexTexCoord;

    gl_Position = MVP * vec4(VertexPosition, 1.0);
}
