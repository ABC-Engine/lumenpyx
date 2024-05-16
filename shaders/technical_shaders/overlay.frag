#version 140 core

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D top_tex;
uniform sampler2D bottom_tex;

void main() {
    vec4 top_color = texture(top_tex, v_tex_coords);
    vec4 bottom_color = texture(bottom_tex, v_tex_coords);
    color = mix(bottom_color, top_color, top_color.a);
}