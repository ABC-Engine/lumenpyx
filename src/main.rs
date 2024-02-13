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

const WINDOW_VIRTUAL_SIZE: (u32, u32) = (32, 32);

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

#[allow(unused_variables)]
fn main() {
    let (event_loop, display, window, indices) = setup();

    // STEP LIST 1:
    // render every albedo to a texture
    // render every height to a texture
    // render every roughness to a texture
    // STEP LIST 2:
    // take the textures and feed it into a lighting shader
    // we do this for every light and then blend the results together
    // STEP LIST 3:
    // upscale the result to the screen size

    let vertex_shader_src = fs::read_to_string("shaders/base_shader.vert").unwrap();
    let vertex_shader_src = vertex_shader_src.as_str();

    let fragment_shader_src = fs::read_to_string("shaders/base_shader.frag").unwrap();
    let fragment_shader_src = fragment_shader_src.as_str();

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 0.5);
    // draw the triangle here

    let behavior = glium::uniforms::SamplerBehavior {
        minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
        magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
        max_anisotropy: 1,
        ..Default::default()
    };

    let image_path = "CapeOriginal 9.png";

    let image = load_image(image_path);
    let image_texture = glium::texture::Texture2d::new(&display, image).unwrap();
    let image_uniform = glium::uniforms::Sampler(&image_texture, behavior);

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
                    draw_all(&mut t, &display, image_uniform, &indices);
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
    t: &mut f32,
    display: &glium::Display<WindowSurface>,
    image_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    indices: &glium::index::NoIndices,
) {
    let texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        WINDOW_VIRTUAL_SIZE.0,
        WINDOW_VIRTUAL_SIZE.1,
    )
    .unwrap();

    {
        let mut framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(display, &texture).unwrap();
        framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);

        draw_ahr(t, &display, image_uniform, &indices, &mut framebuffer);
    }

    {
        let behavior = glium::uniforms::SamplerBehavior {
            minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
            max_anisotropy: 1,
            ..Default::default()
        };

        let finished_texture = glium::uniforms::Sampler(&texture, behavior);
        draw_upscale(&display, finished_texture, &indices);
    }
}

// draw the albedo, height, or roughness
fn draw_ahr(
    t: &mut f32,
    display: &glium::Display<WindowSurface>,
    image_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    indices: &glium::index::NoIndices,
    framebuffer: &mut SimpleFrameBuffer,
) {
    let vertex_shader_src = fs::read_to_string("shaders/base_shader.vert").unwrap();
    let vertex_shader_src = vertex_shader_src.as_str();

    let fragment_shader_src = fs::read_to_string("shaders/base_shader.frag").unwrap();
    let fragment_shader_src = fragment_shader_src.as_str();

    let program =
        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // We update `t`
    *t += 0.001;
    // We use the sine of t as an offset, this way we get a nice smooth animation
    let x = t.sin() * 0.5;

    let shape = vec![
        Vertex {
            position: [-0.5, -0.5],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [0.5, 0.5],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [0.5, 0.5],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5],
            tex_coords: [0.0, 0.0],
        },
    ];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let uniforms = &uniform! {
    matrix: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [ x , 0.0, 0.0, 1.0f32],
        ],
        // all this is to prevent the image from being interpolated
        image: image_uniform
    };

    framebuffer.clear_color(0.0, 0.0, 1.0, 1.0);
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

// draw the albedo, height, or roughness
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
            position: [-0.5, -0.5],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [0.5, 0.5],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [0.5, 0.5],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5],
            tex_coords: [0.0, 0.0],
        },
    ];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let uniforms = &uniform! {
        image: image_uniform
    };

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
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
