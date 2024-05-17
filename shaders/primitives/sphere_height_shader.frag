#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform float radius_squared;

void main() {
    float x = v_tex_coords.x - 0.5;
    float y = v_tex_coords.y - 0.5;
    float distance_squared = x * x + y * y;

    if (distance_squared < radius_squared) {
        float z = sqrt(-x*x - y*y + radius_squared);
        color = vec4(z, z, z, 1.0);
    } else {
        discard;
    }
}