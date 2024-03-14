use glium;
use glium::glutin::surface::WindowSurface;
use glium::implement_vertex;
use glium::Surface;
pub use winit;
use winit::event_loop::EventLoop;
pub mod primitives;
mod shaders;
use shaders::*;
mod drawable_object;
pub use drawable_object::*;
use rustc_hash::FxHashMap;
pub mod lights;
#[doc = include_str!("../README.md")]

pub(crate) const DEFAULT_BEHAVIOR: glium::uniforms::SamplerBehavior =
    glium::uniforms::SamplerBehavior {
        minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
        magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
        max_anisotropy: 1,
        wrap_function: (
            glium::uniforms::SamplerWrapFunction::Mirror,
            glium::uniforms::SamplerWrapFunction::Mirror,
            glium::uniforms::SamplerWrapFunction::Mirror,
        ),
        depth_texture_comparison: None,
    };

pub struct LumenpyxProgram {
    pub window: winit::window::Window,
    pub display: glium::Display<WindowSurface>,
    pub indices: glium::index::NoIndices,
    pub reflection_shader: glium::Program,
    pub upscale_shader: glium::Program,
    other_shaders: FxHashMap<String, glium::Program>,
    dimensions: [u32; 2],
}

impl LumenpyxProgram {
    pub fn new(resolution: [u32; 2]) -> (LumenpyxProgram, EventLoop<()>) {
        let (event_loop, window, display, indices) = setup_program();
        let reflection_shader = glium::Program::from_source(
            &display,
            shaders::REFLECTION_VERTEX_SHADER_SRC,
            shaders::REFLECTION_FRAGMENT_SHADER_SRC,
            None,
        )
        .unwrap();

        let upscale_shader = glium::Program::from_source(
            &display,
            shaders::UPSCALE_VERTEX_SHADER_SRC,
            shaders::UPSCALE_FRAGMENT_SHADER_SRC,
            None,
        )
        .unwrap();

        let program = LumenpyxProgram {
            window,
            display,
            indices,
            reflection_shader,
            upscale_shader,
            other_shaders: FxHashMap::default(),
            dimensions: resolution,
        };

        (program, event_loop)
    }

    pub fn add_shader(&mut self, program: glium::Program, name: &str) {
        self.other_shaders.insert(name.to_string(), program);
    }

    pub fn get_shader(&self, name: &str) -> Option<&glium::Program> {
        self.other_shaders.get(name)
    }
}

#[derive(Copy, Clone)]
pub struct Transform {
    matrix: [[f32; 4]; 4],
}

impl Transform {
    // we multiply by 2.0 because the shader expects the position to be in the range of -1.0 to 1.0 were as the scale of the object is 0.0 to 1.0
    pub fn new(pos: [f32; 3]) -> Transform {
        Transform {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [pos[0] * 2.0, pos[1] * 2.0, pos[2] * 2.0, 1.0],
            ],
        }
    }

    pub fn get_matrix(&self) -> [[f32; 4]; 4] {
        self.matrix
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[3][0] = x * 2.0;
        self.matrix[3][1] = y * 2.0;
        self.matrix[3][2] = z * 2.0;
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[0][0] = x;
        self.matrix[1][1] = y;
        self.matrix[2][2] = z;
    }

    pub fn set_x(&mut self, x: f32) {
        self.matrix[3][0] = x * 2.0;
    }

    pub fn set_y(&mut self, y: f32) {
        self.matrix[3][1] = y * 2.0;
    }

    pub fn set_z(&mut self, z: f32) {
        self.matrix[3][2] = z * 2.0;
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

pub fn setup_program() -> (
    EventLoop<()>,
    winit::window::Window,
    glium::Display<WindowSurface>,
    glium::index::NoIndices,
) {
    // this is just a wrapper for the setup_window function for now
    let (event_loop, display, window, indices) = setup_window();

    (event_loop, window, display, indices)
}

fn load_image(path: &str) -> glium::texture::RawImage2d<u8> {
    let img = image::open(path).unwrap();
    img.flipv();
    let path = format!("{}", path);
    let image = image::load(
        std::io::Cursor::new(std::fs::read(path).unwrap()),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image, image_dimensions);
    image
}

fn setup_window() -> (
    EventLoop<()>,
    glium::Display<WindowSurface>,
    winit::window::Window,
    glium::index::NoIndices,
) {
    // 1. The **winit::EventLoop** for handling events.
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    // 2. Create a glutin context and glium Display
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    (event_loop, display, window, indices)
}

pub struct Camera {
    pub position: [f32; 3],
}

impl Camera {
    pub fn new(position: [f32; 3]) -> Camera {
        Camera { position }
    }
}

pub fn draw_all(
    /*display: &glium::Display<WindowSurface>,
    indices: &glium::index::NoIndices,*/
    lights: Vec<&dyn lights::LightDrawable>,
    drawables: Vec<&dyn Drawable>,
    program: &mut LumenpyxProgram,
    camera: Camera,
) {
    // this is kind of inefficient, but it works for now
    for drawable in &drawables {
        drawable.try_load_shaders(program);
    }
    for light in &lights {
        light.try_load_shaders(program);
    }

    /*
    STEP 1:
        render every albedo to a texture
        render every height to a texture
        render every roughness to a texture
    STEP 2:
        take the textures and feed it into a lighting shader
        we do this for every light and then blend the results together
    STEP 3:
        take the result and feed it into a reflection shader
        it uses screen space reflections and lerps between the reflection and the original image based on the roughness
    STEP 4:
        upscale the result to the screen size
    */
    let display = &program.display;

    let albedo_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        program.dimensions[0],
        program.dimensions[1],
    )
    .unwrap();

    let height_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        program.dimensions[0],
        program.dimensions[1],
    )
    .unwrap();

    let normal_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        program.dimensions[0],
        program.dimensions[1],
    )
    .unwrap();

    let roughness_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        program.dimensions[0],
        program.dimensions[1],
    )
    .unwrap();

    {
        let mut albedo_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &albedo_texture).unwrap();

        let mut height_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &height_texture).unwrap();

        let mut roughness_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &roughness_texture).unwrap();

        let mut normal_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &normal_texture).unwrap();

        for drawable in &drawables {
            let mut new_matrix = drawable.get_position();
            // scale off the resolution
            if program.dimensions[0] > program.dimensions[1] {
                new_matrix[0][0] *= program.dimensions[1] as f32 / program.dimensions[0] as f32;
            } else {
                new_matrix[1][1] *= program.dimensions[0] as f32 / program.dimensions[1] as f32;
            }
            // adjust off the camera
            new_matrix[3][0] -= camera.position[0];
            new_matrix[3][1] -= camera.position[1];
            new_matrix[3][2] -= camera.position[2];

            drawable.draw(
                program,
                new_matrix,
                &mut albedo_framebuffer,
                &mut height_framebuffer,
                &mut roughness_framebuffer,
                &mut normal_framebuffer,
            )
        }
    }

    let lit_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        program.dimensions[0],
        program.dimensions[1],
    )
    .expect("Failed to create lit frame buffer");

    {
        let albedo = glium::uniforms::Sampler(&albedo_texture, DEFAULT_BEHAVIOR);
        let height_sampler = glium::uniforms::Sampler(&height_texture, DEFAULT_BEHAVIOR);
        let roughness_sampler = glium::uniforms::Sampler(&roughness_texture, DEFAULT_BEHAVIOR);

        let mut lit_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &lit_texture).unwrap();

        for light in lights {
            let mut new_matrix = light.get_transform();
            if program.dimensions[0] > program.dimensions[1] {
                new_matrix[0][0] *= program.dimensions[1] as f32 / program.dimensions[0] as f32;
            } else {
                new_matrix[1][1] *= program.dimensions[0] as f32 / program.dimensions[1] as f32;
            }
            new_matrix[3][0] -= camera.position[0];
            new_matrix[3][1] -= camera.position[1];
            new_matrix[3][2] -= camera.position[2];

            light.draw(
                program,
                new_matrix,
                &mut lit_framebuffer,
                height_sampler,
                albedo,
                roughness_sampler,
            );
        }
    }

    let reflected_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        program.dimensions[0],
        program.dimensions[1],
    )
    .expect("Failed to create reflected frame buffer");

    {
        let roughness = glium::uniforms::Sampler(&roughness_texture, DEFAULT_BEHAVIOR);
        let height = glium::uniforms::Sampler(&height_texture, DEFAULT_BEHAVIOR);
        let normal = glium::uniforms::Sampler(&normal_texture, DEFAULT_BEHAVIOR);
        let lit_sampler = glium::uniforms::Sampler(&lit_texture, DEFAULT_BEHAVIOR);

        let mut reflected_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &reflected_texture).unwrap();

        draw_reflections(
            camera,
            lit_sampler,
            height,
            roughness,
            normal,
            &mut reflected_framebuffer,
            &program,
        );
    }

    {
        let finished_texture = glium::uniforms::Sampler(&reflected_texture, DEFAULT_BEHAVIOR);
        draw_upscale(finished_texture, &program);
    }
}
