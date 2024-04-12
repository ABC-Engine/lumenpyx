use crate::load_image;
use crate::shaders;
use crate::Drawable;
use crate::LumenpyxProgram;
use crate::DEFAULT_BEHAVIOR;
use glium;
use glium::framebuffer::SimpleFrameBuffer;
use glium::uniform;
use glium::Surface;

const GENERATE_CIRCLE_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/primitives/circle_ahr_shader.vert");
const GENERATE_CIRCLE_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/primitives/circle_ahr_shader.frag");

const GENERATE_SPHERE_HEIGHT_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/primitives/sphere_height_shader.vert");
const GENERATE_SPHERE_HEIGHT_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/primitives/sphere_height_shader.frag");

const GENERATE_SPHERE_NORMAL_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/primitives/sphere_normal_shader.vert");
const GENERATE_SPHERE_NORMAL_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/primitives/sphere_normal_shader.frag");

const GENERATE_RECTANGLE_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/primitives/rectangle_ahr_shader.vert");
const GENERATE_RECTANGLE_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/primitives/rectangle_ahr_shader.frag");

const GENERATE_CYLINDER_HEIGHT_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/primitives/cylinder_height.vert");
const GENERATE_CYLINDER_HEIGHT_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/primitives/cylinder_height.frag");

const GENERATE_CYLINDER_NORMAL_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/primitives/cylinder_normal.vert");
const GENERATE_CYLINDER_NORMAL_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/primitives/cylinder_normal.frag");

pub(crate) const BASE_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/primitives/sprite_ahr_shader.vert");
pub(crate) const BASE_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/primitives/sprite_ahr_shader.frag");

use crate::shaders::FULL_SCREEN_QUAD;
use crate::Transform;

/// Draws a circle with the given color and radius.
/// The circle is drawn in the framebuffer, this is for custom Drawables.
pub fn draw_circle(
    color: [f32; 4],
    radius: f32,
    matrix_transform: [[f32; 4]; 4],
    program: &LumenpyxProgram,
    framebuffer: &mut SimpleFrameBuffer,
) {
    let display = &program.display;
    let indices = &program.indices;

    let smallest_dim = framebuffer
        .get_dimensions()
        .0
        .min(framebuffer.get_dimensions().1);

    let radius = radius / smallest_dim as f32;

    let shader = program.get_shader("circle_ahr_shader").unwrap();

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let uniforms = &uniform! {
        circle_color: color,
        radius_squared: radius.powi(2),
        matrix: matrix_transform,
    };

    framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &shader,
            uniforms,
            &Default::default(),
        )
        .unwrap();
}

/// Draws a sphere with the given color and radius.
/// The sphere is drawn in the framebuffer, this is for custom Drawables.
pub fn draw_sphere(
    color: [f32; 4],
    radius: f32,
    matrix_transform: [[f32; 4]; 4],
    program: &LumenpyxProgram,
    albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
) {
    let display = &program.display;
    let indices = &program.indices;

    draw_circle(color, radius, matrix_transform, program, albedo_framebuffer);

    let smallest_dim = albedo_framebuffer
        .get_dimensions()
        .0
        .min(albedo_framebuffer.get_dimensions().1);

    let radius = radius / smallest_dim as f32;

    {
        let height_shader = program.get_shader("sphere_height_shader").unwrap();

        let shape = FULL_SCREEN_QUAD;

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

        let uniforms = &uniform! {
            matrix: matrix_transform,
            radius_squared: radius.powi(2),
        };

        height_framebuffer
            .draw(
                &vertex_buffer,
                indices,
                &height_shader,
                uniforms,
                &Default::default(),
            )
            .unwrap();
    }

    {
        let normal_shader = program.get_shader("sphere_normal_shader").unwrap();

        let shape = FULL_SCREEN_QUAD;

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let resolution = [
            albedo_framebuffer.get_dimensions().0 as f32,
            albedo_framebuffer.get_dimensions().1 as f32,
        ];
        let radius_squared = radius.powi(2);

        let uniforms = &uniform! {
            matrix: matrix_transform,
            radius_squared: radius_squared,
            resolution: resolution,
        };

        normal_framebuffer
            .draw(
                &vertex_buffer,
                indices,
                &normal_shader,
                uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

fn draw_rectangle(
    color: [f32; 4],
    width: f32,
    height: f32,
    matrix_transform: [[f32; 4]; 4],
    program: &LumenpyxProgram,
    framebuffer: &mut SimpleFrameBuffer,
) {
    let display = &program.display;
    let indices = &program.indices;

    let smallest_dim = framebuffer
        .get_dimensions()
        .0
        .min(framebuffer.get_dimensions().1);
    let width = width / smallest_dim as f32;
    let height = height / smallest_dim as f32;

    let shader = program.get_shader("rectangle_ahr_shader").unwrap();

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let uniforms = &uniform! {
        rect_color: color,
        width: width,
        height: height,
        matrix: matrix_transform,
    };

    framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &shader,
            uniforms,
            &Default::default(),
        )
        .unwrap();
}

/// A circle primitive.
pub struct Circle {
    color: [f32; 4],
    radius: f32,
    pub transform: Transform,
    shadow_strength: f32,
}

impl Circle {
    /// Creates a new circle with the given color, radius, and transform.
    pub fn new(color: [f32; 4], radius: f32, transform: Transform) -> Circle {
        Circle {
            color,
            radius,
            transform,
            shadow_strength: 0.5,
        }
    }
}

impl Drawable for Circle {
    fn draw(
        &self,
        program: &LumenpyxProgram,
        matrix_transform: [[f32; 4]; 4],
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        draw_circle(
            self.color,
            self.radius,
            matrix_transform,
            program,
            albedo_framebuffer,
        );
    }

    fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
        if program.get_shader("circle_ahr_shader").is_some() {
            return;
        }

        let shader = glium::Program::from_source(
            &program.display,
            GENERATE_CIRCLE_VERTEX_SHADER_SRC,
            GENERATE_CIRCLE_FRAGMENT_SHADER_SRC,
            None,
        )
        .unwrap();

        program.add_shader(shader, "circle_ahr_shader");
    }

    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.matrix
    }

    fn get_recieve_shadows_strength(&self) -> f32 {
        self.shadow_strength
    }
}

/// A sphere primitive.
pub struct Sphere {
    color: [f32; 4],
    radius: f32,
    pub transform: Transform,
    shadow_strength: f32,
}

impl Sphere {
    /// Creates a new sphere with the given color, radius, and transform.
    pub fn new(color: [f32; 4], radius: f32, transform: Transform) -> Sphere {
        Sphere {
            color,
            radius,
            transform,
            shadow_strength: 0.5,
        }
    }
}

impl Drawable for Sphere {
    fn draw(
        &self,
        program: &LumenpyxProgram,
        matrix_transform: [[f32; 4]; 4],
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        draw_sphere(
            self.color,
            self.radius,
            matrix_transform,
            program,
            albedo_framebuffer,
            height_framebuffer,
            normal_framebuffer,
        );
    }

    fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
        // this assumes both shaders will always be loaded together
        if program.get_shader("sphere_height_shader").is_none() {
            let shader = glium::Program::from_source(
                &program.display,
                GENERATE_SPHERE_HEIGHT_VERTEX_SHADER_SRC,
                GENERATE_SPHERE_HEIGHT_FRAGMENT_SHADER_SRC,
                None,
            )
            .unwrap();

            program.add_shader(shader, "sphere_height_shader");
        }

        if program.get_shader("circle_ahr_shader").is_none() {
            let shader = glium::Program::from_source(
                &program.display,
                GENERATE_CIRCLE_VERTEX_SHADER_SRC,
                GENERATE_CIRCLE_FRAGMENT_SHADER_SRC,
                None,
            )
            .unwrap();

            program.add_shader(shader, "circle_ahr_shader");
        }

        if program.get_shader("sphere_normal_shader").is_none() {
            let shader = glium::Program::from_source(
                &program.display,
                GENERATE_SPHERE_NORMAL_VERTEX_SHADER_SRC,
                GENERATE_SPHERE_NORMAL_FRAGMENT_SHADER_SRC,
                None,
            )
            .unwrap();

            program.add_shader(shader, "sphere_normal_shader");
        }
    }

    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.matrix
    }

    fn get_recieve_shadows_strength(&self) -> f32 {
        self.shadow_strength
    }
}

/// A rectangle primitive.
pub struct Rectangle {
    color: [f32; 4],
    width: f32,
    height: f32,
    pub transform: Transform,
    shadow_strength: f32,
}

impl Rectangle {
    /// Creates a new rectangle with the given color, width, height, and transform.
    pub fn new(color: [f32; 4], width: f32, height: f32, transform: Transform) -> Rectangle {
        Rectangle {
            color,
            width,
            height,
            transform,
            shadow_strength: 0.5,
        }
    }
}

impl Drawable for Rectangle {
    fn draw(
        &self,
        program: &LumenpyxProgram,
        matrix_transform: [[f32; 4]; 4],
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        draw_rectangle(
            self.color,
            self.width,
            self.height,
            matrix_transform,
            program,
            albedo_framebuffer,
        );
    }

    fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
        if program.get_shader("rectangle_ahr_shader").is_some() {
            return;
        }

        let shader = glium::Program::from_source(
            &program.display,
            GENERATE_RECTANGLE_VERTEX_SHADER_SRC,
            GENERATE_RECTANGLE_FRAGMENT_SHADER_SRC,
            None,
        )
        .unwrap();

        program.add_shader(shader, "rectangle_ahr_shader");
    }

    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.matrix
    }

    fn get_recieve_shadows_strength(&self) -> f32 {
        self.shadow_strength
    }
}

/// A cylinder primitive.
pub struct Cylinder {
    color: [f32; 4],
    radius: f32,
    height: f32,
    pub transform: Transform,
    shadow_strength: f32,
}

impl Cylinder {
    /// Creates a new cylinder with the given color, radius, height, and transform.
    pub fn new(color: [f32; 4], radius: f32, height: f32, transform: Transform) -> Cylinder {
        Cylinder {
            color,
            radius,
            height,
            transform,
            shadow_strength: 0.5,
        }
    }

    /// Sets the shadow strength of the cylinder.
    pub fn set_shadow_strength(&mut self, shadow_strength: f32) {
        self.shadow_strength = shadow_strength;
    }
}

impl Drawable for Cylinder {
    fn draw(
        &self,
        program: &LumenpyxProgram,
        matrix_transform: [[f32; 4]; 4],
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        draw_cylinder(
            self.color,
            self.radius,
            self.height,
            matrix_transform,
            program,
            albedo_framebuffer,
            height_framebuffer,
            normal_framebuffer,
        );
    }

    fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
        if program.get_shader("rectangle_ahr_shader").is_none() {
            let shader = glium::Program::from_source(
                &program.display,
                GENERATE_RECTANGLE_VERTEX_SHADER_SRC,
                GENERATE_RECTANGLE_FRAGMENT_SHADER_SRC,
                None,
            )
            .unwrap();

            program.add_shader(shader, "rectangle_ahr_shader");
        }

        if program.get_shader("cylinder_height_shader").is_none() {
            let shader = glium::Program::from_source(
                &program.display,
                GENERATE_CYLINDER_HEIGHT_VERTEX_SHADER_SRC,
                GENERATE_CYLINDER_HEIGHT_FRAGMENT_SHADER_SRC,
                None,
            )
            .unwrap();

            program.add_shader(shader, "cylinder_height_shader");
        }

        if program.get_shader("cylinder_normal_shader").is_none() {
            let shader = glium::Program::from_source(
                &program.display,
                GENERATE_CYLINDER_NORMAL_VERTEX_SHADER_SRC,
                GENERATE_CYLINDER_NORMAL_FRAGMENT_SHADER_SRC,
                None,
            )
            .unwrap();

            program.add_shader(shader, "cylinder_normal_shader");
        }
    }

    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.matrix
    }

    fn get_recieve_shadows_strength(&self) -> f32 {
        self.shadow_strength
    }
}

fn draw_cylinder(
    color: [f32; 4],
    radius: f32,
    height: f32,
    matrix_transform: [[f32; 4]; 4],
    program: &LumenpyxProgram,
    albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
) {
    draw_rectangle(
        color,
        radius * 2.0,
        height,
        matrix_transform,
        program,
        albedo_framebuffer,
    );

    let smallest_dim = albedo_framebuffer
        .get_dimensions()
        .0
        .min(albedo_framebuffer.get_dimensions().1);

    let radius = radius / smallest_dim as f32;

    let display = &program.display;
    let indices = &program.indices;

    let shader = program.get_shader("cylinder_height_shader").unwrap();

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let uniforms = &uniform! {
        width: radius * 2.0,
        height: height,
        matrix: matrix_transform,
    };

    height_framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &shader,
            uniforms,
            &Default::default(),
        )
        .unwrap();

    let normal_shader = program.get_shader("cylinder_normal_shader").unwrap();

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let resolution = [
        normal_framebuffer.get_dimensions().0 as f32,
        normal_framebuffer.get_dimensions().1 as f32,
    ];

    let uniforms = &uniform! {
        width: radius * 2.0,
        height: height,
        resolution: resolution,
        matrix: matrix_transform,
    };

    normal_framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &normal_shader,
            uniforms,
            &Default::default(),
        )
        .unwrap();
}

pub enum Texture {
    /// Relative Path to a texture
    Path(String),
    /// Solid color texture
    Solid([f32; 4]),
}

impl From<String> for Texture {
    fn from(path: String) -> Self {
        Texture::Path(path)
    }
}

impl<'a> From<&'a str> for Texture {
    fn from(path: &str) -> Self {
        Texture::Path(path.to_string())
    }
}

impl From<[f32; 4]> for Texture {
    fn from(color: [f32; 4]) -> Self {
        Texture::Solid(color)
    }
}

impl From<[u8; 4]> for Texture {
    fn from(color: [u8; 4]) -> Self {
        Texture::Solid([
            color[0] as f32 / 255.0,
            color[1] as f32 / 255.0,
            color[2] as f32 / 255.0,
            color[3] as f32 / 255.0,
        ])
    }
}

impl From<f32> for Texture {
    fn from(value: f32) -> Self {
        Texture::Solid([value, value, value, 1.0])
    }
}

impl From<u8> for Texture {
    fn from(value: u8) -> Self {
        Texture::Solid([
            value as f32 / 255.0,
            value as f32 / 255.0,
            value as f32 / 255.0,
            1.0,
        ])
    }
}

pub enum Normal {
    /// Path to a normal map
    Path(String),
    /// Solid color normal map
    Solid([f32; 4]),
    /// Generates a relatively accurate normal map from the height map
    AutoGenerated,
}

impl From<&str> for Normal {
    fn from(path: &str) -> Self {
        Normal::Path(path.to_string())
    }
}

impl From<String> for Normal {
    fn from(path: String) -> Self {
        Normal::Path(path)
    }
}

impl From<[f32; 4]> for Normal {
    fn from(color: [f32; 4]) -> Self {
        Normal::Solid(color)
    }
}

impl Default for Normal {
    fn default() -> Self {
        Normal::AutoGenerated
    }
}

pub struct Sprite {
    albedo_texture: glium::texture::Texture2d,
    height_texture: glium::texture::Texture2d,
    roughness_texture: glium::texture::Texture2d,
    normal_texture: glium::texture::Texture2d,
    pub transform: Transform,
    shadow_strength: f32,
}

impl Sprite {
    pub fn new(
        albedo: Texture,
        height: Texture,
        roughness: Texture,
        normal: Normal,
        program: &LumenpyxProgram,
        transform: Transform,
    ) -> Sprite {
        let display = &program.display;

        let albedo_texture = match albedo {
            Texture::Path(path) => {
                let image = load_image(path.as_str());
                glium::texture::Texture2d::new(display, image).unwrap()
            }
            Texture::Solid(color) => {
                let image = glium::texture::RawImage2d::from_raw_rgba(color.to_vec(), (1, 1));
                glium::texture::Texture2d::new(display, image).unwrap()
            }
        };
        let height_texture = match height {
            Texture::Path(path) => {
                let image = load_image(path.as_str());
                glium::texture::Texture2d::new(display, image).unwrap()
            }
            Texture::Solid(color) => {
                let albedo_sampler =
                    glium::uniforms::Sampler(&albedo_texture, crate::DEFAULT_BEHAVIOR);

                shaders::new_fill_alpha_texure(program, albedo_sampler, color)
            }
        };
        let roughness_texture = match roughness {
            Texture::Path(path) => {
                let image = load_image(path.as_str());
                glium::texture::Texture2d::new(display, image).unwrap()
            }
            Texture::Solid(color) => {
                let albedo_sampler =
                    glium::uniforms::Sampler(&albedo_texture, crate::DEFAULT_BEHAVIOR);

                shaders::new_fill_alpha_texure(program, albedo_sampler, color)
            }
        };

        let normal_texture = match normal {
            Normal::Path(path) => {
                let image = load_image(&path);
                glium::texture::Texture2d::new(display, image).unwrap()
            }
            Normal::Solid(color) => {
                let albedo_sampler =
                    glium::uniforms::Sampler(&albedo_texture, crate::DEFAULT_BEHAVIOR);

                shaders::new_fill_alpha_texure(program, albedo_sampler, color)
            }
            Normal::AutoGenerated => {
                let normal_texture = glium::texture::Texture2d::empty_with_format(
                    display,
                    glium::texture::UncompressedFloatFormat::U8U8U8U8,
                    glium::texture::MipmapsOption::NoMipmap,
                    albedo_texture.get_width(),
                    albedo_texture
                        .get_height()
                        .expect("Failed to get height of albedo texture"),
                )
                .unwrap();

                let height_uniform =
                    glium::uniforms::Sampler(&height_texture, crate::DEFAULT_BEHAVIOR);
                let albedo_uniform =
                    glium::uniforms::Sampler(&albedo_texture, crate::DEFAULT_BEHAVIOR);
                let mut normal_framebuffer =
                    glium::framebuffer::SimpleFrameBuffer::new(display, &normal_texture).unwrap();

                crate::draw_generate_normals(
                    program,
                    height_uniform,
                    albedo_uniform,
                    &mut normal_framebuffer,
                );

                normal_texture
            }
        };

        Sprite {
            albedo_texture,
            height_texture,
            roughness_texture,
            normal_texture,
            transform,
            shadow_strength: 0.5,
        }
    }

    pub fn set_shadow_strength(&mut self, strength: f32) {
        self.shadow_strength = strength;
    }
}

impl Drawable for Sprite {
    fn draw(
        &self,
        program: &LumenpyxProgram,
        transform_matrix: [[f32; 4]; 4],
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let indices = &program.indices;
        let display = &program.display;

        let shader = program.get_shader("sprite_shader").unwrap();

        let shape = FULL_SCREEN_QUAD;

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

        let mut image = glium::uniforms::Sampler(&self.albedo_texture, DEFAULT_BEHAVIOR);

        // scale the transform matrix to match the size of the texture
        // check which side is longer and scale the other side to match
        let width = self.albedo_texture.get_width() as f32;
        let height = self.albedo_texture.get_height().unwrap() as f32;
        let mut transform_matrix = transform_matrix;

        // adjust size of the sprite to match the texture
        {
            let smallest_dimension = (albedo_framebuffer.get_dimensions().1 as f32)
                .min(albedo_framebuffer.get_dimensions().0 as f32);
            let x_scale = width as f32 / smallest_dimension;
            let y_scale = height as f32 / smallest_dimension;

            transform_matrix[0][0] *= x_scale;
            transform_matrix[1][1] *= y_scale;
        }

        let uniform = &uniform! {
            matrix: transform_matrix,
            image: image,
        };

        albedo_framebuffer
            .draw(
                &vertex_buffer,
                indices,
                &shader,
                uniform,
                &Default::default(),
            )
            .unwrap();

        image = glium::uniforms::Sampler(&self.height_texture, DEFAULT_BEHAVIOR);
        let uniform = &uniform! {
            matrix: transform_matrix,
            image: image,
        };
        height_framebuffer
            .draw(
                &vertex_buffer,
                indices,
                &shader,
                uniform,
                &Default::default(),
            )
            .unwrap();

        image = glium::uniforms::Sampler(&self.roughness_texture, DEFAULT_BEHAVIOR);
        let uniform = &uniform! {
            matrix: transform_matrix,
            image: image,
        };

        roughness_framebuffer
            .draw(
                &vertex_buffer,
                indices,
                &shader,
                uniform,
                &Default::default(),
            )
            .unwrap();

        image = glium::uniforms::Sampler(&self.normal_texture, DEFAULT_BEHAVIOR);
        let uniform = &uniform! {
            matrix: transform_matrix,
            image: image,
        };
        normal_framebuffer
            .draw(
                &vertex_buffer,
                indices,
                &shader,
                uniform,
                &Default::default(),
            )
            .unwrap();
    }

    fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
        if program.get_shader("sprite_shader").is_some() {
            return;
        }

        let new_shader = glium::Program::from_source(
            &program.display,
            BASE_VERTEX_SHADER_SRC,
            BASE_FRAGMENT_SHADER_SRC,
            None,
        )
        .unwrap();

        program.add_shader(new_shader, "sprite_shader");
    }

    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.matrix
    }

    /// 0.0 is no shadows, 1.0 is full shadows
    fn get_recieve_shadows_strength(&self) -> f32 {
        self.shadow_strength
    }
}
