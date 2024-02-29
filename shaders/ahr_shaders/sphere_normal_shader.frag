#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform float radius;

void main() {
    float x_coord = (v_tex_coords.x - 0.5) * 0.99;
    float y_coord = (v_tex_coords.y - 0.5) * 0.99;
    float dist = length(vec2(x_coord, y_coord));
    
    if (dist < radius) {
        float dydz = -(y_coord / sqrt(radius*radius - y_coord*y_coord));
        float dxdz = -(x_coord / sqrt(radius*radius - x_coord*x_coord));
        vec3 normal = normalize(vec3(dxdz, dydz, 1.0));
        color = vec4(normal, 1.0); // Assign vec3 normal to vec4 color
    } else {
        discard;
    }
}