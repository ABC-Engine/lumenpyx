use crate::Light;
use crate::Vertex;
use crate::WINDOW_VIRTUAL_SIZE;
use glium;
use glium::framebuffer::SimpleFrameBuffer;
use glium::glutin::surface::WindowSurface;
use glium::uniform;
use glium::Surface;

// include the vertex and fragment shaders in the library
const LIGHTING_VERTEX_SHADER_SRC: &str = include_str!("../shaders/shading/lighting.vert");
const LIGHTING_FRAGMENT_SHADER_SRC: &str = include_str!("../shaders/shading/lighting.frag");

const REFLECTION_VERTEX_SHADER_SRC: &str = include_str!("../shaders/shading/reflections.vert");
const REFLECTION_FRAGMENT_SHADER_SRC: &str = include_str!("../shaders/shading/reflections.frag");

const UPSCALE_VERTEX_SHADER_SRC: &str = include_str!("../shaders/shading/upscale_shader.vert");
const UPSCALE_FRAGMENT_SHADER_SRC: &str = include_str!("../shaders/shading/upscale_shader.frag");

const GENERATE_NORMALS_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/shading/normal_generator.vert");
const GENERATE_NORMALS_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/shading/normal_generator.frag");

const LOCAL_MAX_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/technical_shaders/local_max.vert");
const LOCAL_MAX_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/technical_shaders/local_max.frag");

/// upscale the result to the screen size
pub(crate) fn draw_upscale(
    display: &glium::Display<WindowSurface>,
    image_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    indices: &glium::index::NoIndices,
) {
    let program = glium::Program::from_source(
        display,
        UPSCALE_VERTEX_SHADER_SRC,
        UPSCALE_FRAGMENT_SHADER_SRC,
        None,
    )
    .unwrap();

    let mut target = display.draw();
    let dimensions = target.get_dimensions();
    // figure out which dimensions need the black bars
    let [target_width, target_height] = [dimensions.0 as f32, dimensions.1 as f32];
    let [image_width, image_height] = [WINDOW_VIRTUAL_SIZE.0 as f32, WINDOW_VIRTUAL_SIZE.1 as f32];

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
            &program,
            uniforms,
            &Default::default(),
        )
        .unwrap();

    target.finish().unwrap();
}

/// draw the lighting
pub(crate) fn draw_lighting(
    display: &glium::Display<WindowSurface>,
    albedo_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    full_res_heightmap: glium::uniforms::Sampler<glium::texture::Texture2d>,
    medium_res_heightmap: glium::uniforms::Sampler<glium::texture::Texture2d>,
    low_res_heightmap: glium::uniforms::Sampler<glium::texture::Texture2d>,
    light: &Light,
    indices: &glium::index::NoIndices,
    framebuffer: &mut SimpleFrameBuffer,
) {
    let program = glium::Program::from_source(
        display,
        LIGHTING_VERTEX_SHADER_SRC,
        LIGHTING_FRAGMENT_SHADER_SRC,
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
        full_res_heightmap: full_res_heightmap,
        medium_res_heightmap: medium_res_heightmap,
        low_res_heightmap: low_res_heightmap,
        albedomap: albedo_uniform,
        light_pos: light.position,
        light_color: light.color,
        light_intensity: light.intensity,
        light_falloff: light.falloff,
    };

    framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &program,
            uniforms,
            &glium::DrawParameters {
                blend: glium::Blend {
                    color: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::One,
                        destination: glium::LinearBlendingFactor::One,
                    },
                    alpha: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::One,
                        destination: glium::LinearBlendingFactor::One,
                    },
                    constant_value: (0.0, 0.0, 0.0, 0.0),
                },
                ..Default::default()
            },
        )
        .unwrap();
}

pub(crate) fn draw_reflections(
    display: &glium::Display<WindowSurface>,
    lit_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    height_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    rougness_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    normal_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    indices: &glium::index::NoIndices,
    framebuffer: &mut SimpleFrameBuffer,
) {
    let program = glium::Program::from_source(
        display,
        REFLECTION_VERTEX_SHADER_SRC,
        REFLECTION_FRAGMENT_SHADER_SRC,
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

    // TODO: fix placeholder camera position
    let camera_pos: [f32; 3] = [0.5, 0.5, 0.5];

    let uniforms = &uniform! {
        albedomap: lit_uniform,
        heightmap: height_uniform,
        roughnessmap: rougness_uniform,
        normalmap: normal_uniform,
        camera_pos: camera_pos,
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

pub(crate) fn draw_generate_normals(
    display: &glium::Display<WindowSurface>,
    height_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    albedo_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    indices: &glium::index::NoIndices,
    framebuffer: &mut SimpleFrameBuffer,
) {
    let program = glium::Program::from_source(
        display,
        GENERATE_NORMALS_VERTEX_SHADER_SRC,
        GENERATE_NORMALS_FRAGMENT_SHADER_SRC,
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
        heightmap: height_uniform,
        albedomap: albedo_uniform,
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

pub(crate) fn draw_local_max(
    display: &glium::Display<WindowSurface>,
    sampler: glium::uniforms::Sampler<glium::texture::Texture2d>,
    indices: &glium::index::NoIndices,
    framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
) {
    let program = glium::Program::from_source(
        display,
        LOCAL_MAX_VERTEX_SHADER_SRC,
        LOCAL_MAX_FRAGMENT_SHADER_SRC,
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
        high_res_image: sampler,
        new_resolution: framebuffer.get_dimensions(),
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
