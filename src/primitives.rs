use crate::Drawable;
use crate::LumenpyxProgram;
use glium;
use glium::framebuffer::SimpleFrameBuffer;
use glium::uniform;
use glium::Surface;

const GENERATE_CIRCLE_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/ahr_shaders/circle_ahr_shader.vert");
const GENERATE_CIRCLE_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/ahr_shaders/circle_ahr_shader.frag");

const GENERATE_SPHERE_HEIGHT_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/ahr_shaders/sphere_height_shader.vert");
const GENERATE_SPHERE_HEIGHT_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/ahr_shaders/sphere_height_shader.frag");

const GENERATE_SPHERE_NORMAL_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/ahr_shaders/sphere_normal_shader.vert");
const GENERATE_SPHERE_NORMAL_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/ahr_shaders/sphere_normal_shader.frag");

const GENERATE_RECTANGLE_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/ahr_shaders/rectangle_ahr_shader.vert");
const GENERATE_RECTANGLE_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/ahr_shaders/rectangle_ahr_shader.frag");

use crate::Transform;
use crate::Vertex;

pub fn draw_circle(
    color: [f32; 4],
    radius: f32,
    matrix_transform: [[f32; 4]; 4],
    program: &LumenpyxProgram,
    framebuffer: &mut SimpleFrameBuffer,
) {
    let display = &program.display;
    let indices = &program.indices;

    let shader = program.get_shader("circle_ahr_shader").unwrap();

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

    let height_shader = program.get_shader("sphere_height_shader").unwrap();
    let normal_shader = program.get_shader("sphere_normal_shader").unwrap();

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
        radius_squared: radius.powi(2),
        matrix: matrix_transform,
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
        radius_squared: radius.powi(2),
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

    let shader = program.get_shader("rectangle_ahr_shader").unwrap();

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
}

pub struct Sphere {
    color: [f32; 4],
    radius: f32,
    transform: Transform,
}

impl Sphere {
    pub fn new(color: [f32; 4], radius: f32, transform: Transform) -> Sphere {
        Sphere {
            color,
            radius,
            transform,
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
        if program.get_shader("sphere_height_shader").is_some() {
            return;
        }

        let shader = glium::Program::from_source(
            &program.display,
            GENERATE_SPHERE_HEIGHT_VERTEX_SHADER_SRC,
            GENERATE_SPHERE_HEIGHT_FRAGMENT_SHADER_SRC,
            None,
        )
        .unwrap();

        program.add_shader(shader, "sphere_height_shader");

        let shader = glium::Program::from_source(
            &program.display,
            GENERATE_SPHERE_NORMAL_VERTEX_SHADER_SRC,
            GENERATE_SPHERE_NORMAL_FRAGMENT_SHADER_SRC,
            None,
        )
        .unwrap();

        program.add_shader(shader, "sphere_normal_shader");
    }

    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.matrix
    }
}

pub struct Rectangle {
    color: [f32; 4],
    width: f32,
    height: f32,
    transform: Transform,
}

impl Rectangle {
    pub fn new(color: [f32; 4], width: f32, height: f32, transform: Transform) -> Rectangle {
        Rectangle {
            color,
            width,
            height,
            transform,
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
}
