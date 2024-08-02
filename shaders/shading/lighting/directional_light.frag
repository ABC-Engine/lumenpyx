#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D heightmap;
uniform sampler2D albedomap;
uniform sampler2D shadow_strength_map;
uniform vec3 light_pos;
uniform vec3 light_color;
uniform float light_intensity;
uniform float light_distance_falloff;
uniform float light_angular_falloff;
uniform vec3 light_direction;

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

// Function to calculate the angular distance between two vectors
float angularDistance(vec3 P1, vec3 Origin, vec3 P2) {
    vec3 v1 = normalize(P1 - Origin);
    vec3 v2 = normalize(P2 - Origin);
    
    float dotProduct = dot(v1, v2);
    dotProduct = clamp(dotProduct, -1.0, 1.0); // Ensure dot product is within valid range
    
    float angle = acos(dotProduct);
    
    return angle;
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
	light_dist = light_dist * light_distance_falloff;
	float dist_falloff = (1 / (1.0 + light_dist * light_dist));

	float light_angle = angularDistance(vec3(v_tex_coords, new_v_tex_coords.z), light_pos, light_direction);
	light_angle = light_angle * light_angular_falloff;
	float angle_falloff = (1 / (1.0 + light_angle * light_angle));

    vec4 shaded_color = albedo_color * vec4(light_color, 1.0) * min(dist_falloff * angle_falloff * light_intensity, light_intensity);

    if (dimFactor <= 0.01 || !find_intersections(new_light_pos, new_v_tex_coords)) {
		color = shaded_color;
    }
	else {
		color = (shaded_color * (1.0 - dimFactor));
    }
}

