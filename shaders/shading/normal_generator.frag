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
    if (texture_pixel(albedomap, v_tex_coords).a <= 0.0) {
        discard;
    }

    float height = texture(heightmap, v_tex_coords).r;
    vec2 dir =  -vec2(dFdx(height), dFdy(height))* textureSize(heightmap, 0);

    vec3 normal = normalize(vec3(dir, 1.0));
    color = vec4(normal, 1.0);
}