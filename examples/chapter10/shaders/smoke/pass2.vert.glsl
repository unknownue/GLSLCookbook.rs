
#version 410

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexVelocity;
layout (location = 2) in float VertexAge;

layout (location = 0) out float Transp;  // Transparency
layout (location = 1) out vec2 TexCoord; // Texture coordinate


uniform float ParticleLifeTime;       // Particle lifespan
uniform float MinParticleSize = 0.1;  // Minimum size
uniform float MaxParticleSize = 2.5;  // Maximum size

// Transformation matrices
uniform mat4 ModelViewMatrix;
uniform mat4 ProjectionMatrix;


// Offsets to the position in camera coordinates for each vertex of the particle's quad
const vec3 offsets[] = vec3[](
    vec3(-0.5, -0.5, 0.0),
    vec3( 0.5, -0.5, 0.0),
    vec3( 0.5,  0.5, 0.0),
    vec3(-0.5, -0.5, 0.0),
    vec3( 0.5,  0.5, 0.0),
    vec3(-0.5,  0.5, 0.0)
);

// Texture coordinates for each vertex of the particle's quad
const vec2 texCoords[] = vec2[](
    vec2(0.0, 0.0),
    vec2(1.0, 0.0),
    vec2(1.0, 1.0),
    vec2(0.0, 0.0),
    vec2(1.0, 1.0),
    vec2(0.0, 1.0)
);


// Render
void main() {

    Transp = 0.0;
    vec3 camera_pos = vec3(0.0);

    if (VertexAge >= 0.0) {
        float agePct = VertexAge / ParticleLifeTime;
        Transp = clamp(1.0 - agePct, 0.0, 1.0);
        camera_pos = (ModelViewMatrix * vec4(VertexPosition, 1.0)).xyz + offsets[gl_VertexID] * mix(MinParticleSize, MaxParticleSize, agePct);
    }

    TexCoord = texCoords[gl_VertexID];

    gl_Position = ProjectionMatrix * vec4(camera_pos, 1.0);
}
