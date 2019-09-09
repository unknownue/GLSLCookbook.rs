
#version 410

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexVelocity;
layout (location = 2) in float VertexAge;

layout (location = 0) out vec3 Position;
layout (location = 1) out vec3 Velocity;
layout (location = 2) out float Age;


const float PI = 3.14159265359;

uniform float Time;              // Simulation time
uniform float DeltaT;            // Elapsed time between frames
uniform float ParticleLifeTime;  // Particle lifespan
uniform vec3 Accel;              // Particle acceleration (gravity)
uniform vec3 Emitter;            // Emitter position in world coordinates
uniform mat3 EmitterBasis;       // Rotation that rotates y axis to the direction of emitter

uniform sampler1D RandomTex;


vec3 randomInitialVelocity() {

    float theta    = mix(0.0,  PI / 1.5, texelFetch(RandomTex, 3 * gl_VertexID,     0).r);
    float phi      = mix(0.0,  PI * 2.0, texelFetch(RandomTex, 3 * gl_VertexID + 1, 0).r);
    float velocity = mix(0.1,       0.2, texelFetch(RandomTex, 3 * gl_VertexID + 2, 0).r);

    vec3 v = vec3(
        sin(theta) * cos(phi),
        cos(theta),
        sin(theta) * sin(phi)
    );

    return normalize(EmitterBasis * v) * velocity;
}

// Update
void main() {

    if (VertexAge < 0.0 || VertexAge > ParticleLifeTime) {
        // The particle is past its lifetime, recycle.
        Position = Emitter;
        Velocity = randomInitialVelocity();

        if (VertexAge < 0.0)
            Age = VertexAge + DeltaT;
        else
            Age = (VertexAge - ParticleLifeTime) + DeltaT;
    } else {
        // The particle is alive, update
        Position = VertexPosition + VertexVelocity * DeltaT;
        Velocity = VertexVelocity + Accel * DeltaT;
        Age = VertexAge + DeltaT;
    }
}
