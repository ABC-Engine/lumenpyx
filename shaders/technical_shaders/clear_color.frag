#version 140 core

in vec2 v_tex_coords;
out vec4 color;

uniform vec4 new_color;

void main() {
    color = new_color;
}