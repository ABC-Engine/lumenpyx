This uses the same principles as the custom drawable object but takes them to the next level.

```rust
use lumenpyx::lights::DEFAULT_LIGHT_BLENDING;
use lumenpyx::LumenpyxProgram;
use lumenpyx::shaders::FULL_SCREEN_QUAD;
use glium::framebuffer::SimpleFrameBuffer;
use lumenpyx::lights::LightDrawable;
use glium::uniform;
use glium::Surface;

pub(crate) const POINT_LIGHT_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/shading/lighting/point_light.vert");
pub(crate) const POINT_LIGHT_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/shading/lighting/point_light.frag");

pub struct PointLight {
    position: [f32; 3],
    color: [f32; 3],
    intensity: f32,
    falloff: f32,
}

impl LightDrawable for PointLight {
    fn draw(
        &self,
        program: &LumenpyxProgram,
        matrix_transform: [[f32; 4]; 4],
        albedo_framebuffer: &mut SimpleFrameBuffer,
        height_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
        albedo_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
        reflection_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
        shadow_strength_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    ) {
        let display = &program.display;
        let indices = &program.indices;

        // get the shader you loaded in in the load_shaders function
        let shader = &program.get_shader("point_light_shader").unwrap();

        let shape = FULL_SCREEN_QUAD;

        // the magic numbers are to transform the light position from -1.0 to 1.0 to 0.0 to 1.0
        let light_pos = [
            ((matrix_transform[3][0]) + 1.0) * 0.5,
            ((matrix_transform[3][1]) + 1.0) * 0.5,
            self.position[2] * matrix_transform[2][2],
        ];

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

        // provide all the uniforms mentioned in your shader
        let uniforms = &uniform! {
            heightmap: height_uniform,
            albedomap: albedo_uniform,
            shadow_strength_map: shadow_strength_uniform,
            light_pos: light_pos,
            light_color: self.color,
            light_intensity: self.intensity,
            light_falloff: self.falloff,
        };

        // be careful with the blending function here
        // it should be the DEFAULT_LIGHT_BLENDING constant from the lights module
        albedo_framebuffer
            .draw(
                &vertex_buffer,
                indices,
                &shader,
                uniforms,
                &glium::DrawParameters {
                    blend: DEFAULT_LIGHT_BLENDING,
                    ..Default::default()
                },
            )
            .unwrap();
    }

    // load the shader just like in drawable object
    fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
        if program.get_shader("point_light_shader").is_none() {
            let shader = glium::Program::from_source(
                &program.display,
                POINT_LIGHT_VERTEX_SHADER_SRC,
                POINT_LIGHT_FRAGMENT_SHADER_SRC,
                None,
            )
            .unwrap();

            program.add_shader(shader, "point_light_shader");
        }
    }

    /// this is implemented for every custom light so it can be adjusted for the camera
    fn get_transform(&self) -> [[f32; 4]; 4] {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position[0], self.position[1], self.position[2], 0.0],
        ]
    }
}
