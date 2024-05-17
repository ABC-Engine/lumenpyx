#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform float radius_squared;
uniform vec2 resolution;

void main() {
    float x = v_tex_coords.x - 0.5;
    float y = v_tex_coords.y - 0.5;
    float distance_squared = x * x + y * y;

    if (distance_squared < radius_squared) {
        // the z coordinate as seen in the heightmap
        float z = sqrt(-x*x - y*y + radius_squared);

        // this will always be the same for any normal shader, given z is a function of height
        vec2 dir = -vec2(dFdx(z), dFdy(z)) * resolution;
        vec3 normal = normalize(vec3(dir, 1.0));
        
        color = vec4(normal, 1.0); // Assign vec3 normal to vec4 color
    } else {
        discard;
    }
}