use glium;
use glium::glutin::surface::WindowSurface;
use glium::implement_vertex;
use glium::Frame;
use glium::Surface;
use parley::fontique::Collection;
use parley::FontContext;
use parley::LayoutContext;
use primitives::Texture;
use swash::scale::ScaleContext;
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
pub mod blending;
/// This module contains all the lights that can be used in the program
/// As well as containing the trait that all lights must implement
pub mod lights;
pub mod text;

const HANDLE_STRING_ID: &str = "wdAYG8&DWtyiwDhukhjwda";

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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureHandle {
    id: u32,
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
    cache: LumenpyxCache,
    next_texture_id: u32,
    dimensions: [u32; 2],
    pub debug: DebugOption,
    pub render_settings: RenderSettings,
    font_context: Option<FontContext>,
    scale_context: Option<ScaleContext>,
    layout_context: Option<LayoutContext>,
}

impl LumenpyxProgram {
    /// Create a new program with the given resolution and name
    pub fn new(resolution: [u32; 2], name: &str) -> (LumenpyxProgram, EventLoop<()>) {
        let (event_loop, window, display, indices) = setup_program();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let mut program = LumenpyxProgram {
            window,
            display,
            indices,
            shaders: FxHashMap::default(),
            next_texture_id: 0,
            cache: LumenpyxCache::default(),
            dimensions: resolution,
            debug: DebugOption::None,
            render_settings: RenderSettings {
                shadows: true,
                reflections: true,
                render_resolution: None,
                blur_reflections: false,
                blur_strength: 0.01,
            },
            font_context: None,
            scale_context: None,
            layout_context: None,
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

    /// Add a texture to the program with the given name
    pub fn add_texture(&mut self, texture: glium::texture::Texture2d, name: &str) {
        self.cache.insert(name.to_string(), texture);
    }

    /// Get a texture from the program with the given name
    pub fn get_texture(&self, name: &str) -> Option<&glium::texture::Texture2d> {
        self.cache.get_texture(name)
    }

    /// Get a texture from a texture handle
    pub fn get_texture_from_handle(
        &self,
        handle: &TextureHandle,
    ) -> Option<&glium::texture::Texture2d> {
        self.cache
            .get_texture(&format!("{}_{}", HANDLE_STRING_ID, handle.id))
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
                    // RedrawRequested will only when we resize the window, so we need to manually
                    // request it.
                    self.window.request_redraw();
                }
                _ => (),
            })
            .expect("Failed to run event loop");
    }

    pub fn get_dimensions(&self) -> [u32; 2] {
        self.dimensions
    }

    pub(crate) fn get_render_resolution(&self) -> [u32; 2] {
        self.render_settings
            .render_resolution
            .unwrap_or(self.dimensions)
    }

    pub(crate) fn adjust_transform_for_drawable(
        &self,
        mut transform: &Transform,
        camera: &Camera,
    ) -> Transform {
        let render_resolution = self.get_render_resolution();
        let mut new_transform = transform.clone();

        let mut scale = transform.get_scale();
        // scale off the resolution
        if render_resolution[0] > render_resolution[1] {
            scale[0] *= render_resolution[1] as f32 / render_resolution[0] as f32;
        } else {
            scale[1] *= render_resolution[0] as f32 / render_resolution[1] as f32;
        }
        new_transform.set_scale(scale[0], scale[1], scale[2]);

        let (mut x, mut y, z) = (transform.get_x(), transform.get_y(), transform.get_z());
        // adjust off the camera no need to translate the z, it would just mess up the height map's interaction with the light
        x -= camera.position[0];
        y -= camera.position[1];

        x /= render_resolution[0] as f32;
        y /= render_resolution[1] as f32;

        // because its -1 to 1, we need to multiply by 2
        x *= 2.0;
        y *= 2.0;

        new_transform.translate(x, y, z);

        new_transform
    }

    /// Add a texture to the program without a name, returns a handle to the texture
    pub fn add_not_named_texture(&mut self, texture: glium::texture::Texture2d) -> TextureHandle {
        let id = self.next_texture_id;
        self.next_texture_id += 1;

        self.cache
            .insert(format!("{}_{}", HANDLE_STRING_ID, id), texture);

        TextureHandle { id }
    }

    pub fn remove_texture(&mut self, handle: &TextureHandle) {
        self.cache
            .hashmap
            .remove(&format!("{}_{}", HANDLE_STRING_ID, handle.id));
    }

    pub fn set_font_collection(
        &mut self,
        font_collection: Collection,
        lumenpyx_program: &mut crate::LumenpyxProgram,
    ) {
        if let Some(font_context) = &mut lumenpyx_program.font_context {
            font_context.collection = font_collection;
        } else {
            let mut new_font_context = FontContext::default();
            new_font_context.collection = font_collection;
            lumenpyx_program.font_context = Some(new_font_context);
        }
    }

    pub fn add_font_to_collection(&mut self, font: Vec<u8>) {
        if let Some(font_context) = &mut self.font_context {
            font_context.collection.register_fonts(font);
        } else {
            let mut new_font_context = FontContext::default();
            new_font_context.collection.register_fonts(font);
            self.font_context = Some(new_font_context);
        }
    }

    pub fn get_font_collection(&self) -> Option<&Collection> {
        if let Some(font_context) = &self.font_context {
            Some(&font_context.collection)
        } else {
            None
        }
    }

    pub fn get_font_context(&self) -> Option<&FontContext> {
        self.font_context.as_ref()
    }

    pub fn get_font_context_mut(&mut self) -> Option<&mut FontContext> {
        self.font_context.as_mut()
    }
}

struct LumenpyxCache {
    hashmap: FxHashMap<String, glium::Texture2d>,
}

impl Default for LumenpyxCache {
    fn default() -> Self {
        LumenpyxCache {
            hashmap: FxHashMap::default(),
        }
    }
}

impl LumenpyxCache {
    fn insert(&mut self, name: String, texture: glium::Texture2d) {
        self.hashmap.insert(name, texture);
    }

    fn get_texture(&self, name: &str) -> Option<&glium::Texture2d> {
        let texture = self.hashmap.get(name);

        if let Some(texture) = texture {
            Some(texture)
        } else {
            None
        }
    }
}

/// The transform struct is used to determine the position and scale of an object
#[derive(Copy, Clone)]
pub struct Transform {
    matrix: [[f32; 4]; 4],
    rotation: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Transform::new([0.0, 0.0, 0.0])
    }
}

impl Transform {
    pub fn from_matrix(matrix: [[f32; 4]; 4]) -> Transform {
        Transform {
            matrix,
            rotation: 0.0,
        }
    }

    pub fn new(pos: [f32; 3]) -> Transform {
        Transform {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [pos[0], pos[1], pos[2], 1.0],
            ],
            rotation: 0.0,
        }
    }

    /// Get the matrix of the transform
    pub fn get_matrix(&self) -> [[f32; 4]; 4] {
        let rotation_matrix = [
            [self.rotation.cos(), -self.rotation.sin(), 0.0, 0.0],
            [self.rotation.sin(), self.rotation.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        multiply_matrix(rotation_matrix, self.matrix)
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

    /// get the scale of the transform
    pub fn get_scale(&self) -> [f32; 3] {
        [self.matrix[0][0], self.matrix[1][1], self.matrix[2][2]]
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

    /// set the rotation of the transform
    pub fn set_rotation(&mut self, angle: f32) {
        self.rotation = angle;
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn add_parent(&self, parent: &Transform) -> Transform {
        let mut new_transform = Transform::new([0.0, 0.0, 0.0]);

        new_transform.set_x(self.get_x() + parent.get_x());
        new_transform.set_y(self.get_y() + parent.get_y());
        new_transform.set_z(self.get_z() + parent.get_z());

        new_transform.set_rotation(self.get_rotation() + parent.get_rotation());

        new_transform.set_scale(
            self.get_scale()[0] * parent.get_scale()[0],
            self.get_scale()[1] * parent.get_scale()[1],
            self.get_scale()[2] * parent.get_scale()[2],
        );

        new_transform
    }
}

fn multiply_matrix(matrix1: [[f32; 4]; 4], matrix2: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut new_matrix = [[0.0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                new_matrix[i][j] = new_matrix[i][j] + matrix1[i][k] * matrix2[k][j];
            }
        }
    }

    new_matrix
}

/// a is the parent
impl<'a, 'b> std::ops::Add<&'b Transform> for &'a Transform {
    type Output = Transform;

    fn add(self, other: &'b Transform) -> Transform {
        let mut new_transform = Transform::new([0.0, 0.0, 0.0]);

        new_transform.translate(
            self.get_x() + other.get_x(),
            self.get_y() + other.get_y(),
            self.get_z() + other.get_z(),
        );

        new_transform.set_rotation(self.get_rotation() + other.get_rotation());

        new_transform.set_scale(
            self.get_x() * other.get_x(),
            self.get_y() * other.get_y(),
            self.get_z() * other.get_z(),
        );

        new_transform
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
        std::io::Cursor::new(std::fs::read(path).expect("Failed to read image")),
        image::ImageFormat::Png,
    )
    .expect("Failed to load image")
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
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();

    match event_loop {
        Ok(event_loop) => {
            // 2. Create a glutin context and glium Display
            let (window, display) =
                glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

            (event_loop, display, window, indices)
        }
        Err(error) => panic!("Failed to create event loop: {}", error),
    }
}

/// The camera struct is used to determine the position of the camera
#[derive(Copy, Clone)]
pub struct Camera {
    pub position: [f32; 3],
}

impl Camera {
    /// the z position of the camera is just for reflection purposes
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

    // default is 0.01
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
    load_all_textures(program);

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

    let reflected_texture = program
        .get_texture("reflected_texture")
        .expect("Failed to get reflected texture");

    let (
        albedo_texture,
        normal_texture,
        height_texture,
        roughness_texture,
        shadow_strength_texture,
    ) = draw_all_no_post(drawables, program, camera);

    let lit_texture = draw_lighting(
        lights,
        program,
        camera,
        &albedo_texture,
        &height_texture,
        &roughness_texture,
        &shadow_strength_texture,
    );

    let display = &program.display;
    let debug = &program.debug;
    let render_settings = &program.render_settings;
    let render_resolution = render_settings
        .render_resolution
        .unwrap_or(program.dimensions);
    if render_resolution < program.dimensions {
        panic!("Render resolution must be greater than or equal to the window resolution");
    }
    if render_settings.reflections {
        let roughness = glium::uniforms::Sampler(roughness_texture, DEFAULT_BEHAVIOR);
        let height = glium::uniforms::Sampler(height_texture, DEFAULT_BEHAVIOR);
        let normal = glium::uniforms::Sampler(normal_texture, DEFAULT_BEHAVIOR);
        let lit_sampler = if render_settings.shadows {
            glium::uniforms::Sampler(lit_texture, DEFAULT_BEHAVIOR)
        } else {
            glium::uniforms::Sampler(albedo_texture, DEFAULT_BEHAVIOR)
        };

        let mut reflected_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, reflected_texture)
                .expect("Failed to create reflected frame buffer");

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
        let reflected_texture = if render_settings.reflections {
            reflected_texture
        } else if render_settings.shadows {
            &lit_texture
        } else {
            &albedo_texture
        };

        let finished_texture = match debug {
            DebugOption::None => glium::uniforms::Sampler(reflected_texture, DEFAULT_BEHAVIOR),
            DebugOption::Albedo => glium::uniforms::Sampler(albedo_texture, DEFAULT_BEHAVIOR),
            DebugOption::Height => glium::uniforms::Sampler(height_texture, DEFAULT_BEHAVIOR),
            DebugOption::Roughness => glium::uniforms::Sampler(roughness_texture, DEFAULT_BEHAVIOR),
            DebugOption::Normal => glium::uniforms::Sampler(normal_texture, DEFAULT_BEHAVIOR),
            DebugOption::ShadowStrength => {
                glium::uniforms::Sampler(shadow_strength_texture, DEFAULT_BEHAVIOR)
            }
        };

        draw_upscale(finished_texture, &program, program.dimensions);
    }
}

fn load_all_textures(program: &mut LumenpyxProgram) {
    let display = &program.display;
    let render_settings = &program.render_settings;
    let render_resolution = render_settings
        .render_resolution
        .unwrap_or(program.dimensions);
    if render_resolution < program.dimensions {
        panic!("Render resolution must be greater than or equal to the window resolution");
    }

    let albedo_texture = program.cache.get_texture("albedo_texture");
    if albedo_texture.is_none() {
        let albedo_texture_owned = glium::texture::Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            render_resolution[0],
            render_resolution[1],
        )
        .expect("Failed to create albedo texture");

        program
            .cache
            .insert("albedo_texture".to_string(), albedo_texture_owned);
    }

    let height_texture = program.cache.get_texture("height_texture");
    if height_texture.is_none() {
        let height_texture_owned = glium::texture::Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            render_resolution[0],
            render_resolution[1],
        )
        .expect("Failed to create height texture");

        program
            .cache
            .insert("height_texture".to_string(), height_texture_owned);
    }

    let normal_texture = program.cache.get_texture("normal_texture");
    if normal_texture.is_none() {
        let normal_texture_owned = glium::texture::Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            render_resolution[0],
            render_resolution[1],
        )
        .expect("Failed to create normal texture");

        program
            .cache
            .insert("normal_texture".to_string(), normal_texture_owned);
    }

    let roughness_texture = program.cache.get_texture("roughness_texture");
    if roughness_texture.is_none() {
        let roughness_texture_owned = glium::texture::Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            render_resolution[0],
            render_resolution[1],
        )
        .expect("Failed to create roughness texture");

        program
            .cache
            .insert("roughness_texture".to_string(), roughness_texture_owned);
    }

    let shadow_strength_texture = program.cache.get_texture("shadow_strength_texture");
    if shadow_strength_texture.is_none() {
        let shadow_strength_texture_owned = glium::texture::Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            render_resolution[0],
            render_resolution[1],
        )
        .expect("Failed to create shadow strength texture");

        program.cache.insert(
            "shadow_strength_texture".to_string(),
            shadow_strength_texture_owned,
        );
    }

    let last_drawable_texture = program.cache.get_texture("last_drawable_texture");
    if last_drawable_texture.is_none() {
        let last_drawable_texture_owned = glium::texture::Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            render_resolution[0],
            render_resolution[1],
        )
        .expect("Failed to create last drawable texture");

        program.cache.insert(
            "last_drawable_texture".to_string(),
            last_drawable_texture_owned,
        );
    }

    let reflected_texture = program.get_texture("reflected_texture");
    if reflected_texture.is_none() {
        let reflected_texture_owned = glium::texture::Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            render_resolution[0],
            render_resolution[1],
        )
        .expect("Failed to create reflected frame buffer");

        program
            .cache
            .insert("reflected_texture".to_string(), reflected_texture_owned);
    }

    let reflection_texture = program.get_texture("reflection_texture");
    if reflection_texture.is_none() {
        let reflection_texture_owned = glium::texture::Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            render_resolution[0],
            render_resolution[1],
        )
        .expect("Failed to create reflection frame buffer");

        program
            .cache
            .insert("reflection_texture".to_string(), reflection_texture_owned);
    }

    let lit_texture = program.get_texture("lit_texture");
    if lit_texture.is_none() {
        let lit_texture_owned = glium::texture::Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            render_resolution[0],
            render_resolution[1],
        )
        .expect("Failed to create lit frame buffer");

        program
            .cache
            .insert("lit_texture".to_string(), lit_texture_owned);
    }
}

fn draw_all_no_post<'a>(
    drawables: Vec<&dyn Drawable>,
    program: &'a LumenpyxProgram,
    camera: &Camera,
) -> (
    &'a glium::Texture2d,
    &'a glium::Texture2d,
    &'a glium::Texture2d,
    &'a glium::Texture2d,
    &'a glium::Texture2d,
) {
    let display = &program.display;
    let render_settings = &program.render_settings;
    let render_resolution = render_settings
        .render_resolution
        .unwrap_or(program.dimensions);
    if render_resolution < program.dimensions {
        panic!("Render resolution must be greater than or equal to the window resolution");
    }

    let albedo_texture = program
        .get_texture("albedo_texture")
        .expect("Failed to get albedo texture");

    let height_texture = program
        .cache
        .get_texture("height_texture")
        .expect("Failed to get height texture");

    let normal_texture = program
        .cache
        .get_texture("normal_texture")
        .expect("Failed to get normal texture");

    let roughness_texture = program
        .cache
        .get_texture("roughness_texture")
        .expect("Failed to get roughness texture");

    let shadow_strength_texture = program
        .cache
        .get_texture("shadow_strength_texture")
        .expect("Failed to get shadow strength texture");

    {
        let last_drawable_texture = program
            .cache
            .get_texture("last_drawable_texture")
            .expect("Failed to get last drawable texture");

        let mut last_drawable_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, last_drawable_texture)
                .expect("Failed to create last drawable framebuffer");

        last_drawable_framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);

        let last_drawable_sampler =
            glium::uniforms::Sampler(last_drawable_texture, DEFAULT_BEHAVIOR);

        let this_drawable_sampler = glium::uniforms::Sampler(albedo_texture, DEFAULT_BEHAVIOR);

        let mut albedo_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, albedo_texture)
                .expect("Failed to create albedo framebuffer");

        albedo_framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);

        let mut height_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, height_texture)
                .expect("Failed to create height framebuffer");

        height_framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);

        let mut roughness_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, roughness_texture)
                .expect("Failed to create roughness framebuffer");

        roughness_framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);

        let mut normal_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, normal_texture)
                .expect("Failed to create normal framebuffer");

        normal_framebuffer.clear_color(0.0, 0.0, 1.0, 0.0);

        let mut shadow_strength_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, shadow_strength_texture)
                .expect("Failed to create shadow strength framebuffer");

        shadow_strength_framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);

        for drawable in &drawables {
            let new_transform =
                program.adjust_transform_for_drawable(&drawable.get_transform(), camera);

            drawable.draw_albedo(program, &new_transform, &mut albedo_framebuffer);
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

        if render_settings.shadows || render_settings.reflections {
            for drawable in &drawables {
                let new_transform =
                    program.adjust_transform_for_drawable(&drawable.get_transform(), camera);

                drawable.draw_height(program, &new_transform, &mut height_framebuffer);
            }
        }

        if program.render_settings.reflections {
            for drawable in &drawables {
                let new_transform =
                    program.adjust_transform_for_drawable(&drawable.get_transform(), camera);

                drawable.draw_roughness(program, &new_transform, &mut roughness_framebuffer);
            }
        }

        if program.render_settings.reflections {
            for drawable in &drawables {
                let new_transform =
                    program.adjust_transform_for_drawable(&drawable.get_transform(), camera);

                drawable.draw_normal(program, &new_transform, &mut normal_framebuffer);
            }
        }
    }

    (
        albedo_texture,
        normal_texture,
        height_texture,
        roughness_texture,
        shadow_strength_texture,
    )
}

fn draw_lighting<'a>(
    lights: Vec<&dyn lights::LightDrawable>,
    program: &'a LumenpyxProgram,
    camera: &Camera,
    albedo_texture: &glium::Texture2d,
    height_texture: &glium::Texture2d,
    roughness_texture: &glium::Texture2d,
    shadow_strength_texture: &glium::Texture2d,
) -> &'a glium::Texture2d {
    let display = &program.display;
    let render_settings = &program.render_settings;
    let render_resolution = render_settings
        .render_resolution
        .unwrap_or(program.dimensions);
    if render_resolution < program.dimensions {
        panic!("Render resolution must be greater than or equal to the window resolution");
    }

    let lit_texture = program
        .get_texture("lit_texture")
        .expect("Failed to get lit texture");

    if render_settings.shadows {
        let albedo = glium::uniforms::Sampler(albedo_texture, DEFAULT_BEHAVIOR);
        let height_sampler = glium::uniforms::Sampler(height_texture, DEFAULT_BEHAVIOR);
        let roughness_sampler = glium::uniforms::Sampler(roughness_texture, DEFAULT_BEHAVIOR);
        let shadow_strength_sampler =
            glium::uniforms::Sampler(shadow_strength_texture, DEFAULT_BEHAVIOR);

        let mut lit_framebuffer = glium::framebuffer::SimpleFrameBuffer::new(display, lit_texture)
            .expect("Failed to create lit frame buffer");
        lit_framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);

        for light in lights {
            let new_transform =
                program.adjust_transform_for_drawable(&light.get_transform(), camera);

            light.draw(
                program,
                new_transform.get_matrix(),
                &mut lit_framebuffer,
                height_sampler,
                albedo,
                roughness_sampler,
                shadow_strength_sampler,
            );
        }
    }

    lit_texture
}
