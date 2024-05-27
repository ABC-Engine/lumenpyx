#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;
uniform vec4 text_color;

void main() {
    vec4 new_color = text_color * vec4(1.0, 1.0, 1.0, texture(tex, v_tex_coords).r);
    if (new_color.a == 0.0) {
        discard;
    }

    // this might be too big of an assumption but i assume that anti-aliasing isn't wanted and the best way to try to get rid of it is to make the alpha 1.0
    color = vec4(new_color.rgb, round(new_color.a));
}