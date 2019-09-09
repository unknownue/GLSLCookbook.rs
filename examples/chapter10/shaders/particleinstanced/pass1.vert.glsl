
#version 410

layout (location = 0) in vec3 ParticlePosition;
layout (location = 1) in vec3 ParticleVelocity;
layout (location = 2) in float ParticleAge;
layout (location = 3) in vec2 ParticleRotation;

// To transform feedback
layout (location = 0) out vec3 Position;
layout (location = 1) out vec3 Velocity;
layout (location = 2) out float Age;
layout (location = 3) out vec2 Rotation;


const float PI = 3.14159265359;

uniform float Time;               // Simulation time
uniform float DeltaT;             // Elapsed time between frames
uniform float ParticleLifetime;   // Particle lifespan
uniform vec3 Accel;               // Particle acceleration (gravity)
uniform vec3 Emitter = vec3(0.0); // World position of the emitter.
uniform mat3 EmitterBasis;        // Rotation that rotates y axis to the direction of emitter

uniform sampler1D RandomTex;

vec3 randomInitialVelocity() {

    float theta    = mix(0.0, PI / 6.0, texelFetch(RandomTex, 4 * gl_VertexID,     0).r);
    float phi      = mix(0.0, PI * 2.0, texelFetch(RandomTex, 4 * gl_VertexID + 1, 0).r);
    float velocity = mix(1.25,     1.5, texelFetch(RandomTex, 4 * gl_VertexID + 2, 0).r);

    vec3 v = vec3(
        sin(theta) * cos(phi),
        cos(theta),
        sin(theta) * sin(phi)
    );

    return normalize(EmitterBasis * v) * velocity;
}

float randomInitialRotationalVelocity() {
    return mix(-15.0, 15.0, texelFetch(RandomTex, 4 * gl_VertexID + 3, 0).r);
}

void main() {

    if (ParticleAge < 0.0 || ParticleAge > ParticleLifetime) {
        // The particle is past it's lifetime, recycle.
        Position = Emitter;
        Velocity = randomInitialVelocity();
        Rotation = vec2(0.0, randomInitialRotationalVelocity());
        if (ParticleAge < 0.0)
            Age = ParticleAge + DeltaT;
        else
            Age = (ParticleAge - ParticleLifetime) + DeltaT;
    } else {
        // The particle is alive, update.
        Position = ParticlePosition + ParticleVelocity * DeltaT;
        Velocity = ParticleVelocity + Accel * DeltaT;
        Rotation.x = mod(ParticleRotation.x + ParticleRotation.y * DeltaT, 2.0 * PI);
        Rotation.y = ParticleRotation.y;
        Age = ParticleAge + DeltaT;
    }
}
