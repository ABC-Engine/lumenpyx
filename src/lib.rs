use std::path::Path;

use glium;
use glium::glutin::surface::WindowSurface;
use glium::implement_vertex;
use glium::Surface;
/// This module contains all the window and display setup functions
pub use winit;
use winit::event_loop::EventLoop;
/// This module contains all the objects that can be drawn in the program
pub mod primitives;
/// This module contains the full screen quad for custom shaders
pub mod shaders;
use shaders::*;
/// This module contains all the objects that can be drawn in the program
/// As well as containing the trait that all drawable objects must implement
pub mod drawable_object;
use drawable_object::*;
use rustc_hash::FxHashMap;
pub mod animation;
/// This module contains all the lights that can be used in the program
/// As well as containing the trait that all lights must implement
pub mod lights;

// include the whole lumenpyx.wiki folder into the documentation
#[doc = include_str!("../lumenpyx wiki/Home.md")]
#[doc = include_str!("../lumenpyx wiki/Common-problems-and-their-solutions.md")]
#[doc = include_str!("../lumenpyx wiki/Creating-custom-drawable-objects.md")]
#[doc = include_str!("../lumenpyx wiki/Creating-Custom-Lights.md")]
#[doc = include_str!("../lumenpyx wiki/Creating-Custom-Renderables.md")]
#[doc = include_str!("../lumenpyx wiki/Rendering-a-Sprite.md")]
#[doc = include_str!("../lumenpyx wiki/Technical-Documentation.md")]

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

/// The debug option is used to determine what to display on the screen
pub enum DebugOption {
    /// Display the final image
    None,
    /// Display the albedo texture
    Albedo,
    /// Display the height texture
    Height,
    /// Display the roughness texture
    Roughness,
    /// Display the normal texture
    Normal,
    /// Display the internal shadow strength texture
    ShadowStrength,
}

impl Default for DebugOption {
    fn default() -> Self {
        DebugOption::None
    }
}

/// The main struct that contains the window and display
pub struct LumenpyxProgram {
    /// The window that the program is running in (this is a winit window)
    pub window: winit::window::Window,
    /// The display that the program is running in
    pub display: glium::Display<WindowSurface>,
    /// The indices for the program (there are no indices, but glium requires this to be here)
    pub indices: glium::index::NoIndices,
    shaders: FxHashMap<String, glium::Program>,
    dimensions: [u32; 2],
    pub debug: DebugOption,
    pub render_settings: RenderSettings,
}

impl LumenpyxProgram {
    /// Create a new program with the given resolution and name
    pub fn new(resolution: [u32; 2], name: &str) -> (LumenpyxProgram, EventLoop<()>) {
        let (event_loop, window, display, indices) = setup_program();

        let mut program = LumenpyxProgram {
            window,
            display,
            indices,
            shaders: FxHashMap::default(),
            dimensions: resolution,
            debug: DebugOption::None,
            render_settings: RenderSettings {
                shadows: true,
                reflections: true,
                render_resolution: None,
                blur_reflections: false,
                blur_strength: 0.01,
            },
        };

        program.set_name(name);

        shaders::load_all_system_shaders(&mut program);

        (program, event_loop)
    }

    /// Add a shader to the program with the given name
    pub fn add_shader(&mut self, program: glium::Program, name: &str) {
        self.shaders.insert(name.to_string(), program);
    }

    /// Get a shader from the program with the given name
    pub fn get_shader(&self, name: &str) -> Option<&glium::Program> {
        self.shaders.get(name)
    }

    /// Remove a shader from the program
    pub fn remove_shader(&mut self, name: &str) {
        self.shaders.remove(name);
    }

    /// Set the name of the window
    pub fn set_name(&mut self, name: &str) {
        self.window.set_title(name);
    }

    /// Set the debug option of the program
    pub fn set_debug(&mut self, debug: DebugOption) {
        self.debug = debug;
    }

    /// Set the render settings of the program
    pub fn set_render_settings(&mut self, settings: RenderSettings) {
        self.render_settings = settings;
    }

    /// Set the resolution of the program
    pub fn set_resolution(&mut self, resolution: [u32; 2]) {
        self.dimensions = resolution;
    }

    /// run the program with the given update function
    pub fn run<F>(&mut self, event_loop: EventLoop<()>, mut update: F)
    where
        F: FnMut(&mut Self),
    {
        event_loop
            .run(move |ev, window_target| match ev {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    winit::event::WindowEvent::Resized(physical_size) => {
                        self.display.resize(physical_size.into());
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        update(self);
                    }
                    _ => (),
                },
                winit::event::Event::AboutToWait => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    self.window.request_redraw();
                }
                _ => (),
            })
            .unwrap();
    }
}

/// The transform struct is used to determine the position and scale of an object
#[derive(Copy, Clone)]
pub struct Transform {
    matrix: [[f32; 4]; 4],
}

impl Transform {
    pub fn new(pos: [f32; 3]) -> Transform {
        Transform {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [pos[0], pos[1], pos[2], 1.0],
            ],
        }
    }

    /// Get the matrix of the transform
    pub fn get_matrix(&self) -> [[f32; 4]; 4] {
        self.matrix
    }

    /// Set the matrix of the transform
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[3][0] = x;
        self.matrix[3][1] = y;
        self.matrix[3][2] = z;
    }

    /// set the scale of the transform
    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[0][0] = x;
        self.matrix[1][1] = y;
        self.matrix[2][2] = z;
    }

    /// set the x position of the transform
    pub fn set_x(&mut self, x: f32) {
        self.matrix[3][0] = x;
    }

    /// get the x position of the transform
    pub fn get_x(&self) -> f32 {
        self.matrix[3][0]
    }

    /// set the y position of the transform
    pub fn set_y(&mut self, y: f32) {
        self.matrix[3][1] = y;
    }

    /// get the y position of the transform
    pub fn get_y(&self) -> f32 {
        self.matrix[3][1]
    }

    /// set the z position of the transform
    pub fn set_z(&mut self, z: f32) {
        self.matrix[3][2] = z;
    }

    /// get the z position of the transform
    pub fn get_z(&self) -> f32 {
        self.matrix[3][2]
    }
}

/// The vertex struct for the program.
/// This doesn't need to be messed with unless you are making a custom shader
/// that doesn't use the full screen quad.
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

/// Setup the program with the window and display
pub(crate) fn setup_program() -> (
    EventLoop<()>,
    winit::window::Window,
    glium::Display<WindowSurface>,
    glium::index::NoIndices,
) {
    // this is just a wrapper for the setup_window function for now
    let (event_loop, display, window, indices) = setup_window();

    (event_loop, window, display, indices)
}

fn load_image(path: &str) -> glium::texture::RawImage2d<f32> {
    let img = image::open(path).expect(format!("Failed to load image at path {}", path,).as_str());
    img.flipv();
    let path = format!("{}", path);
    let image = image::load(
        std::io::Cursor::new(std::fs::read(path).unwrap()),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba32f();
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

/// The camera struct is used to determine the position of the camera
pub struct Camera {
    pub position: [f32; 3],
}

impl Camera {
    pub fn new(position: [f32; 3]) -> Camera {
        Camera { position }
    }
}

pub struct RenderSettings {
    shadows: bool,
    reflections: bool,
    /// The resolution that the program is rendering at
    /// this is different from the window resolution,
    /// the program will only show window resolution pixels,
    /// set the render resolution to a higher value for reflecting things that are off screen
    render_resolution: Option<[u32; 2]>,
    blur_reflections: bool,
    blur_strength: f32,
}

impl Default for RenderSettings {
    fn default() -> Self {
        RenderSettings {
            shadows: true,
            reflections: true,
            render_resolution: None,
            blur_reflections: false,
            blur_strength: 0.01,
        }
    }
}

impl RenderSettings {
    pub fn with_shadows(mut self, shadows: bool) -> Self {
        self.shadows = shadows;
        self
    }

    pub fn with_reflections(mut self, reflections: bool) -> Self {
        self.reflections = reflections;
        self
    }

    pub fn with_render_resolution(mut self, resolution: [u32; 2]) -> Self {
        self.render_resolution = Some(resolution);
        self
    }

    pub fn with_blur_reflections(mut self, blur: bool) -> Self {
        self.blur_reflections = blur;
        self
    }

    pub fn with_blur_strength(mut self, strength: f32) -> Self {
        self.blur_strength = strength;
        self
    }
}

/// Draw everything to the screen
pub fn draw_all(
    lights: Vec<&dyn lights::LightDrawable>,
    drawables: Vec<&dyn Drawable>,
    program: &mut LumenpyxProgram,
    camera: &Camera,
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
        render every normal to a texture

        find the difference between the last frame and this frame
        use this to color the different pixels with the shadow strength
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
    let debug = &program.debug;
    let render_settings = &program.render_settings;
    let render_resolution = render_settings
        .render_resolution
        .unwrap_or(program.dimensions);
    if render_resolution < program.dimensions {
        panic!("Render resolution must be greater than or equal to the window resolution");
    }

    let albedo_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        render_resolution[0],
        render_resolution[1],
    )
    .unwrap();

    let height_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        render_resolution[0],
        render_resolution[1],
    )
    .unwrap();

    let normal_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        render_resolution[0],
        render_resolution[1],
    )
    .unwrap();

    let roughness_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        render_resolution[0],
        render_resolution[1],
    )
    .unwrap();

    let shadow_strength_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        render_resolution[0],
        render_resolution[1],
    )
    .unwrap();

    {
        let last_drawable_texture = glium::texture::Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            render_resolution[0],
            render_resolution[1],
        )
        .unwrap();

        let last_drawable_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &last_drawable_texture).unwrap();

        let last_drawable_sampler =
            glium::uniforms::Sampler(&last_drawable_texture, DEFAULT_BEHAVIOR);

        let this_drawable_sampler = glium::uniforms::Sampler(&albedo_texture, DEFAULT_BEHAVIOR);

        let mut albedo_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &albedo_texture).unwrap();

        let mut height_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &height_texture).unwrap();

        let mut roughness_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &roughness_texture).unwrap();

        let mut normal_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &normal_texture).unwrap();

        normal_framebuffer.clear_color(0.0, 0.0, 1.0, 1.0);

        let mut shadow_strength_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &shadow_strength_texture).unwrap();

        for drawable in &drawables {
            let mut new_matrix = drawable.get_position();
            // scale off the resolution
            if render_resolution[0] > render_resolution[1] {
                new_matrix[0][0] *= render_resolution[1] as f32 / render_resolution[0] as f32;
            } else {
                new_matrix[1][1] *= render_resolution[0] as f32 / render_resolution[1] as f32;
            }

            // adjust off the camera no need to translate the z, it would just mess up the height map's interaction with the light
            new_matrix[3][0] -= camera.position[0];
            new_matrix[3][1] -= camera.position[1];

            new_matrix[3][0] /= render_resolution[0] as f32;
            new_matrix[3][1] /= render_resolution[1] as f32;

            // because its -1 to 1, we need to multiply by 2
            new_matrix[3][0] *= 2.0;
            new_matrix[3][1] *= 2.0;

            drawable.draw(
                program,
                new_matrix,
                &mut albedo_framebuffer,
                &mut height_framebuffer,
                &mut roughness_framebuffer,
                &mut normal_framebuffer,
            );

            if render_settings.shadows {
                let shadow_strength = drawable.get_recieve_shadows_strength();

                shaders::draw_recieve_shadows(
                    &mut shadow_strength_framebuffer,
                    &program,
                    shadow_strength,
                    last_drawable_sampler,
                    this_drawable_sampler,
                );

                // copy the albedo to the last drawable framebuffer
                albedo_framebuffer.blit_whole_color_to(
                    &last_drawable_framebuffer,
                    &glium::BlitTarget {
                        left: 0,
                        bottom: 0,
                        width: render_resolution[0] as i32,
                        height: render_resolution[1] as i32,
                    },
                    glium::uniforms::MagnifySamplerFilter::Nearest,
                );
            }
        }
    }

    let lit_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        render_resolution[0],
        render_resolution[1],
    )
    .expect("Failed to create lit frame buffer");

    if render_settings.shadows {
        let albedo = glium::uniforms::Sampler(&albedo_texture, DEFAULT_BEHAVIOR);
        let height_sampler = glium::uniforms::Sampler(&height_texture, DEFAULT_BEHAVIOR);
        let roughness_sampler = glium::uniforms::Sampler(&roughness_texture, DEFAULT_BEHAVIOR);
        let shadow_strength_sampler =
            glium::uniforms::Sampler(&shadow_strength_texture, DEFAULT_BEHAVIOR);

        let mut lit_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &lit_texture).unwrap();

        for light in lights {
            let mut new_matrix = light.get_transform();
            if render_resolution[0] > render_resolution[1] {
                new_matrix[0][0] *= render_resolution[1] as f32 / render_resolution[0] as f32;
            } else {
                new_matrix[1][1] *= render_resolution[0] as f32 / render_resolution[1] as f32;
            }
            // adjust off the camera no need to translate the z, it would just mess up the height map's interaction with the light
            new_matrix[3][0] -= camera.position[0];
            new_matrix[3][1] -= camera.position[1];

            new_matrix[3][0] /= render_resolution[0] as f32;
            new_matrix[3][1] /= render_resolution[1] as f32;

            // because its -1 to 1, we need to multiply by 2
            new_matrix[3][0] *= 2.0;
            new_matrix[3][1] *= 2.0;

            light.draw(
                program,
                new_matrix,
                &mut lit_framebuffer,
                height_sampler,
                albedo,
                roughness_sampler,
                shadow_strength_sampler,
            );
        }
    }

    let reflected_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        render_resolution[0],
        render_resolution[1],
    )
    .expect("Failed to create reflected frame buffer");

    if render_settings.reflections {
        let roughness = glium::uniforms::Sampler(&roughness_texture, DEFAULT_BEHAVIOR);
        let height = glium::uniforms::Sampler(&height_texture, DEFAULT_BEHAVIOR);
        let normal = glium::uniforms::Sampler(&normal_texture, DEFAULT_BEHAVIOR);
        let lit_sampler = if render_settings.shadows {
            glium::uniforms::Sampler(&lit_texture, DEFAULT_BEHAVIOR)
        } else {
            glium::uniforms::Sampler(&albedo_texture, DEFAULT_BEHAVIOR)
        };

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
        let finished_texture = match debug {
            DebugOption::None => glium::uniforms::Sampler(&reflected_texture, DEFAULT_BEHAVIOR),
            DebugOption::Albedo => glium::uniforms::Sampler(&albedo_texture, DEFAULT_BEHAVIOR),
            DebugOption::Height => glium::uniforms::Sampler(&height_texture, DEFAULT_BEHAVIOR),
            DebugOption::Roughness => {
                glium::uniforms::Sampler(&roughness_texture, DEFAULT_BEHAVIOR)
            }
            DebugOption::Normal => glium::uniforms::Sampler(&normal_texture, DEFAULT_BEHAVIOR),
            DebugOption::ShadowStrength => {
                glium::uniforms::Sampler(&shadow_strength_texture, DEFAULT_BEHAVIOR)
            }
        };

        draw_upscale(finished_texture, &program, program.dimensions);
    }
}
