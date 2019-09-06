
#version 410

layout (location = 0) in vec3  VertexInitVel;   // Particle initial velocity
layout (location = 1) in float VertexBirthTime; // Particle birth time

layout (location = 0) out float Transp;   // Transparency of the particle
layout (location = 1) out vec2  TexCoord; // Texture coordinate

uniform float Time;                            // Animation time
uniform float ParticleLiftTime;                // Max particle lifetime
uniform float ParticleSize = 0.05;             // Particle size
uniform vec3 Gracity = vec3(0.0, -0.2, 0.0);   // Gravity vector in world coords
uniform vec3 EmitterPos;                       // Emiter position in world coordinates

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

void main() {

    vec3 cameraPos;  // Position in camera coordinates

    float t = Time - VertexBirthTime;
    if (t >= 0 && t < ParticleLiftTime) {
        vec3 pos = EmitterPos + VertexInitVel * t + Gracity * t * t;
        cameraPos = (ModelViewMatrix * vec4(pos, 1.0)).xyz + (offsets[gl_VertexID] * ParticleSize);
        Transp = mix(1.0, 0.0, t / ParticleLiftTime);
    } else {
        // Particle doesn't "exist", draw fully transparent
        cameraPos = vec3(0.0);
        Transp = 0.0;
    }

    TexCoord = texCoords[gl_VertexID];

    gl_Position = ProjectionMatrix * vec4(cameraPos, 1.0);
}
