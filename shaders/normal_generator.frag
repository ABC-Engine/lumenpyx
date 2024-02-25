#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D heightmap;
// so that we can see if the albedo is invisible
uniform sampler2D albedomap;

const vec2 RESOLUTION = vec2(128.0, 128.0);

// this function is the same as the one in the lighting shader
vec4 texture_pixel(sampler2D tex, vec2 coords) {
    vec2 new_coords = coords / RESOLUTION;
    return texture(tex, new_coords);
}

void main() {
    if (texture(albedomap, v_tex_coords).a == 0.0) {
        color = vec4(0.0, 0.0, 0.0, 0.0);
        return;
    }

    vec4 center_color = texture(heightmap, v_tex_coords);
    if (center_color.a == 0.0) {
        color = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }
    float center_height = center_color.r;

    vec2 new_coords = v_tex_coords * RESOLUTION;

    float left_x_height = texture_pixel(heightmap, new_coords + vec2(-1.0, 0.0)).r;
    float left_x_slope = (center_height - left_x_height);

    float right_x_height = texture_pixel(heightmap, new_coords + vec2(1.0, 0.0)).r;
    float right_x_slope = (right_x_height - center_height);

    float top_y_height = texture_pixel(heightmap, new_coords + vec2(0.0, -1.0)).r;
    float top_y_slope = (center_height - top_y_height);

    float bottom_y_height = texture_pixel(heightmap, new_coords + vec2(0.0, 1.0)).r;
    float bottom_y_slope = (bottom_y_height - center_height);

    float x_slope = (left_x_slope + right_x_slope) / 2.0;
    float y_slope = (top_y_slope + bottom_y_slope) / 2.0;

    vec3 normal = normalize(vec3(x_slope, y_slope, 1.0));
    
    color = vec4(normal, 1.0);
}