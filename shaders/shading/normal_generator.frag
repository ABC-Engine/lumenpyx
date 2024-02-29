#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D heightmap;
// so that we can see if the albedo is invisible
uniform sampler2D albedomap;

// this function is the same as the one in the lighting shader
vec4 texture_pixel(sampler2D tex, vec2 coords) {
    vec2 new_coords = coords / textureSize(albedomap, 0);
    return texture(tex, new_coords);
}

void main() {
    if (texture(albedomap, v_tex_coords).a == 0.0) {
        color = vec4(0.0, 0.0, 0.0, 0.0);
        return;
    }

    vec4 center_color = texture(heightmap, v_tex_coords);
    if (center_color.a == 0.0) {
        color = vec4(0.0, 0.0, 0.0, 0.0);
        return;
    }

    vec2 new_coords = v_tex_coords * textureSize(albedomap, 0);

    float dzdx = (texture_pixel(heightmap, new_coords + vec2(1.0, 0.0)).r - texture_pixel(heightmap, new_coords - vec2(1.0, 0.0)).r) / 2.0;
    float dzdy = (texture_pixel(heightmap, new_coords + vec2(0.0, 1.0)).r - texture_pixel(heightmap, new_coords - vec2(0.0, 1.0)).r) / 2.0;
    vec3 direction = vec3(dzdx, dzdy, 1.0);

    color = normalize(vec4(direction, 1.0));
}