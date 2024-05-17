#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform float width;
uniform float height;
uniform vec2 resolution;

void main() {
    float x = v_tex_coords.x;
    float y = v_tex_coords.y;
    
    // center the rectangle
    if (x < 0.5 - width / 2 || x > 0.5 + width / 2 || y < 0.5 - height / 2 || y > 0.5 + height / 2) {
        discard;
    } else {
        x = x - 0.5;
        
        float height = sqrt((width / 2) * (width / 2) - x * x);
        
        vec2 dir = -vec2(dFdx(height), dFdy(height)) * resolution;
        vec3 normal = normalize(vec3(dir, 1.0));
        color = vec4(normal, 1.0);
    }
}