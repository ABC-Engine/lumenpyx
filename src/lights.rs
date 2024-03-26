use crate::shaders::FULL_SCREEN_QUAD;
use crate::LumenpyxProgram;
use glium;
use glium::framebuffer::SimpleFrameBuffer;
use glium::uniform;
use glium::Blend;
use glium::Surface;

pub(crate) const POINT_LIGHT_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/shading/lighting/point_light.vert");
pub(crate) const POINT_LIGHT_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/shading/lighting/point_light.frag");

pub(crate) const RECTANGLE_LIGHT_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/shading/lighting/rectangle_light.vert");
pub(crate) const RECTANGLE_LIGHT_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/shading/lighting/rectangle_light.frag");

pub(crate) const DIRECTIONAL_LIGHT_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/shading/lighting/directional_light.vert");
pub(crate) const DIRECTIONAL_LIGHT_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/shading/lighting/directional_light.frag");

pub const DEFAULT_LIGHT_BLENDING: Blend = glium::Blend {
    color: glium::BlendingFunction::Addition {
        source: glium::LinearBlendingFactor::One,
        destination: glium::LinearBlendingFactor::One,
    },
    alpha: glium::BlendingFunction::Addition {
        source: glium::LinearBlendingFactor::One,
        destination: glium::LinearBlendingFactor::One,
    },
    constant_value: (0.0, 0.0, 0.0, 0.0),
};

/// A trait for drawable lights
/// This trait is used to draw lights in the scene
/// If you want to create a custom light, you can implement this trait
/// Follow the example here: https://github.com/ABC-Engine/lumenpyx/wiki/Creating-custom-drawable-objects
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

/// A point light source
/// falloff is the distance falloff of the light
#[derive(Copy, Clone)]
pub struct PointLight {
    position: [f32; 3],
    color: [f32; 3],
    intensity: f32,
    falloff: f32,
}

impl PointLight {
    /// Create a new point light
    pub fn new(position: [f32; 3], color: [f32; 3], intensity: f32, falloff: f32) -> PointLight {
        PointLight {
            position,
            color,
            intensity,
            falloff,
        }
    }

    /// Set the position of the light
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = [x, y, z];
    }

    /// Get the position of the light
    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }

    /// Set the color of the light in 0.0 - 1.0 range
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }

    /// Set the intensity of the light
    /// If the intensity is above 1.0, it can result in overexposure
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    /// Set the falloff of the light
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
        draw_point_light(
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

/// An area light source
pub struct AreaLight {
    position: [f32; 3],
    color: [f32; 3],
    intensity: f32,
    falloff: f32,
    width: f32,
    height: f32,
}

impl AreaLight {
    /// Create a new area light
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

    /// Set the position of the light
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = [x, y, z];
    }

    /// Get the position of the light
    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }

    /// Set the color of the light in 0.0 - 1.0 range
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }

    /// Set the intensity of the light
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    /// Set the falloff of the light
    pub fn set_falloff(&mut self, falloff: f32) {
        self.falloff = falloff;
    }

    /// Set the width of the light
    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    /// Set the height of the light
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

/// A directional light source with directional and distance falloff
pub struct DirectionalLight {
    position: [f32; 3],
    direction: [f32; 3],
    color: [f32; 3],
    intensity: f32,
    angular_falloff: f32,
    distance_falloff: f32,
}

impl Default for DirectionalLight {
    fn default() -> Self {
        DirectionalLight {
            position: [0.0, 0.0, 0.0],
            direction: [0.0, 0.0, 1.0],
            color: [1.0, 1.0, 1.0],
            intensity: 1.0,
            angular_falloff: 0.001,
            distance_falloff: 0.0,
        }
    }
}

impl DirectionalLight {
    /// Create a new directional light
    pub fn new(
        position: [f32; 3],
        direction: [f32; 3],
        color: [f32; 3],
        intensity: f32,
        angular_falloff: f32,
        distance_falloff: f32,
    ) -> DirectionalLight {
        DirectionalLight {
            position,
            direction,
            color,
            intensity,
            angular_falloff,
            distance_falloff,
        }
    }

    /// Set the direction of the light
    /// the light points in the direction of the vector
    pub fn set_direction(&mut self, x: f32, y: f32, z: f32) {
        self.direction = [x, y, z];
    }

    /// Get the direction of the light
    /// the light points in the direction of the vector
    pub fn get_direction(&self) -> [f32; 3] {
        self.direction
    }

    /// Set the color of the light in 0.0 - 1.0 range
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }

    /// Set the intensity of the light
    /// If the intensity is above 1.0, it can result in overexposure
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    /// Set the angular falloff of the light
    /// 0.0 is no falloff, 1.0 is full falloff
    pub fn set_angular_falloff(&mut self, angular_falloff: f32) {
        self.angular_falloff = angular_falloff;
    }

    /// Set the distance falloff of the light
    /// 0.0 is no falloff, 1.0 is full falloff
    pub fn set_distance_falloff(&mut self, distance_falloff: f32) {
        self.distance_falloff = distance_falloff;
    }
}

impl LightDrawable for DirectionalLight {
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
        draw_directional_light(
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
        if program.get_shader("directional_light_shader").is_none() {
            let shader = glium::Program::from_source(
                &program.display,
                DIRECTIONAL_LIGHT_VERTEX_SHADER_SRC,
                DIRECTIONAL_LIGHT_FRAGMENT_SHADER_SRC,
                None,
            )
            .unwrap();

            program.add_shader(shader, "directional_light_shader");
        }
    }

    fn get_transform(&self) -> [[f32; 4]; 4] {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position[0], self.position[1], self.position[2], 1.0],
        ]
    }
}

/// draw the point light
pub(crate) fn draw_point_light(
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

    let shape = FULL_SCREEN_QUAD;

    // the magic numbers are to transform the light position from -1.0 to 1.0 to 0.0 to 1.0
    let light_pos = [
        ((matrix_transform[3][0]) + 1.0) * 0.5,
        ((matrix_transform[3][1]) + 1.0) * 0.5,
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
                blend: DEFAULT_LIGHT_BLENDING,
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

    let shape = FULL_SCREEN_QUAD;

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
        shadow_strength_map: shadow_strength_uniform,
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
                blend: DEFAULT_LIGHT_BLENDING,
                ..Default::default()
            },
        )
        .unwrap();
}

fn draw_directional_light(
    program: &LumenpyxProgram,
    framebuffer: &mut SimpleFrameBuffer,
    albedo_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    height_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    shadow_strength_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    light: &DirectionalLight,
    matrix_transform: [[f32; 4]; 4],
) {
    let display = &program.display;
    let indices = &program.indices;
    let shader = &program.get_shader("directional_light_shader").unwrap();

    let shape = FULL_SCREEN_QUAD;

    let light_pos = [
        ((light.position[0] * matrix_transform[0][0]) + 1.0) * 0.5,
        ((light.position[1] * matrix_transform[1][1]) + 1.0) * 0.5,
        light.position[2] * matrix_transform[2][2],
    ];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let uniforms = &uniform! {
        heightmap: height_uniform,
        albedomap: albedo_uniform,
        shadow_strength_map: shadow_strength_uniform,
        light_pos: light_pos,
        light_color: light.color,
        light_intensity: light.intensity,
        light_distance_falloff: light.distance_falloff,
        light_angular_falloff: light.angular_falloff,
        light_direction: light.direction,
    };

    framebuffer
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
