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
        color = vec4(0.0, 0.0, 0.0, 0.0);
        return;
    }


    /*
    dzdx=(z(x+1,y)-z(x-1,y))/2.0;
    dzdy=(z(x,y+1)-z(x,y-1))/2.0;
    direction=(-dzdx,-dzdy,1.0)
    magnitude=sqrt(direction.x**2 + direction.y**2 + direction.z**2)
    normal=direction/magnitude
    */

    vec2 new_coords = v_tex_coords * RESOLUTION;

    float dzdx = (texture_pixel(heightmap, new_coords + vec2(1.0, 0.0)).r - texture_pixel(heightmap, new_coords - vec2(1.0, 0.0)).r) / 2.0;
    float dzdy = (texture_pixel(heightmap, new_coords + vec2(0.0, 1.0)).r - texture_pixel(heightmap, new_coords - vec2(0.0, 1.0)).r) / 2.0;
    vec3 direction = vec3(dzdx, dzdy, 1.0);

    color = normalize(vec4(direction, 1.0));
}