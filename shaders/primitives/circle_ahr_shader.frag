#version 140 core

in vec2 v_tex_coords;
out vec4 color;

uniform vec4 circle_color;
uniform float radius_squared;

void main() {
    float x = v_tex_coords.x - 0.5;
    float y = v_tex_coords.y - 0.5;
    float distance_squared = x * x + y * y;
    if (distance_squared < radius_squared) {
        color = circle_color;
    } else {
        discard;
    }
}