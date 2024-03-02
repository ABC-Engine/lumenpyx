#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D full_res_heightmap;
uniform sampler2D medium_res_heightmap;
uniform sampler2D low_res_heightmap;

uniform sampler2D albedomap;
uniform vec3 light_pos;
uniform vec3 light_color;
uniform float light_intensity;
uniform float light_falloff;

// the unlit color will be the albedo color dimmed by dimFactor
const float dimFactor = 0.0;

const vec3 NO_INTERSECTION = vec3(-1, -1, -1);

vec4 texture_pixel(sampler2D tex, vec2 coords) {
    vec2 new_coords = coords / textureSize(albedomap, 0);
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
vec3 find_intersections(vec3 p1, vec3 p2, sampler2D heightmap) {
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
        // linear interpolation between the two points to get the height of the line at the current x and y
        // TODO: double check the interpolation
		float height_of_line = lerp(p1, p2, vec2(x, y));
        if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
            return vec3(x, y, height_of_line);
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
                return vec3(x, y, height_of_line);
            }
		}

	} else {
		float height_of_line = lerp(p1, p2, vec2(x, y));
        if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
            return vec3(x, y, height_of_line);
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
                return vec3(x, y, height_of_line);
            }
		}
	}
    return NO_INTERSECTION;
}

// most this code attributed to https://gist.github.com/nowke/965fed0d5191bf373f1262be584207bb
// bounds is left, right, top, bottom
vec3 find_intersections_within_bounds(vec3 p1, vec3 p2, sampler2D heightmap, vec4 bounds) {
	int left = int(bounds.x);
	int right = int(bounds.y);
	int top = int(bounds.z);
	int bottom = int(bounds.w);

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
		if (x<left || x>right || y<top || y>bottom) {
			return NO_INTERSECTION;
		}

        // linear interpolation between the two points to get the height of the line at the current x and y
        // TODO: double check the interpolation
		float height_of_line = lerp(p1, p2, vec2(x, y));
        if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
            return vec3(x, y, height_of_line);
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

			if (x<left || x>right || y<top || y>bottom) {
				return NO_INTERSECTION;
			}

			float height_of_line = lerp(p1, p2, vec2(x, y));
            if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
                return vec3(x, y, height_of_line);
            }
		}

	} else {
		if (x<left || x>right || y<top || y>bottom) {
			return NO_INTERSECTION;
		}

		float height_of_line = lerp(p1, p2, vec2(x, y));
        if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
            return vec3(x, y, height_of_line);
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

			if (x<left || x>right || y<top || y>bottom) {
				return NO_INTERSECTION;
			}

			float height_of_line = lerp(p1, p2, vec2(x, y));
            if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
                return vec3(x, y, height_of_line);
            }
		}
	}
    return NO_INTERSECTION;
}

vec3 check_intersection_in_square(vec3 p1, vec3 p2, bool convert_coords, sampler2D lower_res_heightmap, sampler2D higher_res_heightmap)
{
	vec2 pixel_conversion;
	if (convert_coords)
	{
		pixel_conversion = textureSize(higher_res_heightmap, 0) / textureSize(lower_res_heightmap, 0);
	}
	else
	{
		pixel_conversion = vec2(1, 1);
	}
	vec2 pixel_pos_xy = p1.xy * pixel_conversion;
	vec3 pixel_pos = vec3(pixel_pos_xy, texture(higher_res_heightmap, pixel_pos_xy).r);
	vec4 bounds = vec4(pixel_pos_xy, pixel_pos_xy + pixel_conversion);
	vec3 intersection = find_intersections_within_bounds(pixel_pos, p2, higher_res_heightmap, bounds);
	if (intersection == NO_INTERSECTION) {
		return NO_INTERSECTION;
	}
	else {
		return intersection;
	}
}

// WIP
// this should probably be a recursive function
bool does_intersect_grid_method(vec3 p1, vec3 p2)
{
	float original_slope = (p1.y - p2.y) / (p1.x - p2.x);

	vec3 last_intersection_low_res = p1;

	while (true)
	{
		// we go from light to pixel here, so we need to start at the last intersection and go to the pixel
		vec3 intersection_low_res = find_intersections(last_intersection_low_res, p2, low_res_heightmap);
		if (intersection_low_res == NO_INTERSECTION) {
			// we don't find any intersections, in the entire heightmap
			return false;
		}
		else {
			last_intersection_low_res = intersection_low_res;
			// if the intersection is on the medium res heightmap
			// we need to check only the points on the pixel that intersect with the low res heightmap
			// we go from low_res pixel pos to the equivalent medium res pixel pos

			const vec3 FIRST_VALUE = vec3(-1.0, -1.0, -1.0);
			vec3 last_intersection_medium_res = FIRST_VALUE;

			for (int i = 0; i < 4; i++)
			{
				bool is_first_value = last_intersection_medium_res == FIRST_VALUE;
				// use the check_intersection_in_square function to check the medium res heightmap
				vec3 intersection_medium_res = check_intersection_in_square(last_intersection_low_res, p2, is_first_value, medium_res_heightmap, low_res_heightmap);

				if (last_intersection_medium_res == NO_INTERSECTION) {
					// we don't find any intersections, in the single low res pixel we still need to check the rest of the pixels
					break;
				}
				else {
					last_intersection_medium_res = intersection_medium_res;
					// if the intersection is on the full res heightmap
					// we need to check only the points on the pixel that intersect with the medium res heightmap
					// we go from medium_res pixel pos to the equivalent full res pixel pos

					vec3 intersection_high_res = check_intersection_in_square(last_intersection_medium_res, p2, true, full_res_heightmap, medium_res_heightmap);
					if (intersection_high_res == NO_INTERSECTION) {
						// we don't find any intersections, in the single medium res pixel we still need to check the rest of the pixels
						continue;
					}
					else {
						// we found an intersection on the full res heightmap
						return true;
					}
				}
			}
		}
	}
}

void main() {
	vec4 albedo_color = texture(albedomap, v_tex_coords);
	if (albedo_color.a == 0.0) {
		discard;
	}

    vec3 new_light_pos = vec3(textureSize(albedomap, 0) * (light_pos.xy), light_pos.z);
    vec3 new_v_tex_coords = vec3(textureSize(albedomap, 0) * v_tex_coords, texture(full_res_heightmap, v_tex_coords).r);
    
	float light_dist = distance(new_v_tex_coords, new_light_pos);
	light_dist = max(light_dist * light_falloff, 1.0);
    vec4 shaded_color = albedo_color * vec4(light_color, 1.0) * (light_intensity / (light_dist * light_dist));

	// we check if there are any intersections on the low res heightmap
	// if there are, we check the medium res heightmap for intersections
	// if there are, we check the full res heightmap 
	// if there are any intersections we dim the color

	if (does_intersect_grid_method(new_v_tex_coords, new_light_pos)) {
		color = shaded_color * dimFactor;
	}
	else {
		color = shaded_color;
	}


    /*if (new_v_tex_coords.z < new_light_pos.z && !does_intersect(new_light_pos, new_v_tex_coords)) {
		color = shaded_color;
    }
	else {
		color = (shaded_color * dimFactor);
    }*/
}

