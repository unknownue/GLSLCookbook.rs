
#version 410

layout (location = 0) noperspective in vec3 EdgeDistance;

layout (location = 0) out vec4 FragColor;

uniform float LineWidth;
uniform vec4 LineColor;
uniform vec4 QuadColor;


float edgeMix() {
    
    // Find the smallest distance
    float d = min(min(EdgeDistance.x, EdgeDistance.y), EdgeDistance.z);

    if(d < LineWidth - 1.0) {
        return 1.0;
    } else if(d > LineWidth + 1) {
        return 0.0;
    } else {
        float x = d - (LineWidth - 1.0);
        return exp2(-2.0 * (x * x));
    }
}

void main() {

    float mixVal = edgeMix();

    FragColor = mix(QuadColor, LineColor, mixVal);
}
