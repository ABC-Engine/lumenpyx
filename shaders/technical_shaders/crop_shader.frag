#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D image;
uniform vec2 image_res;
uniform vec2 screen_res;

void main() {
    // take the center of the image
    // crop the image to the screen size
    vec2 uv = v_tex_coords;

    vec2 resolution_diff = image_res - screen_res;

    vec2 offset = resolution_diff / image_res / 2.0;

    uv = uv * screen_res / image_res + offset;

    color = texture(image, uv);
}