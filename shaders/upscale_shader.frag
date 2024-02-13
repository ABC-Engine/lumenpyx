#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D image;

void main() {
    vec4 new_color = texture(image, v_tex_coords);
    color = new_color;
}