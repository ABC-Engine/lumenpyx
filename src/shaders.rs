use crate::primitives::{BASE_FRAGMENT_SHADER_SRC, BASE_VERTEX_SHADER_SRC};
use crate::Camera;
use crate::LumenpyxProgram;
use crate::Vertex;
use glium::framebuffer::SimpleFrameBuffer;
use glium::Surface;
use glium::{self, BlitTarget};
use glium::{uniform, DrawParameters};

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

pub(crate) const FILL_ALPHA_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/technical_shaders/fill_alpha.vert");

pub(crate) const FILL_ALPHA_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/technical_shaders/fill_alpha.frag");

pub(crate) const CROP_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/technical_shaders/crop_shader.frag");

pub(crate) const CROP_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/technical_shaders/crop_shader.vert");

pub(crate) const OVERLAY_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/technical_shaders/overlay.frag");

pub(crate) const OVERLAY_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/technical_shaders/overlay.vert");

pub(crate) const GAUSSIAN_BLUR_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/technical_shaders/blur_area.frag");

pub(crate) const GAUSSIAN_BLUR_VERTEX_SHADER_SRC: &str =
    include_str!("../shaders/technical_shaders/blur_area.vert");

pub(crate) const DEFAULT_BLEND: glium::Blend = glium::Blend {
    color: glium::BlendingFunction::Addition {
        source: glium::LinearBlendingFactor::SourceAlpha,
        destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
    },
    alpha: glium::BlendingFunction::Addition {
        source: glium::LinearBlendingFactor::SourceAlpha,
        destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
    },
    constant_value: (0.0, 0.0, 0.0, 0.0),
};

/// A full screen quad that can be used to draw to the screen with a shader
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

pub(crate) fn draw_crop_centered(
    image_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    framebuffer: &mut SimpleFrameBuffer,
    lumenpyx_program: &LumenpyxProgram,
    cropped_res: [u32; 2],
) {
    let display = &lumenpyx_program.display;
    let indices = &lumenpyx_program.indices;

    let crop_shader = &lumenpyx_program
        .get_shader("crop_shader")
        .expect("Failed to load crop shader");

    let image_dimensions = [
        image_uniform.0.dimensions().0 as f32,
        image_uniform.0.dimensions().1 as f32,
    ];

    let [target_width, target_height] = [cropped_res[0] as f32, cropped_res[1] as f32];

    let uniform = uniform! {
        image: image_uniform,
        image_res: image_dimensions,
        screen_res: [target_width, target_height],
    };

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer =
        glium::VertexBuffer::new(display, &shape).expect("Failed to create vertex buffer");

    framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &crop_shader,
            &uniform,
            &Default::default(),
        )
        .expect("Failed to draw to the framebuffer");
}

/// upscale the result to the screen size
pub(crate) fn draw_upscale(
    image_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    lumenpyx_program: &LumenpyxProgram,
    window_resolution: [u32; 2],
) {
    let display = &lumenpyx_program.display;
    let indices = &lumenpyx_program.indices;

    let texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        window_resolution[0],
        window_resolution[1],
    )
    .expect("Failed to create texture for height texture");

    let new_uniform = match [
        image_uniform.0.dimensions().0,
        image_uniform.0.dimensions().1,
    ] == window_resolution
    {
        true => image_uniform,
        false => {
            let mut framebuffer = glium::framebuffer::SimpleFrameBuffer::new(display, &texture)
                .expect("Failed to create framebuffer for height texture");

            draw_crop_centered(
                image_uniform,
                &mut framebuffer,
                lumenpyx_program,
                window_resolution,
            );

            glium::uniforms::Sampler::new(&texture)
                .anisotropy(1)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
        }
    };

    let upscale_shader = &lumenpyx_program
        .get_shader("upscale_shader")
        .expect("Failed to load upscale shader");

    let mut target = display.draw();
    let dimensions = target.get_dimensions();

    // figure out which dimensions need the black bars
    let [target_width, target_height] = [dimensions.0 as f32, dimensions.1 as f32];
    let [image_width, image_height] = [window_resolution[0] as f32, window_resolution[1] as f32];

    let mut dim_scales = [image_width / target_width, image_height / target_height];

    // make the max value 1.0
    if dim_scales[0] > dim_scales[1] {
        dim_scales[1] *= 1.0 / dim_scales[0];
        dim_scales[0] = 1.0;
    } else {
        dim_scales[0] *= 1.0 / dim_scales[1];
        dim_scales[1] = 1.0;
    }

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

    let vertex_buffer =
        glium::VertexBuffer::new(display, &shape).expect("Failed to create vertex buffer");

    let uniforms = &uniform! {
        image: new_uniform,
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
        .expect("Failed to draw to the screen");

    target
        .finish()
        .expect("Failed to finish drawing to the screen");
}

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

    let reflection_texture = program
        .cached_textures
        .get("reflection_texture")
        .expect("Failed to get reflection texture");

    let mut reflection_framebuffer =
        glium::framebuffer::SimpleFrameBuffer::new(display, reflection_texture)
            .expect("Failed to create framebuffer for reflection texture");

    reflection_framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);

    let shader = &program
        .get_shader("reflection_shader")
        .expect("Failed to load reflection shader");

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer =
        glium::VertexBuffer::new(display, &shape).expect("Failed to create vertex buffer");

    let camera_pos = camera.position;

    let uniforms = &uniform! {
        albedomap: lit_uniform,
        heightmap: height_uniform,
        roughnessmap: rougness_uniform,
        normalmap: normal_uniform,
        camera_z: camera_pos[2],
        blur_reflections: program.render_settings.blur_reflections,
    };

    // the intersection distance is stored in the alpha channel of the texture,
    // since there isn't really a point of the alpha channel in this context
    reflection_framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &shader,
            uniforms,
            &DrawParameters {
                blend: glium::Blend {
                    color: glium::BlendingFunction::AlwaysReplace,
                    alpha: glium::BlendingFunction::AlwaysReplace,
                    constant_value: (0.0, 0.0, 0.0, 0.0),
                },
                ..Default::default()
            },
        )
        .expect("Failed to draw to the framebuffer");

    let reflection_sampler = glium::uniforms::Sampler::new(reflection_texture)
        .anisotropy(1)
        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest);

    if program.render_settings.blur_reflections {
        draw_gaussian_blur(
            &mut reflection_framebuffer,
            program,
            reflection_sampler,
            program.render_settings.blur_strength,
        );
    }
    println!("draw_reflections");
    draw_overlay(framebuffer, program, reflection_sampler, lit_uniform);
}

pub(crate) fn draw_gaussian_blur(
    framebuffer: &mut SimpleFrameBuffer,
    program: &LumenpyxProgram,
    blur_sampler: glium::uniforms::Sampler<glium::texture::Texture2d>,
    blur_size: f32,
) {
    let display = &program.display;
    let indices = &program.indices;
    let shader = &program
        .get_shader("gaussian_blur")
        .expect("Failed to load gaussian blur shader");

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer =
        glium::VertexBuffer::new(display, &shape).expect("Failed to create vertex buffer");

    let uniforms = &uniform! {
        tex: blur_sampler,
        blur_scale: blur_size,
    };

    framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &shader,
            uniforms,
            &DrawParameters {
                blend: glium::Blend {
                    color: glium::BlendingFunction::AlwaysReplace,
                    alpha: glium::BlendingFunction::AlwaysReplace,
                    constant_value: (0.0, 0.0, 0.0, 0.0),
                },
                ..Default::default()
            },
        )
        .expect("Failed to draw to the framebuffer");
}

pub(crate) fn draw_generate_normals(
    program: &LumenpyxProgram,
    height_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    albedo_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    framebuffer: &mut SimpleFrameBuffer,
) {
    let display = &program.display;
    let indices = &program.indices;

    let shader = program
        .get_shader("generate_normals_shader")
        .expect("Failed to load generate normals shader");

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer =
        glium::VertexBuffer::new(display, &shape).expect("Failed to create vertex buffer");

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
        .expect("Failed to draw to the framebuffer");
}

pub(crate) fn draw_fill_alpha(
    framebuffer: &mut SimpleFrameBuffer,
    program: &LumenpyxProgram,
    target_sampler: glium::uniforms::Sampler<glium::texture::Texture2d>,
    fill_color: [f32; 4],
) {
    let display = &program.display;
    let indices = &program.indices;
    let shader = &program
        .get_shader("fill_alpha")
        .expect("Failed to load fill alpha shader");

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer =
        glium::VertexBuffer::new(display, &shape).expect("Failed to create vertex buffer");

    let uniforms = &uniform! {
        target_fill: target_sampler,
        color_fill: fill_color,
    };

    framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &shader,
            uniforms,
            &Default::default(),
        )
        .expect("Failed to draw to the framebuffer");
}

pub(crate) fn new_fill_alpha_texure(
    program: &LumenpyxProgram,
    albedo_sampler: glium::uniforms::Sampler<glium::texture::Texture2d>,
    color: [f32; 4],
) -> glium::texture::Texture2d {
    let display = &program.display;
    let dimensions = albedo_sampler.0.dimensions();

    let texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        dimensions.0,
        dimensions.1,
    )
    .expect("Failed to create texture for height texture");

    let mut framebuffer = glium::framebuffer::SimpleFrameBuffer::new(display, &texture)
        .expect("Failed to create framebuffer for height texture");
    draw_fill_alpha(&mut framebuffer, program, albedo_sampler, color);

    texture
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
    let shader = &program
        .get_shader("faster_clear_color_shader")
        .expect("Failed to load shader");

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer =
        glium::VertexBuffer::new(display, &shape).expect("Failed to create vertex buffer");

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
        .expect("Failed to draw to the framebuffer");
}

pub(crate) fn draw_overlay(
    framebuffer: &mut SimpleFrameBuffer,
    program: &LumenpyxProgram,
    top_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
    bottom_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
) {
    let display = &program.display;
    let indices = &program.indices;
    let shader = &program
        .get_shader("overlay_shader")
        .expect("Failed to load overlay shader");

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer =
        glium::VertexBuffer::new(display, &shape).expect("Failed to create vertex buffer");

    let uniforms = &uniform! {
        top_tex: top_uniform,
        bottom_tex: bottom_uniform,
    };

    framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &shader,
            uniforms,
            &Default::default(),
        )
        .expect("Failed to draw to the framebuffer");
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
    let shader = &program
        .get_shader("recieve_shadows_shader")
        .expect("Failed to load shader");

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer =
        glium::VertexBuffer::new(display, &shape).expect("Failed to create vertex buffer");

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
        .expect("Failed to draw to the framebuffer");
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

    {
        let display = &program.display;
        let upscale_shader = glium::Program::from_source(
            display,
            UPSCALE_VERTEX_SHADER_SRC,
            UPSCALE_FRAGMENT_SHADER_SRC,
            None,
        )
        .expect("Failed to load upscale shader");

        program.add_shader(upscale_shader, "upscale_shader");
    }

    {
        let display = &program.display;
        let reflection_shader = glium::Program::from_source(
            display,
            REFLECTION_VERTEX_SHADER_SRC,
            REFLECTION_FRAGMENT_SHADER_SRC,
            None,
        )
        .expect("Failed to load reflection shader");

        program.add_shader(reflection_shader, "reflection_shader");
    }

    {
        let display = &program.display;
        let shader = glium::Program::from_source(
            display,
            FILL_ALPHA_VERTEX_SHADER_SRC,
            FILL_ALPHA_FRAGMENT_SHADER_SRC,
            None,
        )
        .expect("Failed to load fill alpha shader");
        program.add_shader(shader, "fill_alpha");
    }

    {
        let display = &program.display;
        let shader = glium::Program::from_source(
            display,
            CROP_VERTEX_SHADER_SRC,
            CROP_FRAGMENT_SHADER_SRC,
            None,
        )
        .expect("Failed to load crop shader");
        program.add_shader(shader, "crop_shader");
    }

    {
        let display = &program.display;
        let shader = glium::Program::from_source(
            display,
            OVERLAY_VERTEX_SHADER_SRC,
            OVERLAY_FRAGMENT_SHADER_SRC,
            None,
        )
        .expect("Failed to load overlay shader");
        program.add_shader(shader, "overlay_shader");
    }

    {
        let display = &program.display;
        let shader = glium::Program::from_source(
            display,
            GAUSSIAN_BLUR_VERTEX_SHADER_SRC,
            GAUSSIAN_BLUR_FRAGMENT_SHADER_SRC,
            None,
        )
        .expect("Failed to load gaussian blur shader");
        program.add_shader(shader, "gaussian_blur");
    }
}
