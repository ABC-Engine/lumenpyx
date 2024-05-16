#version 140 core

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D target_fill;
uniform vec4 color_fill;

void main() {
    if (texture(target_fill, v_tex_coords).a > 0.0) {
        color = vec4(color_fill.xyz, color_fill.a * texture(target_fill, v_tex_coords).a);
    } else {
        color = vec4(0.0, 0.0, 0.0, 0.0);
    }
}