#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D roughnessmap;
uniform sampler2D heightmap;
uniform sampler2D albedomap;
uniform vec3 light_pos;

vec4 texture_pixel(sampler2D tex, vec2 coords) {
    // adjust from 0-1 to 0-128
    // TODO: FIX MAGIC NUMBER
    vec2 new_coords = 128.0 * coords;
    return texture(tex, new_coords);
}

// most this code attributed to https://gist.github.com/nowke/965fed0d5191bf373f1262be584207bb
bool does_intersect(vec3 p1, vec3 p2) {
    int x1 = int(p1.x);
    int y1 = int(p1.y);
    int x2 = int(p2.x);
    int y2 = int(p2.y);

	int dx, dy, i, e;
	int incx, incy, inc1, inc2;
	int x,y;

	dx = x2-x1;
	dy = y2-y1;

	if (dx < 0) dx = -dx;
	if (dy < 0) dy = -dy;
	incx = 1;
	if (x2 < x1) incx = -1;
	incy = 1;
	if (y2 < y1) incy = -1;
	x = x1; y = y1;
	if (dx > dy) {
        // linear interpolation between the two points to get the height of the line at the current x and y
        // TODO: double check the interpolation
        float height_of_line = mix(p1.z, p2.z, (x - p1.x) / (p2.x - p1.x));
        if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
            return true;
        }

		e = 2 * dy-dx;
		inc1 = 2*(dy-dx);
		inc2 = 2*dy;
		for (i=0; i<dx; i++) {
			if (e >= 0) {
				y += incy;
				e += inc1;
			}
			else
			    e += inc2;
			x += incx;

			float height_of_line = mix(p1.z, p2.z, (x - p1.x) / (p2.x - p1.x));
            if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
                return true;
            }
		}

	} else {
		float height_of_line = mix(p1.z, p2.z, (x - p1.x) / (p2.x - p1.x));
        if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
            return true;
        }

		e = 2*dx-dy;
		inc1 = 2*(dx-dy);
		inc2 = 2*dx;
		for (i=0; i<dy; i++) {
			if (e >= 0) {
				x += incx;
				e += inc1;
			}
			else
				e += inc2;
			y += incy;

		    float height_of_line = mix(p1.z, p2.z, (x - p1.x) / (p2.x - p1.x));
            if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
                return true;
            }
		}
	}
    return false;
}

void main() {
    vec3 new_light_pos = 128.0 * light_pos;
    vec3 new_v_tex_coords = 128.0 * vec3(v_tex_coords, texture(heightmap, v_tex_coords).r);
    
    if (!does_intersect(new_v_tex_coords, light_pos)) {
        color = texture(albedomap, v_tex_coords);
    } else {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    }
}
