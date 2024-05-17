#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D bottom_image;
uniform sampler2D top_image;

// only one of these will be true
uniform bool add;
uniform bool subtract;
uniform bool multiply;
uniform bool divide;

void main() {
    vec4 bottom_color = texture(bottom_image, v_tex_coords);
    vec4 top_color = texture(top_image, v_tex_coords);
    if (top_color.a == 0.0 || bottom_color.a == 0.0) {
        color = bottom_color;
        return;
    }

    if(add) {
        color = bottom_color + top_color;
    } else if(subtract) {
        color = bottom_color - top_color;
    } else if(multiply) {
        color = bottom_color * top_color;
    } else if(divide) {
        color = bottom_color / top_color;
    }
    else {
        // should never happen, but if it does it should be obvious
        color = vec4(1.0, 0.0, 1.0, 1.0);
    }
}