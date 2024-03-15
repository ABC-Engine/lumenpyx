use crate::LumenpyxProgram;
use crate::Vertex;
use glium;
use glium::framebuffer::SimpleFrameBuffer;
use glium::glutin::surface::WindowSurface;
use glium::uniform;
use glium::Surface;

pub(crate) const POINT_LIGHT_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/shading/lighting/point_light.vert");
pub(crate) const POINT_LIGHT_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/shading/lighting/point_light.frag");

pub(crate) const RECTANGLE_LIGHT_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/shading/lighting/rectangle_light.vert");
pub(crate) const RECTANGLE_LIGHT_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/shading/lighting/rectangle_light.frag");

pub trait LightDrawable {
    fn draw(
        &self,
        program: &LumenpyxProgram,
        matrix_transform: [[f32; 4]; 4],
        albedo_framebuffer: &mut SimpleFrameBuffer,
        height_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
        albedo_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
        roughness_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
        shadow_strength_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    );
    fn try_load_shaders(&self, program: &mut LumenpyxProgram);
    fn get_transform(&self) -> [[f32; 4]; 4];
}

#[derive(Copy, Clone)]
pub struct PointLight {
    position: [f32; 3],
    color: [f32; 3],
    intensity: f32,
    falloff: f32,
}

impl PointLight {
    pub fn new(position: [f32; 3], color: [f32; 3], intensity: f32, falloff: f32) -> PointLight {
        PointLight {
            position,
            color,
            intensity,
            falloff,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = [x, y, z];
    }

    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }

    /// Set the color of the light in 0.0 - 1.0 range
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    pub fn set_falloff(&mut self, falloff: f32) {
        self.falloff = falloff;
    }
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
        draw_lighting(
            albedo_uniform,
            height_uniform,
            shadow_strength_uniform,
            albedo_framebuffer,
            program,
            &self,
            matrix_transform,
        )
    }

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

    fn get_transform(&self) -> [[f32; 4]; 4] {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position[0], self.position[1], self.position[2], 0.0],
        ]
    }
}

pub struct AreaLight {
    position: [f32; 3],
    color: [f32; 3],
    intensity: f32,
    falloff: f32,
    width: f32,
    height: f32,
}

impl AreaLight {
    pub fn new(
        position: [f32; 3],
        color: [f32; 3],
        intensity: f32,
        falloff: f32,
        width: f32,
        height: f32,
    ) -> AreaLight {
        AreaLight {
            position,
            color,
            intensity,
            falloff,
            width,
            height,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = [x, y, z];
    }

    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }

    /// Set the color of the light in 0.0 - 1.0 range
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    pub fn set_falloff(&mut self, falloff: f32) {
        self.falloff = falloff;
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }
}

impl LightDrawable for AreaLight {
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
        draw_area_light(
            program,
            albedo_framebuffer,
            albedo_uniform,
            height_uniform,
            shadow_strength_uniform,
            &self,
            matrix_transform,
        )
    }

    fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
        if program.get_shader("rectangle_light_shader").is_none() {
            let shader = glium::Program::from_source(
                &program.display,
                RECTANGLE_LIGHT_VERTEX_SHADER_SRC,
                RECTANGLE_LIGHT_FRAGMENT_SHADER_SRC,
                None,
            )
            .unwrap();

            program.add_shader(shader, "rectangle_light_shader");
        }
    }

    fn get_transform(&self) -> [[f32; 4]; 4] {
        [
            [self.width, 0.0, 0.0, 0.0],
            [0.0, self.height, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position[0], self.position[1], self.position[2], 1.0],
        ]
    }
}

/// draw the lighting
pub(crate) fn draw_lighting(
    albedo_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    heightmap: glium::uniforms::Sampler<glium::texture::Texture2d>,
    shadow_strength_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    framebuffer: &mut SimpleFrameBuffer,
    program: &LumenpyxProgram,
    light: &PointLight,
    matrix_transform: [[f32; 4]; 4],
) {
    let display = &program.display;
    let indices = &program.indices;
    let shader = &program.get_shader("point_light_shader").unwrap();

    let shape = vec![
        Vertex {
            position: [-1.0, -1.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [1.0, -1.0],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [-1.0, -1.0],
            tex_coords: [0.0, 0.0],
        },
    ];

    // the magic numbers are to transform the light position from -1.0 to 1.0 to 0.0 to 1.0
    let light_pos = [
        ((light.position[0] * matrix_transform[0][0]) + 1.0) * 0.5,
        ((light.position[1] * matrix_transform[1][1]) + 1.0) * 0.5,
        light.position[2] * matrix_transform[2][2],
    ];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let uniforms = &uniform! {
        heightmap: heightmap,
        albedomap: albedo_uniform,
        shadow_strength_map: shadow_strength_uniform,
        light_pos: light_pos,
        light_color: light.color,
        light_intensity: light.intensity,
        light_falloff: light.falloff,
    };

    framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &shader,
            uniforms,
            &glium::DrawParameters {
                blend: glium::Blend {
                    color: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::One,
                        destination: glium::LinearBlendingFactor::One,
                    },
                    alpha: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::One,
                        destination: glium::LinearBlendingFactor::One,
                    },
                    constant_value: (0.0, 0.0, 0.0, 0.0),
                },
                ..Default::default()
            },
        )
        .unwrap();
}

fn draw_area_light(
    program: &LumenpyxProgram,
    framebuffer: &mut SimpleFrameBuffer,
    albedo_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    height_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    shadow_strength_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    light: &AreaLight,
    matrix_transform: [[f32; 4]; 4],
) {
    let display = &program.display;
    let indices = &program.indices;
    let shader = &program.get_shader("rectangle_light_shader").unwrap();

    let shape = vec![
        Vertex {
            position: [-1.0, -1.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [1.0, -1.0],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [-1.0, -1.0],
            tex_coords: [0.0, 0.0],
        },
    ];

    let light_pos = [
        ((light.position[0] * matrix_transform[0][0]) + 1.0) * 0.5,
        ((light.position[1] * matrix_transform[1][1]) + 1.0) * 0.5,
        light.position[2] * matrix_transform[2][2],
    ];
    let light_width = light.width * matrix_transform[0][0];
    let light_height = light.height * matrix_transform[1][1];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let uniforms = &uniform! {
        heightmap: height_uniform,
        albedomap: albedo_uniform,
        light_pos: light_pos,
        light_color: light.color,
        light_intensity: light.intensity,
        light_falloff: light.falloff,
        width: light_width,
        height: light_height,
    };

    framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &shader,
            uniforms,
            &glium::DrawParameters {
                blend: glium::Blend {
                    color: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::One,
                        destination: glium::LinearBlendingFactor::One,
                    },
                    alpha: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::One,
                        destination: glium::LinearBlendingFactor::One,
                    },
                    constant_value: (0.0, 0.0, 0.0, 0.0),
                },
                ..Default::default()
            },
        )
        .unwrap();
}
