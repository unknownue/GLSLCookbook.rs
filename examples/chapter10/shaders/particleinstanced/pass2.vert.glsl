
#version 410

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexNormal;
layout (location = 2) in vec2 VertexTexCoord;

layout (location = 3) in vec3 ParticlePosition;
layout (location = 4) in vec3 ParticleVelocity;
layout (location = 5) in float ParticleAge;
layout (location = 6) in vec2 ParticleRotation;

// To fragment shader
layout (location = 0) out vec3 fPosition;
layout (location = 1) out vec3 fNormal;


// Transforms
uniform mat4 ModelViewMatrix;   // View * Model
uniform mat4 ProjectionMatrix;  // ProjectionMatrixection matrix


void main() {

    float cs = cos(ParticleRotation.x);
    float sn = sin(ParticleRotation.x);

    mat4 rotationAndTranslation = mat4(
        1, 0, 0, 0,
        0, cs, sn, 0,
        0, -sn, cs, 0,
        ParticlePosition.x, ParticlePosition.y, ParticlePosition.z, 1
    );

    mat4 m = ModelViewMatrix * rotationAndTranslation;
    fPosition = (m * vec4(VertexPosition, 1.0)).xyz;
    fNormal   = (m * vec4(VertexNormal,   0.0)).xyz;

    // Draw at the current position
    gl_Position = ProjectionMatrix * vec4(fPosition, 1.0);
}
