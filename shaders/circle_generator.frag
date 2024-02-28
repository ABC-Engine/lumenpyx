#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform vec4 color;
uniform float radius;

void main() {
    float dist = length(v_tex_coords - vec2(0.5, 0.5));
    if (dist < radius) {
        color.a = 1.0;
    } else {
        color.a = 0.0;
    }
    color.rgb = vec3(1.0, 1.0, 1.0);
}