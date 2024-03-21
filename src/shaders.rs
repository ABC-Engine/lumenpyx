use crate::Camera;
use crate::LumenpyxProgram;
use crate::Vertex;
use crate::BASE_FRAGMENT_SHADER_SRC;
use crate::BASE_VERTEX_SHADER_SRC;
use glium;
use glium::framebuffer::SimpleFrameBuffer;
use glium::glutin::surface::WindowSurface;
use glium::uniform;
use glium::Surface;

// include the vertex and fragment shaders in the library
pub(crate) const REFLECTION_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/shading/reflections.vert");
pub(crate) const REFLECTION_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/shading/reflections.frag");

pub(crate) const UPSCALE_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/shading/upscale_shader.vert");
pub(crate) const UPSCALE_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/shading/upscale_shader.frag");

pub(crate) const GENERATE_NORMALS_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/shading/normal_generator.vert");
pub(crate) const GENERATE_NORMALS_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/shading/normal_generator.frag");

pub(crate) const FASTER_CLEAR_COLOR_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/technical_shaders/clear_color.vert");
pub(crate) const FASTER_CLEAR_COLOR_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/technical_shaders/clear_color.frag");

pub(crate) const RECIEVE_SHADOWS_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/technical_shaders/set_recieve_shadows.vert");

pub(crate) const RECIEVE_SHADOWS_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/technical_shaders/set_recieve_shadows.frag");

pub const FULL_SCREEN_QUAD: [Vertex; 6] = [
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

/// upscale the result to the screen size
pub(crate) fn draw_upscale(
    image_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    lumenpyx_program: &LumenpyxProgram,
) {
    let display = &lumenpyx_program.display;
    let indices = &lumenpyx_program.indices;
    let upscale_shader = &lumenpyx_program.upscale_shader;

    let mut target = display.draw();
    let dimensions = target.get_dimensions();
    // figure out which dimensions need the black bars
    let [target_width, target_height] = [dimensions.0 as f32, dimensions.1 as f32];
    let [image_width, image_height] = [
        lumenpyx_program.dimensions[0] as f32,
        lumenpyx_program.dimensions[1] as f32,
    ];

    let mut dim_scales = [image_width / target_width, image_height / target_height];
    // make the max value 1.0
    if dim_scales[0] > dim_scales[1] {
        dim_scales[1] *= 1.0 / dim_scales[0];
        dim_scales[0] = 1.0;
    } else {
        dim_scales[0] *= 1.0 / dim_scales[1];
        dim_scales[1] = 1.0;
    }

    //let (target_width, target_height) = (target_width * image_width, target_height * image_height);
    //let (target_width, target_height) = (target_width as u32, target_height as u32);
    // change the position of the vertices to fit the screen not the tex_coords

    let shape = vec![
        Vertex {
            position: [-1.0 * dim_scales[0], -1.0 * dim_scales[1]],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [1.0 * dim_scales[0], -1.0 * dim_scales[1]],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [1.0 * dim_scales[0], 1.0 * dim_scales[1]],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [1.0 * dim_scales[0], 1.0 * dim_scales[1]],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [-1.0 * dim_scales[0], 1.0 * dim_scales[1]],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [-1.0 * dim_scales[0], -1.0 * dim_scales[1]],
            tex_coords: [0.0, 0.0],
        },
    ];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let uniforms = &uniform! {
        image: image_uniform
    };

    target.clear_color(0.0, 0.0, 0.0, 0.0);
    target
        .draw(
            &vertex_buffer,
            indices,
            &upscale_shader,
            uniforms,
            &Default::default(),
        )
        .unwrap();

    target.finish().unwrap();
}

#[no_mangle]
pub(crate) fn draw_reflections(
    camera: &Camera,
    lit_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    height_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    rougness_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    normal_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    framebuffer: &mut SimpleFrameBuffer,
    program: &LumenpyxProgram,
) {
    let display = &program.display;
    let indices = &program.indices;
    let shader = &program.reflection_shader;

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let camera_pos = camera.position;

    let uniforms = &uniform! {
        albedomap: lit_uniform,
        heightmap: height_uniform,
        roughnessmap: rougness_uniform,
        normalmap: normal_uniform,
        camera_z: camera_pos[2],
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

pub(crate) fn draw_generate_normals(
    program: &LumenpyxProgram,
    height_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    albedo_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    framebuffer: &mut SimpleFrameBuffer,
) {
    let display = &program.display;
    let indices = &program.indices;

    let shader = program.get_shader("generate_normals_shader").unwrap();

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let uniforms = &uniform! {
        heightmap: height_uniform,
        albedomap: albedo_uniform,
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

// Profiling seems to indicate that the glium clear color is the slowest part of the rendering
// process. So this this is a simpler and faster version of the clear color function
pub(crate) fn faster_clear_color(
    framebuffer: &mut SimpleFrameBuffer,
    color: [f32; 4],
    program: &LumenpyxProgram,
) {
    let display = &program.display;
    let indices = &program.indices;
    let shader = &program.get_shader("faster_clear_color_shader").unwrap();

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let uniforms = &uniform! {
        new_color: color,
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

pub(crate) fn draw_recieve_shadows(
    framebuffer: &mut SimpleFrameBuffer,
    program: &LumenpyxProgram,
    recieve_shadows_strength: f32,
    last_frame_sampler: glium::uniforms::Sampler<glium::texture::Texture2d>,
    this_frame_sampler: glium::uniforms::Sampler<glium::texture::Texture2d>,
) {
    let display = &program.display;
    let indices = &program.indices;
    let shader = &program.get_shader("recieve_shadows_shader").unwrap();

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let uniforms = &uniform! {
        last_draw: last_frame_sampler,
        this_draw: this_frame_sampler,
        shadow_strength: recieve_shadows_strength,
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

pub(crate) fn load_all_system_shaders(program: &mut LumenpyxProgram) {
    {
        let display = &program.display;
        let set_recieve_shadows_shader = glium::Program::from_source(
            display,
            RECIEVE_SHADOWS_VERTEX_SHADER_SRC,
            RECIEVE_SHADOWS_FRAGMENT_SHADER_SRC,
            None,
        )
        .expect("Failed to load recieve shadows shader");

        program.add_shader(set_recieve_shadows_shader, "recieve_shadows_shader");
    }

    {
        let display = &program.display;
        let sprite_shader = glium::Program::from_source(
            display,
            BASE_VERTEX_SHADER_SRC,
            BASE_FRAGMENT_SHADER_SRC,
            None,
        )
        .expect("Failed to load sprite shader");

        program.add_shader(sprite_shader, "sprite_shader");
    }

    {
        let display = &program.display;
        let generate_normals_shader = glium::Program::from_source(
            display,
            GENERATE_NORMALS_VERTEX_SHADER_SRC,
            GENERATE_NORMALS_FRAGMENT_SHADER_SRC,
            None,
        )
        .expect("Failed to load generate normals shader");

        program.add_shader(generate_normals_shader, "generate_normals_shader");
    }
}
