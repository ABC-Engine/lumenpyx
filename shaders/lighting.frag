#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D roughnessmap;
uniform sampler2D heightmap;
uniform sampler2D albedomap;
uniform vec3 light_pos;
uniform vec3 light_color;
uniform float light_intensity;
uniform float light_falloff;

// Green for now for debugging
const vec4 UNLIT_COLOR = vec4(0.0, 1.0, 0.0, 1.0);

// the unlit color will be the albedo color dimmed by dimFactor
const float dimFactor = 0.5;
const vec2 RESOLUTION = vec2(128.0, 128.0);

vec4 texture_pixel(sampler2D tex, vec2 coords) {
    vec2 new_coords = RESOLUTION * coords;
    return texture(tex, new_coords);
}

/// Linearly interpolates between two points, P1 and P2 are the endpoints, and P3 is the point to interpolate to
float lerp(vec3 P1, vec3 P2, vec2 P3) {
	if (P1.x == P2.x && P1.y == P2.y) {
		return P2.z;
	}
	if (abs(P1.x - P2.x) > abs(P1.y - P2.y)) {
		float t = (P3.x - P2.x) / (P1.x - P2.x);

		return P2.z + t * (P1.z - P2.z);
	}
	else {
		float t = (P3.y - P2.y) / (P1.y - P2.y);

		return P2.z + t * (P1.z - P2.z);
	}
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
		float height_of_line = lerp(p1, p2, vec2(x, y));
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

			float height_of_line = lerp(p1, p2, vec2(x, y));
            if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
                return true;
            }
		}

	} else {
		float height_of_line = lerp(p1, p2, vec2(x, y));
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

			float height_of_line = lerp(p1, p2, vec2(x, y));
            if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
                return true;
            }
		}
	}
    return false;
}

void main() {
	vec4 albedo_color = texture(albedomap, v_tex_coords);
	if (albedo_color.a == 0.0) {
		discard;
	}

    vec3 new_light_pos = vec3(RESOLUTION * (light_pos.xy), light_pos.z);
    vec3 new_v_tex_coords = vec3(RESOLUTION * v_tex_coords, texture(heightmap, v_tex_coords).r);
    
	float light_dist = distance(new_v_tex_coords, new_light_pos);
	light_dist = max(light_dist * light_falloff, 1.0);
    vec4 shaded_color = albedo_color * vec4(light_color, 1.0) * (light_intensity / (light_dist * light_dist));

    if (!does_intersect(new_v_tex_coords, new_light_pos)) {
        color += shaded_color;
		if (color == UNLIT_COLOR) {
			color = shaded_color;
		}
    } else {
		//color += shaded_color * dimFactor;
		// TODO: fix this, it currently forms line y = 1.0x + light_pos.y - light_pos.x
		color = UNLIT_COLOR;
    }
}
