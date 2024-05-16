#version 140 core

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
    if (texture_pixel(albedomap, v_tex_coords).a <= 0.0) {
        discard;
    }

    float dx = texture(heightmap, v_tex_coords + vec2(1.0/textureSize(heightmap, 0).x, 0.0)).r - texture(heightmap, v_tex_coords - vec2(1.0/textureSize(heightmap, 0).x, 0.0)).r;
    float dy = texture(heightmap, v_tex_coords + vec2(0.0, 1.0/textureSize(heightmap, 0).y)).r - texture(heightmap, v_tex_coords - vec2(0.0, 1.0/textureSize(heightmap, 0).y)).r;
    vec2 dir = -vec2(dx, dy)* textureSize(heightmap, 0) * 0.5;

    vec3 normal = normalize(vec3(dir, 1.0));
    color = vec4(normal, 1.0);
}