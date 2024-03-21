use std::any::TypeId;

use crate::load_image;
use crate::shaders::draw_generate_normals;
use crate::shaders::FULL_SCREEN_QUAD;
use crate::LumenpyxProgram;
use crate::Transform;
use crate::Vertex;
use crate::DEFAULT_BEHAVIOR;
use glium;
use glium::glutin::surface::WindowSurface;
use glium::uniform;
use glium::Surface;

pub(crate) const BASE_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/primitives/sprite_ahr_shader.vert");
pub(crate) const BASE_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/primitives/sprite_ahr_shader.frag");

pub trait Drawable {
    /// Draw the object to the screen
    fn draw(
        &self,
        program: &LumenpyxProgram,
        transform_matrix: [[f32; 4]; 4],
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    );

    /// Load the shaders for the object
    /// This is called every frame, so make sure to check
    /// if the shader is already loaded or your performance will suffer
    fn try_load_shaders(&self, program: &mut LumenpyxProgram);

    fn get_position(&self) -> [[f32; 4]; 4];

    fn get_recieve_shadows_strength(&self) -> f32 {
        0.5
    }
}

pub enum Texture<'a> {
    Path(&'a str),
    Solid([f32; 4]),
}

impl<'a> From<&'a str> for Texture<'a> {
    fn from(path: &'a str) -> Self {
        Texture::Path(path)
    }
}

impl From<[f32; 4]> for Texture<'_> {
    fn from(color: [f32; 4]) -> Self {
        Texture::Solid(color)
    }
}

pub struct Sprite {
    albedo_texture: glium::texture::Texture2d,
    height_texture: glium::texture::Texture2d,
    roughness_texture: glium::texture::Texture2d,
    normal_texture: glium::texture::Texture2d,
    transform: Transform,
}

impl Sprite {
    pub fn new(
        albedo: Texture,
        height_path: Texture,
        roughness_path: Texture,
        program: &LumenpyxProgram,
        transform: Transform,
    ) -> Sprite {
        let display = &program.display;
        let indices = &program.indices;

        let albedo_texture = match albedo {
            Texture::Path(path) => {
                let image = load_image(path);
                glium::texture::Texture2d::new(display, image).unwrap()
            }
            Texture::Solid(color) => {
                let image = glium::texture::RawImage2d::from_raw_rgba(color.to_vec(), (1, 1));
                glium::texture::Texture2d::new(display, image).unwrap()
            }
        };
        let height_texture = match height_path {
            Texture::Path(path) => {
                let image = load_image(path);
                glium::texture::Texture2d::new(display, image).unwrap()
            }
            Texture::Solid(color) => {
                let image = glium::texture::RawImage2d::from_raw_rgba(color.to_vec(), (1, 1));
                glium::texture::Texture2d::new(display, image).unwrap()
            }
        };
        let roughness_texture = match roughness_path {
            Texture::Path(path) => {
                let image = load_image(path);
                glium::texture::Texture2d::new(display, image).unwrap()
            }
            Texture::Solid(color) => {
                let image = glium::texture::RawImage2d::from_raw_rgba(color.to_vec(), (1, 1));
                glium::texture::Texture2d::new(display, image).unwrap()
            }
        };

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

        {
            let height_uniform = glium::uniforms::Sampler(&height_texture, DEFAULT_BEHAVIOR);
            let albedo_uniform = glium::uniforms::Sampler(&albedo_texture, DEFAULT_BEHAVIOR);
            let mut normal_framebuffer =
                glium::framebuffer::SimpleFrameBuffer::new(display, &normal_texture).unwrap();

            draw_generate_normals(
                program,
                height_uniform,
                albedo_uniform,
                &mut normal_framebuffer,
            )
        }

        Sprite {
            albedo_texture,
            height_texture,
            normal_texture,
            roughness_texture,
            transform,
        }
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
}
