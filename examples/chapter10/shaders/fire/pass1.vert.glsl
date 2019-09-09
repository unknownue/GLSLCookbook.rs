
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
uniform vec3 Emitter = vec3(0);  // Emitter position in world coordinates
uniform mat3 EmitterBasis;       // Rotation that rotates y axis to the direction of emitter

uniform sampler1D RandomTex;

vec3 randomInitialVelocity() {
    float velocity = mix(0.1, 0.5, texelFetch(RandomTex, 2 * gl_VertexID, 0).r);
    return EmitterBasis * vec3(0.0, velocity, 0.0);
}

vec3 randomInitialPosition() {
    float offset = mix(-2.0, 2.0, texelFetch(RandomTex, 2 * gl_VertexID + 1, 0).r);
    return Emitter + vec3(offset, 0.0, 0.0);
}

// Update
void main() {

    Age = VertexAge + DeltaT;

    if (VertexAge < 0.0 || VertexAge > ParticleLifeTime) {
        // The particle is past its lifetime (or not born yet)
        Position = randomInitialPosition();
        Velocity = randomInitialVelocity();

        if (VertexAge > ParticleLifeTime)
            Age = (VertexAge - ParticleLifeTime) + DeltaT;
    } else {
        // The particle is alive, update
        Position = VertexPosition + VertexVelocity * DeltaT;
        Velocity = VertexVelocity + Accel * DeltaT;
    }
}
