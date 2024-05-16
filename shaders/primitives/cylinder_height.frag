#version 140 core

in vec2 v_tex_coords;
out vec4 color;

uniform float width;
uniform float height;

void main() {
    float x = v_tex_coords.x;
    float y = v_tex_coords.y;
    
    // center the rectangle
    if (x < 0.5 - width / 2 || x > 0.5 + width / 2 || y < 0.5 - height / 2 || y > 0.5 + height / 2) {
        discard;
    } else {
        x = x - 0.5;
        
        float height = sqrt((width / 2) * (width / 2) - x * x);
        color = vec4(height, height, height, 1.0);
    }
}