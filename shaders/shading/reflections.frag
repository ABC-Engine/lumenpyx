#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D albedomap;
uniform sampler2D heightmap;
uniform sampler2D roughnessmap;
uniform sampler2D normalmap;
uniform float camera_z;
uniform bool blur_reflections;

const vec4 NON_INTERSECT_COLOR = vec4(0.0, 0.0, 0.0, 0.0);
const float MAX_ROUGHNESS = 1.0;

bool is_pixel_in_bounds(vec2 coords) {
	// convert the pixel coordinates to texture coordinates
	vec2 tex_coords = coords / textureSize(albedomap, 0);
	// check if the texture coordinates are in the range [0, 1]
	if (tex_coords.x < 0.0 || tex_coords.x > 1.0 || tex_coords.y < 0.0 || tex_coords.y > 1.0) {
		return false;
	}
	return true;
}

// this function is the same as the one in the lighting shader
vec4 texture_pixel(sampler2D tex, vec2 coords) {
    vec2 new_coords = coords / textureSize(albedomap, 0);
    return texture(tex, new_coords);
}

// this function is the same as the one in the lighting shader
/// Linearly interpolates between two points, P1 and P2 are the endpoints, and P3 is the point to interpolate to
float lerp(vec3 P1, vec3 P2, vec2 P3) {
	float t = clamp((dot(P3 - P1.xy, P2.xy - P1.xy) / dot(P2.xy - P1.xy, P2.xy - P1.xy)), 0.0, 1.0);
	return mix(P1.z, P2.z, t);
}

// this function is the same as the one in the lighting shader
// most this code attributed to https://gist.github.com/nowke/965fed0d5191bf373f1262be584207bb
vec4 find_intersect_color(vec3 p1, vec3 p2) {
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
		// check if the point is out of bounds
		if (!is_pixel_in_bounds(vec2(x, y))) {
			return NON_INTERSECT_COLOR;
		}

        // linear interpolation between the two points to get the height of the line at the current x and y
		float height_of_line = lerp(p1, p2, vec2(x, y));
		// if the height of the line is greater than the height of the heightmap at the current x and y, return the color of the heightmap at the current x and y
		// also make sure the current x and y are not the same as the x and y of the two points, because in this use case they intersect by definition
        if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line && vec2(x,y) != vec2(x1, y1) && vec2(x,y) != vec2(x2, y2)) {
            vec4 new_color = texture_pixel(albedomap, vec2(x, y));
			float intersection_distance = distance(p1, vec3(x,y, height_of_line));
			new_color.a = intersection_distance;
			return new_color;
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

			// check if the point is out of bounds
			if (!is_pixel_in_bounds(vec2(x, y))) {
				return NON_INTERSECT_COLOR;
			}

			float height_of_line = lerp(p1, p2, vec2(x, y));
            if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line  && vec2(x,y) != vec2(x1, y1) && vec2(x,y) != vec2(x2, y2)) {
                vec4 new_color = texture_pixel(albedomap, vec2(x, y));
				float intersection_distance = distance(p1, vec3(x,y, height_of_line));
				new_color.a = intersection_distance;
				return new_color;
            }
		}

	} else {
		// check if the point is out of bounds
		if (!is_pixel_in_bounds(vec2(x, y))) {
			return NON_INTERSECT_COLOR;
		}

		float height_of_line = lerp(p1, p2, vec2(x, y));
        if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line && vec2(x,y) != vec2(x1, y1) && vec2(x,y) != vec2(x2, y2)) {
            vec4 new_color = texture_pixel(albedomap, vec2(x, y));
			float intersection_distance = distance(p1, vec3(x,y, height_of_line));
			new_color.a = intersection_distance;
			return new_color;
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

			// check if the point is out of bounds
			if (!is_pixel_in_bounds(vec2(x, y))) {
				return NON_INTERSECT_COLOR;
			}

			float height_of_line = lerp(p1, p2, vec2(x, y));
            if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line && vec2(x,y) != vec2(x1, y1) && vec2(x,y) != vec2(x2, y2)) {
                vec4 new_color = texture_pixel(albedomap, vec2(x, y));
				float intersection_distance = distance(p1, vec3(x,y, height_of_line));
				new_color.a = intersection_distance;
				return new_color;
            }
		}
	}
    return NON_INTERSECT_COLOR;
}

void main() {
	{
		float roughness = texture(roughnessmap, v_tex_coords).r;
		vec4 albedo = texture(albedomap, v_tex_coords);
		if (roughness <= 0.0 || albedo.a <= 0.0) {
			color = vec4(0.0, 0.0, 0.0, 0.0);
			return;
		}
	}

	vec3 new_v_tex_coords = vec3(v_tex_coords, texture(heightmap, v_tex_coords).r);

	// the camera will always be at the center of local space
	vec3 camera_pos = vec3(0.5, 0.5, camera_z);
	
	vec3 incident = normalize(new_v_tex_coords - camera_pos);
	vec3 normal = normalize(texture(normalmap, v_tex_coords).xyz);
	vec3 reflected = reflect(incident, normal);

	vec3 scaling = vec3(textureSize(heightmap, 0), 1.0);

	vec4 new_color = find_intersect_color(new_v_tex_coords * scaling, (new_v_tex_coords + reflected) * scaling);
	
	vec4 albedo = texture(albedomap, v_tex_coords);
	if (new_color == NON_INTERSECT_COLOR) {
		color = vec4(0.0, 0.0, 0.0, 0.0);
	}
	else {
		vec4 roughness = texture(roughnessmap, v_tex_coords);
		color = new_color * roughness.r + albedo * (1.0 - roughness.r);
		if (blur_reflections) {
			color.a = color.a / 500.0;
		}
		else {
			color.a = 1.0;
		}
	}
}