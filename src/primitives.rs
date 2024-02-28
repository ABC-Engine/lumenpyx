use crate::Drawable;
use glium;
use glium::framebuffer::SimpleFrameBuffer;
use glium::glutin::surface::WindowSurface;
use glium::uniform;
use glium::Surface;

const GENERATE_CIRCLE_VERTEX_SHADER_SRC: &str = include_str!("../shaders/circle_generator.vert");
const GENERATE_CIRCLE_FRAGMENT_SHADER_SRC: &str = include_str!("../shaders/circle_generator.frag");

use crate::Transform;
use crate::Vertex;

pub struct Circle {
    color: [f32; 4],
    radius: f32,
    transform: Transform,
}

impl Circle {
    pub fn new(color: [f32; 4], radius: f32, transform: Transform) -> Circle {
        Circle {
            color,
            radius,
            transform,
        }
    }
}

impl Drawable for Circle {
    fn draw(
        &self,
        display: &glium::Display<WindowSurface>,
        indices: &glium::index::NoIndices,
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        draw_circle(
            self.color,
            self.radius,
            self.transform,
            display,
            indices,
            albedo_framebuffer,
        );
    }
}

pub fn draw_circle(
    color: [f32; 4],
    radius: f32,
    transform: Transform,
    display: &glium::Display<WindowSurface>,
    indices: &glium::index::NoIndices,
    framebuffer: &mut SimpleFrameBuffer,
) {
    let program = glium::Program::from_source(
        display,
        GENERATE_CIRCLE_VERTEX_SHADER_SRC,
        GENERATE_CIRCLE_FRAGMENT_SHADER_SRC,
        None,
    )
    .unwrap();

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
        color: color,
        radius: radius,
        matrix: transform.matrix,
    };

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
