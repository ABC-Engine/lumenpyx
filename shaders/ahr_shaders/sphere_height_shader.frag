#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform float radius;

void main() {
    float x_coord = v_tex_coords.x - 0.5;
    float y_coord = v_tex_coords.y - 0.5;
    float dist = length(vec2(x_coord, y_coord));
    
    if (dist < radius) {
        float z = sqrt(-x_coord*x_coord - y_coord*y_coord + radius*radius);
        color = vec4(z, z, z, 1.0);
    } else {
        discard;
    }
}