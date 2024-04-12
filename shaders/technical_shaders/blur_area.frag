#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;
uniform float Size;

void main() {
    float Pi = 6.28318530718; // Pi*2
    
    // GAUSSIAN BLUR SETTINGS {{{
    float Directions = 16.0; // BLUR DIRECTIONS (Default 16.0 - More is better but slower)
    float Quality = 3.0; // BLUR QUALITY (Default 4.0 - More is better but slower)
    float Size = 4.0; // BLUR SIZE (Radius)
    // GAUSSIAN BLUR SETTINGS }}}
   
    vec2 Radius = Size/iResolution.xy;
    
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;
    // Pixel colour
    vec4 new_color = texture(tex, uv);
    
    int color_magnitude = Quality * Directions - 15.0;

    // Blur calculations
    for( float d=0.0; d<Pi; d+=Pi/Directions)
    {
		for(float i=1.0/Quality; i<=1.0; i+=1.0/Quality)
        {
            vec4 pix_color = texture( tex, uv+vec2(cos(d),sin(d))*Radius*i);
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
    color = new_color;
}
