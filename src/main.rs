use std::fs;
use std::os::windows::thread;

use glium;
use glium::framebuffer::SimpleFrameBuffer;
use glium::glutin::surface::WindowSurface;
use glium::implement_vertex;
use glium::uniform;
use glium::uniforms::AsUniformValue;
use glium::Surface;
use winit;
use winit::event::Event;
use winit::event_loop;
use winit::event_loop::EventLoop;

const WINDOW_VIRTUAL_SIZE: (u32, u32) = (128, 128);
const DEFAULT_BEHAVIOR: glium::uniforms::SamplerBehavior = glium::uniforms::SamplerBehavior {
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

struct DrawableObject {
    albedo_texture: glium::texture::Texture2d,
    height_texture: glium::texture::Texture2d,
    roughness_texture: glium::texture::Texture2d,
    transform: Transform,
}

impl DrawableObject {
    fn new(
        albedo_path: &str,
        height_path: &str,
        roughness_path: &str,
        display: &glium::Display<WindowSurface>,
        transform: Transform,
    ) -> DrawableObject {
        let albedo_image = load_image(albedo_path);
        let albedo_texture = glium::texture::Texture2d::new(display, albedo_image).unwrap();

        let height_image = load_image(height_path);
        let height_texture = glium::texture::Texture2d::new(display, height_image).unwrap();

        let roughness_image = load_image(roughness_path);
        let roughness_texture = glium::texture::Texture2d::new(display, roughness_image).unwrap();

        DrawableObject {
            albedo_texture,
            height_texture,
            roughness_texture,
            transform,
        }
    }
}

#[derive(Copy, Clone)]
struct Transform {
    matrix: [[f32; 4]; 4],
}

impl Transform {
    fn new() -> Transform {
        Transform {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[3][0] = x;
        self.matrix[3][1] = y;
        self.matrix[3][2] = z;
    }

    fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[0][0] = x;
        self.matrix[1][1] = y;
        self.matrix[2][2] = z;
    }

    fn set_x(&mut self, x: f32) {
        self.matrix[3][0] = x;
    }

    fn set_y(&mut self, y: f32) {
        self.matrix[3][1] = y;
    }

    fn set_z(&mut self, z: f32) {
        self.matrix[3][2] = z;
    }
}

#[derive(Copy, Clone)]
struct Light {
    position: [f32; 3],
    color: [f32; 3],
    intensity: f32,
    falloff: f32,
}

impl Light {
    fn new() -> Light {
        Light {
            position: [0.0, 0.0, 0.0],
            color: [1.0, 1.0, 1.0],
            intensity: 1.0,
            falloff: 1.0,
        }
    }

    fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = [x, y, z];
    }

    /// Set the color of the light in 0.0 - 1.0 range
    fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }

    fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    fn set_falloff(&mut self, falloff: f32) {
        self.falloff = falloff;
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

#[allow(unused_variables)]
fn main() {
    let (event_loop, display, window, indices) = setup();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 0.0);

    let paths = vec![
        "images/bricks_pixelated.png",
        "images/test_sphere_heightmap.png",
        "images/Border_Heightmap_Test.png",
    ];

    let mut drawables = vec![];
    let mut lights = vec![Light {
        position: [0.5, 1.0, 1.0],
        color: [1.0, 1.0, 1.0],
        intensity: 2.0,
        falloff: 0.01,
    }];

    for path in paths {
        let drawable = DrawableObject::new(path, path, path, &display, Transform::new());
        drawables.push(drawable);
    }

    let mut t: f32 = 0.0;
    event_loop
        .run(move |ev, window_target| match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                winit::event::WindowEvent::Resized(physical_size) => {
                    display.resize(physical_size.into());
                }
                winit::event::WindowEvent::RedrawRequested => {
                    {
                        t += 0.001;
                        //drawables[1].transform.set_x(t.sin() * 0.5);
                        lights[0].position = [(t.sin() + 1.0) / 2.0, 0.5, 1.0];
                    }

                    let drawable_refs: Vec<&DrawableObject> = drawables.iter().collect();
                    let light_refs: Vec<&Light> = lights.iter().collect();
                    draw_all(&display, drawable_refs, light_refs, &indices);
                }
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => (),
        })
        .unwrap();
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

fn setup() -> (
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

fn draw_all(
    display: &glium::Display<WindowSurface>,
    drawables: Vec<&DrawableObject>,
    lights: Vec<&Light>,
    indices: &glium::index::NoIndices,
) {
    // STEP 1:
    // render every albedo to a texture
    // render every height to a texture
    // render every roughness to a texture
    // STEP 2:
    // take the textures and feed it into a lighting shader
    // we do this for every light and then blend the results together
    // STEP 3:
    // upscale the result to the screen size

    let albedo_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        WINDOW_VIRTUAL_SIZE.0,
        WINDOW_VIRTUAL_SIZE.1,
    )
    .unwrap();

    let height_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        WINDOW_VIRTUAL_SIZE.0,
        WINDOW_VIRTUAL_SIZE.1,
    )
    .unwrap();

    let roughness_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        WINDOW_VIRTUAL_SIZE.0,
        WINDOW_VIRTUAL_SIZE.1,
    )
    .unwrap();

    {
        let mut albedo_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &albedo_texture).unwrap();
        albedo_framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);

        let mut height_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &height_texture).unwrap();
        height_framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);
        let mut roughness_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &roughness_texture).unwrap();
        roughness_framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);

        for drawable in &drawables {
            draw_ahr(
                &display,
                drawable,
                &indices,
                &mut albedo_framebuffer,
                &mut height_framebuffer,
                &mut roughness_framebuffer,
            );
        }
    }

    let lit_texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        WINDOW_VIRTUAL_SIZE.0,
        WINDOW_VIRTUAL_SIZE.1,
    )
    .unwrap();

    {
        let albedo = glium::uniforms::Sampler(&albedo_texture, DEFAULT_BEHAVIOR);
        let height = glium::uniforms::Sampler(&height_texture, DEFAULT_BEHAVIOR);
        let roughness = glium::uniforms::Sampler(&roughness_texture, DEFAULT_BEHAVIOR);

        let mut lit_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &lit_texture).unwrap();

        for light in &lights {
            draw_lighting(
                &display,
                albedo,
                height,
                roughness,
                light,
                &indices,
                &mut lit_framebuffer,
            );
        }
    }

    {
        let behavior = glium::uniforms::SamplerBehavior {
            minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
            max_anisotropy: 1,
            ..Default::default()
        };

        let finished_texture = glium::uniforms::Sampler(&lit_texture, behavior);
        draw_upscale(&display, finished_texture, &indices);
    }
}

/// draw the albedo, height, or roughness
fn draw_ahr(
    display: &glium::Display<WindowSurface>,
    drawable: &DrawableObject,
    indices: &glium::index::NoIndices,
    albedo_framebuffer: &mut SimpleFrameBuffer,
    height_framebuffer: &mut SimpleFrameBuffer,
    roughness_framebuffer: &mut SimpleFrameBuffer,
) {
    let vertex_shader_src = fs::read_to_string("shaders/base_shader.vert").unwrap();
    let vertex_shader_src = vertex_shader_src.as_str();

    let fragment_shader_src = fs::read_to_string("shaders/base_shader.frag").unwrap();
    let fragment_shader_src = fragment_shader_src.as_str();

    let program =
        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

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

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let matrix = drawable.transform.matrix;

    let mut image = glium::uniforms::Sampler(&drawable.albedo_texture, DEFAULT_BEHAVIOR);

    let uniform = &uniform! {
        matrix: matrix,
        image: image,
    };
    albedo_framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &program,
            uniform,
            &Default::default(),
        )
        .unwrap();

    image = glium::uniforms::Sampler(&drawable.height_texture, DEFAULT_BEHAVIOR);
    let uniform = &uniform! {
        matrix: matrix,
        image: image,
    };
    height_framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &program,
            uniform,
            &Default::default(),
        )
        .unwrap();

    image = glium::uniforms::Sampler(&drawable.roughness_texture, DEFAULT_BEHAVIOR);
    let uniform = &uniform! {
        matrix: matrix,
        image: image,
    };
    roughness_framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &program,
            uniform,
            &Default::default(),
        )
        .unwrap();
}

/// upscale the result to the screen size
fn draw_upscale(
    display: &glium::Display<WindowSurface>,
    image_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    indices: &glium::index::NoIndices,
) {
    let vertex_shader_src = fs::read_to_string("shaders/upscale_shader.vert").unwrap();
    let vertex_shader_src = vertex_shader_src.as_str();

    let fragment_shader_src = fs::read_to_string("shaders/upscale_shader.frag").unwrap();
    let fragment_shader_src = fragment_shader_src.as_str();

    let program =
        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

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

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let uniforms = &uniform! {
        image: image_uniform
    };

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 0.0);
    target
        .draw(
            &vertex_buffer,
            indices,
            &program,
            uniforms,
            &Default::default(),
        )
        .unwrap();

    target.finish().unwrap();
}

/// draw the lighting
fn draw_lighting(
    display: &glium::Display<WindowSurface>,
    albedo_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    height_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    roughness_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    light: &Light,
    indices: &glium::index::NoIndices,
    framebuffer: &mut SimpleFrameBuffer,
) {
    let vertex_shader_src = fs::read_to_string("shaders/lighting.vert").unwrap();
    let vertex_shader_src = vertex_shader_src.as_str();

    let fragment_shader_src = fs::read_to_string("shaders/lighting.frag").unwrap();
    let fragment_shader_src = fragment_shader_src.as_str();

    let program =
        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // this is doubled
    // why you might ask? Absolutely no clue
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

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    //let light_pos = ;

    let uniforms = &uniform! {
        roughnessmap: roughness_uniform,
        heightmap: height_uniform,
        albedomap: albedo_uniform,
        light_pos: light.position,
        light_color: light.color,
        light_intensity: light.intensity,
        light_falloff: light.falloff,
    };

    framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);
    framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &program,
            uniforms,
            &Default::default(),
        )
        .unwrap();
}
