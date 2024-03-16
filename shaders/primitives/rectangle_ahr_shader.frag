#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform float width;
uniform float height;
uniform vec4 rect_color;

void main() {
    float x = v_tex_coords.x;
    float y = v_tex_coords.y;
    // center the rectangle
    if (x < 0.5 - width / 2 || x > 0.5 + width / 2 || y < 0.5 - height / 2 || y > 0.5 + height / 2) {
        discard;
    } else {
        color = rect_color;
    }
}