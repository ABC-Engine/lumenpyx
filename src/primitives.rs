use crate::load_image;
use crate::shaders;
use crate::Drawable;
use crate::LumenpyxProgram;
use crate::DEFAULT_BEHAVIOR;
use glium;
use glium::framebuffer::SimpleFrameBuffer;
use glium::uniform;
use glium::DrawParameters;
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
    blend_mode: Option<glium::Blend>,
) {
    let display = &program.display;
    let indices = &program.indices;

    let smallest_dim = framebuffer
        .get_dimensions()
        .0
        .min(framebuffer.get_dimensions().1);

    let radius = radius / smallest_dim as f32;

    let shader = program
        .get_shader("circle_ahr_shader")
        .expect("'circle_ahr_shader' shader not found, did you override the default shaders?");

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer = glium::VertexBuffer::new(display, &shape)
        .expect("Failed to create vertex buffer for circle");

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
            &DrawParameters {
                blend: blend_mode.unwrap_or(Default::default()),
                ..Default::default()
            },
        )
        .expect("Failed to draw circle");
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
    blend_mode: Option<glium::Blend>,
) {
    let display = &program.display;
    let indices = &program.indices;

    draw_circle(
        color,
        radius,
        matrix_transform,
        program,
        albedo_framebuffer,
        blend_mode,
    );

    let smallest_dim = albedo_framebuffer
        .get_dimensions()
        .0
        .min(albedo_framebuffer.get_dimensions().1);

    let radius = radius / smallest_dim as f32;

    {
        let height_shader = program
            .get_shader("sphere_height_shader")
            .expect("Failed to get sphere height shader");

        let shape = FULL_SCREEN_QUAD;

        let vertex_buffer = glium::VertexBuffer::new(display, &shape)
            .expect("Failed to create vertex buffer for sphere");

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
                &DrawParameters {
                    blend: blend_mode.unwrap_or(Default::default()),
                    ..Default::default()
                },
            )
            .expect("Failed to draw sphere height map");
    }

    {
        let normal_shader = program
            .get_shader("sphere_normal_shader")
            .expect("Failed to get sphere normal shader");

        let shape = FULL_SCREEN_QUAD;

        let vertex_buffer = glium::VertexBuffer::new(display, &shape)
            .expect("Failed to create vertex buffer for sphere");
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
                &DrawParameters {
                    blend: blend_mode.unwrap_or(Default::default()),
                    ..Default::default()
                },
            )
            .expect("Failed to draw sphere normal map");
    }
}

fn draw_rectangle(
    color: [f32; 4],
    width: f32,
    height: f32,
    matrix_transform: [[f32; 4]; 4],
    program: &LumenpyxProgram,
    framebuffer: &mut SimpleFrameBuffer,
    blend_mode: Option<glium::Blend>,
) {
    let display = &program.display;
    let indices = &program.indices;

    let smallest_dim = framebuffer
        .get_dimensions()
        .0
        .min(framebuffer.get_dimensions().1);
    let width = width / smallest_dim as f32;
    let height = height / smallest_dim as f32;

    let shader = program
        .get_shader("rectangle_ahr_shader")
        .expect("Failed to get rectangle shader");

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer = glium::VertexBuffer::new(display, &shape)
        .expect("Failed to create vertex buffer for rectangle");

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
            &DrawParameters {
                blend: blend_mode.unwrap_or(Default::default()),
                ..Default::default()
            },
        )
        .expect("Failed to draw rectangle");
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
        blend_mode: Option<glium::Blend>,
    ) {
        draw_circle(
            self.color,
            self.radius,
            matrix_transform,
            program,
            albedo_framebuffer,
            blend_mode,
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
        .expect("Failed to create circle shader");

        program.add_shader(shader, "circle_ahr_shader");
    }

    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.matrix
    }

    fn get_recieve_shadows_strength(&self) -> f32 {
        self.shadow_strength
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
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
        blend_mode: Option<glium::Blend>,
    ) {
        draw_sphere(
            self.color,
            self.radius,
            matrix_transform,
            program,
            albedo_framebuffer,
            height_framebuffer,
            normal_framebuffer,
            blend_mode,
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
            .expect("Failed to create sphere height shader");

            program.add_shader(shader, "sphere_height_shader");
        }

        if program.get_shader("circle_ahr_shader").is_none() {
            let shader = glium::Program::from_source(
                &program.display,
                GENERATE_CIRCLE_VERTEX_SHADER_SRC,
                GENERATE_CIRCLE_FRAGMENT_SHADER_SRC,
                None,
            )
            .expect("Failed to create circle shader");

            program.add_shader(shader, "circle_ahr_shader");
        }

        if program.get_shader("sphere_normal_shader").is_none() {
            let shader = glium::Program::from_source(
                &program.display,
                GENERATE_SPHERE_NORMAL_VERTEX_SHADER_SRC,
                GENERATE_SPHERE_NORMAL_FRAGMENT_SHADER_SRC,
                None,
            )
            .expect("Failed to create sphere normal shader");

            program.add_shader(shader, "sphere_normal_shader");
        }
    }

    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.matrix
    }

    fn get_recieve_shadows_strength(&self) -> f32 {
        self.shadow_strength
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
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
        blend_mode: Option<glium::Blend>,
    ) {
        draw_rectangle(
            self.color,
            self.width,
            self.height,
            matrix_transform,
            program,
            albedo_framebuffer,
            blend_mode,
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
        .expect("Failed to create rectangle shader");

        program.add_shader(shader, "rectangle_ahr_shader");
    }

    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.matrix
    }

    fn get_recieve_shadows_strength(&self) -> f32 {
        self.shadow_strength
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
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
        blend_mode: Option<glium::Blend>,
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
            blend_mode,
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
            .expect("Failed to create rectangle shader");

            program.add_shader(shader, "rectangle_ahr_shader");
        }

        if program.get_shader("cylinder_height_shader").is_none() {
            let shader = glium::Program::from_source(
                &program.display,
                GENERATE_CYLINDER_HEIGHT_VERTEX_SHADER_SRC,
                GENERATE_CYLINDER_HEIGHT_FRAGMENT_SHADER_SRC,
                None,
            )
            .expect("Failed to create cylinder height shader");

            program.add_shader(shader, "cylinder_height_shader");
        }

        if program.get_shader("cylinder_normal_shader").is_none() {
            let shader = glium::Program::from_source(
                &program.display,
                GENERATE_CYLINDER_NORMAL_VERTEX_SHADER_SRC,
                GENERATE_CYLINDER_NORMAL_FRAGMENT_SHADER_SRC,
                None,
            )
            .expect("Failed to create cylinder normal shader");

            program.add_shader(shader, "cylinder_normal_shader");
        }
    }

    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.matrix
    }

    fn get_recieve_shadows_strength(&self) -> f32 {
        self.shadow_strength
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
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
    blend_mode: Option<glium::Blend>,
) {
    draw_rectangle(
        color,
        radius * 2.0,
        height,
        matrix_transform,
        program,
        albedo_framebuffer,
        blend_mode,
    );

    let smallest_dim = albedo_framebuffer
        .get_dimensions()
        .0
        .min(albedo_framebuffer.get_dimensions().1);

    let radius = radius / smallest_dim as f32;

    let display = &program.display;
    let indices = &program.indices;

    let shader = program
        .get_shader("cylinder_height_shader")
        .expect("Failed to get cylinder height shader");

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer = glium::VertexBuffer::new(display, &shape)
        .expect("Failed to create vertex buffer for cylinder");

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
            &DrawParameters {
                blend: blend_mode.unwrap_or(Default::default()),
                ..Default::default()
            },
        )
        .expect("Failed to draw cylinder height map");

    let normal_shader = program
        .get_shader("cylinder_normal_shader")
        .expect("Failed to get cylinder normal shader");

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer = glium::VertexBuffer::new(display, &shape)
        .expect("Failed to create vertex buffer for cylinder");

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
            &DrawParameters {
                blend: blend_mode.unwrap_or(Default::default()),
                ..Default::default()
            },
        )
        .expect("Failed to draw cylinder normal map");
}

pub enum Texture {
    /// Relative Path to a texture
    Path(String),
    /// Solid color texture
    Solid([f32; 4]),
    Texture(glium::texture::Texture2d),
}

impl Texture {
    pub(crate) fn try_clone(&self) -> Texture {
        match self {
            Texture::Path(path) => Texture::Path(path.clone()),
            Texture::Solid(color) => Texture::Solid(*color),
            Texture::Texture(texture) => {
                panic!("Cannot clone a texture, make sure this isn't a texture before cloning")
            }
        }
    }
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

impl From<glium::texture::Texture2d> for Texture {
    fn from(texture: glium::texture::Texture2d) -> Self {
        Texture::Texture(texture)
    }
}

pub enum Normal {
    /// Path to a normal map
    Path(String),
    /// Solid color normal map
    Solid([f32; 4]),
    /// Generates a relatively accurate normal map from the height map
    AutoGenerated,
    Texture(glium::texture::Texture2d),
}

impl Normal {
    pub(crate) fn try_clone(&self) -> Normal {
        match self {
            Normal::Path(path) => Normal::Path(path.clone()),
            Normal::Solid(color) => Normal::Solid(*color),
            Normal::AutoGenerated => Normal::AutoGenerated,
            Normal::Texture(texture) => {
                panic!("Cannot clone a texture, make sure this isn't a texture before cloning")
            }
        }
    }
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

impl From<glium::texture::Texture2d> for Normal {
    fn from(texture: glium::texture::Texture2d) -> Self {
        Normal::Texture(texture)
    }
}

pub(crate) fn new_albedo_texture(
    program: &LumenpyxProgram,
    albedo: Texture,
) -> glium::texture::Texture2d {
    let display = &program.display;
    let albedo_texture = match albedo {
        Texture::Path(path) => {
            let image = load_image(path.as_str());
            glium::texture::Texture2d::new(display, image).expect("Failed to load texture")
        }
        Texture::Texture(texture) => texture,
        _ => panic!("Albedo texture must be a path or a texture"),
    };
    albedo_texture
}

pub(crate) fn new_non_albedo_texture(
    program: &LumenpyxProgram,
    texture: Texture,
    albedo_texture: &glium::texture::Texture2d,
) -> glium::texture::Texture2d {
    let display = &program.display;
    match texture {
        Texture::Path(path) => {
            let image = load_image(path.as_str());
            glium::texture::Texture2d::new(display, image).expect("Failed to load texture")
        }
        Texture::Solid(color) => {
            let albedo_sampler = glium::uniforms::Sampler(albedo_texture, crate::DEFAULT_BEHAVIOR);

            shaders::new_fill_alpha_texure(program, albedo_sampler, color)
        }
        Texture::Texture(texture) => texture,
    }
}

pub(crate) fn new_normal_texture(
    program: &LumenpyxProgram,
    normal: Normal,
    height_texture: &glium::texture::Texture2d,
    albedo_texture: &glium::texture::Texture2d,
) -> glium::texture::Texture2d {
    let display = &program.display;
    match normal {
        Normal::Path(path) => {
            let image = load_image(&path);
            glium::texture::Texture2d::new(display, image).expect("Failed to load texture")
        }
        Normal::Solid(color) => {
            let albedo_sampler = glium::uniforms::Sampler(albedo_texture, crate::DEFAULT_BEHAVIOR);

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
            .expect("Failed to create normal texture");

            let height_uniform = glium::uniforms::Sampler(height_texture, crate::DEFAULT_BEHAVIOR);
            let albedo_uniform = glium::uniforms::Sampler(albedo_texture, crate::DEFAULT_BEHAVIOR);
            let mut normal_framebuffer =
                glium::framebuffer::SimpleFrameBuffer::new(display, &normal_texture)
                    .expect("Failed to create normal framebuffer");

            crate::draw_generate_normals(
                program,
                height_uniform,
                albedo_uniform,
                &mut normal_framebuffer,
            );

            normal_texture
        }
        Normal::Texture(texture) => texture,
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

        let albedo_texture = new_albedo_texture(program, albedo);
        let height_texture = new_non_albedo_texture(program, height, &albedo_texture);
        let roughness_texture = new_non_albedo_texture(program, roughness, &albedo_texture);
        let normal_texture = new_normal_texture(program, normal, &height_texture, &albedo_texture);

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
        blend_mode: Option<glium::Blend>,
    ) {
        let indices = &program.indices;
        let display = &program.display;

        let shader = program
            .get_shader("sprite_shader")
            .expect("Failed to get sprite shader");

        let shape = FULL_SCREEN_QUAD;

        let vertex_buffer = glium::VertexBuffer::new(display, &shape)
            .expect("Failed to create vertex buffer for sprite");

        let mut image = glium::uniforms::Sampler(&self.albedo_texture, DEFAULT_BEHAVIOR);

        // scale the transform matrix to match the size of the texture
        // check which side is longer and scale the other side to match
        let width = self.albedo_texture.get_width() as f32;
        let height = self
            .albedo_texture
            .get_height()
            .expect("failed to get height of sprite's texture") as f32;
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

        draw_texture(
            &self.albedo_texture,
            transform_matrix,
            program,
            albedo_framebuffer,
            blend_mode,
        );

        draw_texture(
            &self.height_texture,
            transform_matrix,
            program,
            height_framebuffer,
            blend_mode,
        );

        draw_texture(
            &self.roughness_texture,
            transform_matrix,
            program,
            roughness_framebuffer,
            blend_mode,
        );

        draw_texture(
            &self.normal_texture,
            transform_matrix,
            program,
            normal_framebuffer,
            blend_mode,
        );
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
        .expect("Failed to create sprite shader");

        program.add_shader(new_shader, "sprite_shader");
    }

    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.matrix
    }

    /// 0.0 is no shadows, 1.0 is full shadows
    fn get_recieve_shadows_strength(&self) -> f32 {
        self.shadow_strength
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

pub(crate) fn draw_texture(
    texture: &glium::texture::Texture2d,
    matrix_transform: [[f32; 4]; 4],
    program: &LumenpyxProgram,
    framebuffer: &mut SimpleFrameBuffer,
    blend_mode: Option<glium::Blend>,
) {
    let display = &program.display;
    let indices = &program.indices;

    let shader = program
        .get_shader("sprite_shader")
        .expect("Failed to get sprite shader");

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer = glium::VertexBuffer::new(display, &shape)
        .expect("Failed to create vertex buffer for sprite");

    let image = glium::uniforms::Sampler(texture, DEFAULT_BEHAVIOR);

    let uniform = &uniform! {
        matrix: matrix_transform,
        image: image,
    };

    framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &shader,
            uniform,
            &DrawParameters {
                blend: blend_mode.unwrap_or(Default::default()),
                ..Default::default()
            },
        )
        .expect("failed to draw sprite");
}
