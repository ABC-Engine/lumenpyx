use glium::DrawParameters;
use glium::Surface;

//use crate::primitives::draw_texture;
use crate::Drawable;
use crate::LumenpyxProgram;
use crate::Transform;
use crate::DEFAULT_BLEND;
use crate::FULL_SCREEN_QUAD;
use glium::uniform;

const MIX_SHADER_FRAG: &str = include_str!("../shaders/technical_shaders/mix.frag");
const MIX_SHADER_VERT: &str = include_str!("../shaders/technical_shaders/mix.vert");

#[derive(Clone, Copy, Debug)]
pub enum BlendMode {
    Additive,
    Subtractive,
    Multiplicative,
    Divisive,
}

pub struct BlendObject<'a, T, U>
where
    T: Drawable + ?Sized,
    U: Drawable + ?Sized,
{
    object_1: &'a T,
    object_2: &'a U,
    transform: Transform,
    // override the shadow strength of the object
    shadow_strength: f32,
    blend: BlendMode,
}

impl<'a, T, U> BlendObject<'a, T, U>
where
    T: Drawable + ?Sized,
    U: Drawable + ?Sized,
{
    pub fn new(object_1: &'a T, object_2: &'a U, blend: BlendMode) -> Self {
        Self {
            object_1,
            object_2,
            transform: Transform::new([0.0, 0.0, 0.0]),
            shadow_strength: 0.5,
            blend,
        }
    }

    pub fn set_shadow_strength(&mut self, shadow_strength: f32) {
        self.shadow_strength = shadow_strength;
    }

    pub fn set_blend(&mut self, blend: BlendMode) {
        self.blend = blend;
    }
}

impl<'a, T, U> Drawable for BlendObject<'a, T, U>
where
    T: Drawable + ?Sized,
    U: Drawable + ?Sized,
{
    fn draw(
        &self,
        program: &LumenpyxProgram,
        transform_matrix: [[f32; 4]; 4],
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let display = &program.display;
        let render_resolution = albedo_framebuffer.get_dimensions();

        let mut new_textures = Vec::new();
        let mut new_framebuffers = Vec::new();

        // create textures for albdeo, height, roughness, and normal
        // not sure how inefficient this is, but probably not great
        for _ in 0..8 {
            new_textures.push(
                glium::texture::Texture2d::empty_with_format(
                    display,
                    glium::texture::UncompressedFloatFormat::U8U8U8U8,
                    glium::texture::MipmapsOption::NoMipmap,
                    render_resolution.0,
                    render_resolution.1,
                )
                .expect("Failed to create blending texture"),
            );
        }

        for i in 0..8 {
            new_framebuffers.push(
                glium::framebuffer::SimpleFrameBuffer::new(display, &new_textures[i])
                    .expect("Failed to create blending framebuffer"),
            );
        }

        {
            // needs to be done to sneak around the borrow checker
            let mut mutable_refs = new_framebuffers.iter_mut().collect::<Vec<_>>();
            let mut mutable_iter = mutable_refs.iter_mut();

            self.object_1.draw(
                program,
                transform_matrix,
                mutable_iter.next().unwrap(),
                mutable_iter.next().unwrap(),
                mutable_iter.next().unwrap(),
                mutable_iter.next().unwrap(),
            );

            self.object_2.draw(
                program,
                transform_matrix,
                mutable_iter.next().unwrap(),
                mutable_iter.next().unwrap(),
                mutable_iter.next().unwrap(),
                mutable_iter.next().unwrap(),
            );
        }

        // overlay our texture to the main framebuffers
        // the blending mode here is meant to blend the new textures with the main framebuffers aka the one passed in
        // combine the textures
        draw_mix(
            &new_textures[0],
            &new_textures[4],
            &self.blend,
            program,
            albedo_framebuffer,
        );

        draw_mix(
            &new_textures[1],
            &new_textures[5],
            &self.blend,
            program,
            height_framebuffer,
        );

        draw_mix(
            &new_textures[2],
            &new_textures[6],
            &self.blend,
            program,
            roughness_framebuffer,
        );

        draw_mix(
            &new_textures[3],
            &new_textures[7],
            &self.blend,
            program,
            normal_framebuffer,
        );
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.get_matrix()
    }

    fn get_recieve_shadows_strength(&self) -> f32 {
        self.shadow_strength
    }

    fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
        self.object_1.try_load_shaders(program);
        self.object_2.try_load_shaders(program);

        if program.get_shader("mix").is_none() {
            let shader = glium::Program::from_source(
                &program.display,
                MIX_SHADER_VERT,
                MIX_SHADER_FRAG,
                None,
            )
            .expect("Failed to create mix shader");

            program.add_shader(shader, "mix");
        }
    }
}

fn draw_mix(
    bottom: &glium::texture::Texture2d,
    top: &glium::texture::Texture2d,
    blend: &BlendMode,
    program: &LumenpyxProgram,
    framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
) {
    let display = &program.display;
    let indices = &program.indices;

    let shader = program.get_shader("mix").unwrap();

    let uniforms = uniform! {
        bottom_image: bottom,
        top_image: top,
        add: match blend {
            BlendMode::Additive => true,
            _ => false,
        },
        subtract: match blend {
            BlendMode::Subtractive => true,
            _ => false,
        },
        multiply: match blend {
            BlendMode::Multiplicative => true,
            _ => false,
        },
        divide: match blend {
            BlendMode::Divisive => true,
            _ => false,
        },
    };

    let shape = FULL_SCREEN_QUAD;

    let vertex_buffer =
        glium::VertexBuffer::new(display, &shape).expect("Failed to create vertex buffer");

    framebuffer
        .draw(
            &vertex_buffer,
            indices,
            &shader,
            &uniforms,
            &DrawParameters {
                blend: DEFAULT_BLEND,
                ..Default::default()
            },
        )
        .expect("Failed to draw mix");
}
