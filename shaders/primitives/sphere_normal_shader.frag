#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform float radius_squared;

void main() {
    float x = v_tex_coords.x - 0.5;
    float y = v_tex_coords.y - 0.5;
    float distance_squared = x * x + y * y;

    if (distance_squared < radius_squared) {
        float dydz = -(y / sqrt(radius_squared - y*y));
        float dxdz = -(x / sqrt(radius_squared - x*x));
        vec3 normal = normalize(vec3(dxdz, dydz, 1.0));
        color = vec4(normal, 1.0); // Assign vec3 normal to vec4 color
    } else {
        discard;
    }
}