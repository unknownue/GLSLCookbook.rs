
#version 410

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexNormal;
layout (location = 2) in vec2 VertexTexCoord;

layout (location = 0) out vec4 Position;
layout (location = 1) out vec3 Normal;
layout (location = 2) out vec2 TexCoord;

uniform float Time;
uniform float Freq = 2.5;
uniform float Velocity = 2.5;
uniform float Amp = 0.6;

uniform mat4 ModelViewMatrix;
uniform mat3 NormalMatrix;
uniform mat4 MVP;

void main() {

    vec4 pos = vec4(VertexPosition, 1.0);

    float u = Freq * pos.x - Velocity * Time;
    pos.y = Amp * sin(u);

    vec3 n = vec3(0.0);
    n.xy = normalize(vec2(cos(u), 1.0));

    Position    = ModelViewMatrix * pos;
    Normal      = NormalMatrix * n;
    TexCoord    = VertexTexCoord;
    gl_Position = MVP * pos;
}
