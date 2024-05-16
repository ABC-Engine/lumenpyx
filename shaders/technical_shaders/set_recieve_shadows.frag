#version 140 core

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D last_draw;
uniform sampler2D this_draw;
uniform float shadow_strength;


void main() {
    vec4 subtracted_colors = texture(last_draw, v_tex_coords) - texture(this_draw, v_tex_coords);
    float total_difference = abs(subtracted_colors.r) + abs(subtracted_colors.g) + abs(subtracted_colors.b) + abs(subtracted_colors.a);
    if (total_difference < 0.01) {
        discard;
    } else {
        color = vec4(shadow_strength, shadow_strength, shadow_strength, shadow_strength);
    }
}