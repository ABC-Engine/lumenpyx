use crate::Drawable;
use crate::LumenpyxProgram;
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

use crate::shaders::FULL_SCREEN_QUAD;
use crate::Transform;

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

pub struct Cylinder {
    color: [f32; 4],
    radius: f32,
    height: f32,
    transform: Transform,
}

impl Cylinder {
    pub fn new(color: [f32; 4], radius: f32, height: f32, transform: Transform) -> Cylinder {
        Cylinder {
            color,
            radius,
            height,
            transform,
        }
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
