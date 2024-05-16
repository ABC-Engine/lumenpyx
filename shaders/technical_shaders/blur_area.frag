#version 140 core

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;
uniform float blur_scale;   

const float Pi = 6.28318530718; // Pi*2
const float Directions = 16.0;
const float Quality = 3.0; 

// This code is adapted from https://www.shadertoy.com/view/Xltfzj
void main() {
    vec4 pixel_color = texture(tex, v_tex_coords);
    if(pixel_color.a == 0.0) {
        discard;
    }

    // alpha is the radius of the blur
    vec2 resolution = textureSize(tex, 0);
    // on one hand i think this should be squared but on the other hand it looks better this way
    vec2 Radius = pixel_color.aa * blur_scale;
    
    vec2 uv = v_tex_coords;

    vec4 new_color = texture(tex, uv);
    
    float color_magnitude = Quality * Directions - 15.0;

    // Blur calculations
    // d is rotation angle
    for( float d=0.0; d<Pi; d+=Pi/Directions)
    {
        // i is distance from center
		for(float i=1.0/Quality; i<=1.0; i+=1.0/Quality)
        {
            vec4 pix_color = texture( tex, uv + vec2(cos(d),sin(d)) * Radius * i);
			if (pix_color.a > 0.0)
            {
                new_color += pix_color;
            }
            else {
                color_magnitude -= 1;
            }
        }
    }
    
    // Output to screen
    new_color /= color_magnitude;
    color = vec4(new_color.rgb, 1.0);
}
