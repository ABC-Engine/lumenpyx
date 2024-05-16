#version 140 core

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D image;

void main() {
    color = texture(image, v_tex_coords);
}