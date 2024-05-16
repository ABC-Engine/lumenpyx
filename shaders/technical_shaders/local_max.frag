#version 140 core

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D high_res_image;
// the resolution of the low res image an int
uniform uvec2 new_resolution;

void main() {
    // collect all colors that are in the pixel of the low res image in the high res image
    vec4 new_color = vec4(0.0, 0.0, 0.0, 0.0);
    vec2 high_res_resolution = textureSize(high_res_image, 0);

    vec2 pixel_size = high_res_resolution / new_resolution;

    for (int i = 0; i < pixel_size.x; i++) {
        for (int j = 0; j < pixel_size.y; j++) {
            vec2 new_coords = v_tex_coords + (vec2(i, j) / high_res_resolution);

            if(texture(high_res_image, new_coords).r > new_color.r) {
                new_color = texture(high_res_image, new_coords);
            }
        }
    }

    color = new_color;
}