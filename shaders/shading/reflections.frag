#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D albedomap;
uniform sampler2D heightmap;
uniform sampler2D roughnessmap;
uniform sampler2D normalmap;
uniform vec3 camera_pos;

const vec4 NON_INTERSECT_COLOR = vec4(0.0, 0.0, 0.0, 0.0);
const float MAX_ROUGHNESS = 1.0;

// this function is the same as the one in the lighting shader
vec4 texture_pixel(sampler2D tex, vec2 coords) {
    vec2 new_coords = coords / textureSize(albedomap, 0);
    return texture(tex, new_coords);
}

// this function is the same as the one in the lighting shader
/// Linearly interpolates between two points, P1 and P2 are the endpoints, and P3 is the point to interpolate to
float lerp(vec3 P1, vec3 P2, vec2 P3) {
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
        // linear interpolation between the two points to get the height of the line at the current x and y
        // TODO: double check the interpolation
		float height_of_line = lerp(p1, p2, vec2(x, y));
        if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
            return texture_pixel(albedomap, vec2(x, y));
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
                return texture_pixel(albedomap, vec2(x, y));
            }
		}

	} else {
		float height_of_line = lerp(p1, p2, vec2(x, y));
        if (texture_pixel(heightmap, vec2(x, y)).r > height_of_line) {
            return texture_pixel(albedomap, vec2(x, y));
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
                return texture_pixel(albedomap, vec2(x, y));
            }
		}
	}
    return NON_INTERSECT_COLOR;
}

/// returns a point that is the reflection that ends at the screen bounds
vec3 get_reflected_point(vec3 p1, vec3 p2, vec3 normal) {
    vec3 dir = normalize(p1-p2);
    vec3 reflected = reflect(dir, normal);
    return p2 - (p1 - p2) + reflected;
}

void main() {
    vec4 albedo = texture(albedomap, v_tex_coords);
    vec3 normal = texture(normalmap, v_tex_coords).xyz;
    float roughness = texture(roughnessmap, v_tex_coords).r;
    roughness = min(roughness, MAX_ROUGHNESS);
    if (roughness <= 0.0) {
        color = albedo;
        return;
    }

    vec3 new_camera_pos = vec3(textureSize(albedomap, 0) * (camera_pos.xy), camera_pos.z);
    vec3 new_v_tex_coords = vec3(textureSize(albedomap, 0) * v_tex_coords, texture(heightmap, v_tex_coords).r);
    vec3 reflection_point = get_reflected_point(new_v_tex_coords, new_camera_pos , normal);

    vec4 intersection_color = find_intersect_color(new_v_tex_coords, reflection_point);

    if (intersection_color == NON_INTERSECT_COLOR) {
        color = albedo;
    }
    else {
        // lerp the intersection color * albedo with the albedo
        color = mix(albedo, intersection_color, roughness);
    }
}