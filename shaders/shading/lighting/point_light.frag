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
	float t = clamp((dot(P3 - P1.xy, P2.xy - P1.xy) / dot(P2.xy - P1.xy, P2.xy - P1.xy)), 0.0, 1.0);
	return mix(P1.z, P2.z, t);
}

// most this code attributed to https://gist.github.com/nowke/965fed0d5191bf373f1262be584207bb
bool find_intersections(vec3 p1, vec3 p2) {
    int x1 = int(p1.x);
    int y1 = int(p1.y);
    int x2 = int(p2.x);
    int y2 = int(p2.y);

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
		// if it hits the last point, return false
		// because it hits the pixel
		if (vec2(x, y) == vec2(x2, y2)) {
			return false;
		}
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

			if (vec2(x, y) == vec2(x2, y2)) {
				return false;
			}
			float height_of_line = lerp(p1, p2, vec2(x, y));
            if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
                return true;
            }
		}

	} else {
		if (vec2(x, y) == vec2(x2, y2)) {
			return false;
		}
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

			if (vec2(x, y) == vec2(x2, y2)) {
				return false;
			}
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
	light_dist = light_dist * light_falloff;
	float falloff = (light_intensity / (1.0 + light_dist * light_dist));
    vec4 shaded_color = albedo_color * vec4(light_color, 1.0) * falloff;

    if (dimFactor <= 0.01 || !find_intersections(new_light_pos, new_v_tex_coords)) {
		color = shaded_color;
    }
	else {
		color = (shaded_color * (1.0 - dimFactor));
    }
}

