#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D heightmap;
uniform sampler2D albedomap;
uniform sampler2D shadow_strength_map;
uniform vec3 light_pos;
uniform vec3 light_color;
uniform float light_intensity;
uniform float light_falloff;

vec4 texture_pixel(sampler2D tex, vec2 coords) {
    vec2 new_coords = coords / textureSize(tex, 0);
	// if the coords are the v_tex_coords or the light_pos, return 0.0
	if (coords == v_tex_coords || coords == light_pos.xy) {
		return vec4(0.0, 0.0, 0.0, 0.0);
	}
    return texture(tex, new_coords);
}

/// Linearly interpolates between two points, P1 and P2 are the endpoints, and P3 is the point to interpolate to
float lerp(vec3 P1, vec3 P2, vec2 P3) {
	if (P1.xy == P3) {
		return 1000.0;
	}
	else if (P2.xy == P3) {
		return 1000.0;
	}

	if (P1 == P2) {
		return P2.z;
	}
	else if (abs(P1.x - P2.x) > abs(P1.y - P2.y)) {
		float t = (P3.x - P2.x) / (P1.x - P2.x);

		if (t < 0.0 || t > 1.0) {
			return 1000.0;
		}
		return P2.z + t * (P1.z - P2.z);
	}
	else {
		float t = (P3.y - P2.y) / (P1.y - P2.y);

		if (t < 0.0 || t > 1.0) {
			return 1000.0;
		}

		return P2.z + t * (P1.z - P2.z);
	}
}

// most this code attributed to https://gist.github.com/nowke/965fed0d5191bf373f1262be584207bb
bool find_intersections(vec3 p1, vec3 p2) {
    int x1 = int(round(p1.x));
    int y1 = int(round(p1.y));
    int x2 = int(round(p2.x));
    int y2 = int(round(p2.y));

	int dx, dy, i, e;
	int incx, incy, inc1, inc2;
	int x,y;

	dx = x2-x1;
	dy = y2-y1;

	dx = abs(dx);
	dy = abs(dy);
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
	float dimFactor = texture(shadow_strength_map, v_tex_coords).r;
	if (albedo_color.a == 0.0) {
		discard;
	}

    vec3 new_light_pos = vec3(textureSize(albedomap, 0) * (light_pos.xy), light_pos.z);
    vec3 new_v_tex_coords = vec3(textureSize(albedomap, 0) * v_tex_coords, texture(heightmap, v_tex_coords).r);
    
	float light_dist = distance(new_v_tex_coords, new_light_pos);
	light_dist = max(light_dist * light_falloff, 1.0);
    vec4 shaded_color = albedo_color * vec4(light_color, 1.0) * (light_intensity / (light_dist * light_dist));


    if (!find_intersections(new_light_pos, new_v_tex_coords)) {
		color = shaded_color;
    }
	else {
		color = (shaded_color * dimFactor);
    }
}

