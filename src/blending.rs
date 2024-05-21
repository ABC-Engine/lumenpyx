use std::any::TypeId;

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
        transform: &Transform,
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let display = &program.display;

        let mut new_textures = Vec::new();
        let mut new_framebuffers = Vec::new();

        // create textures for albdeo, height, roughness, and normal
        // not sure how inefficient this is, but probably not great
        for i in 0..8 {
            new_textures.push(
                program
                    .get_texture(&format!("blend_texture_{}", i))
                    .expect("Failed to get blending texture"),
            );
        }

        for i in 0..8 {
            let mut new_framebuffer =
                glium::framebuffer::SimpleFrameBuffer::new(display, new_textures[i])
                    .expect("Failed to create blending framebuffer");

            new_framebuffer.clear_color(0.0, 0.0, 0.0, 0.0);

            new_framebuffers.push(new_framebuffer);
        }

        {
            // needs to be done to sneak around the borrow checker
            let mut mutable_refs = new_framebuffers.iter_mut().collect::<Vec<_>>();
            let mut mutable_iter = mutable_refs.iter_mut();

            {
                // all of this is a little bit of a hack to correctly position the object
                let non_ajusted_object_transform = self.object_1.get_transform();
                let camera = crate::Camera::new([0.0, 0.0, 0.0]);
                let mut adjusted_transform_matrix = program
                    .adjust_transform_for_drawable(
                        &non_ajusted_object_transform.add_parent(&self.transform),
                        &camera,
                    )
                    .get_matrix();
                // adjust based off camera, the camera offset is the plugged in camera position, because we don't give it the actual position of the camera
                // add instead of subtract because the transform matrix is -camera_position
                let transform_matrix = transform.get_matrix();
                adjusted_transform_matrix[3][0] += transform_matrix[3][0];
                adjusted_transform_matrix[3][1] += transform_matrix[3][1];

                self.object_1.draw(
                    program,
                    &Transform::from_matrix(adjusted_transform_matrix),
                    mutable_iter.next().unwrap(),
                    mutable_iter.next().unwrap(),
                    mutable_iter.next().unwrap(),
                    mutable_iter.next().unwrap(),
                );
            }

            {
                // all of this is a little bit of a hack to correctly position the object
                let non_ajusted_object_transform = self.object_2.get_transform();
                let camera = crate::Camera::new([0.0, 0.0, 0.0]);
                let mut adjusted_transform_matrix = program
                    .adjust_transform_for_drawable(
                        &non_ajusted_object_transform.add_parent(&self.transform),
                        &camera,
                    )
                    .get_matrix();
                // adjust based off camera, the camera offset is the plugged in camera position, because we don't give it the actual position of the camera
                // add instead of subtract because the transform matrix is -camera_position
                let transform_matrix = transform.get_matrix();
                adjusted_transform_matrix[3][0] += transform_matrix[3][0];
                adjusted_transform_matrix[3][1] += transform_matrix[3][1];

                self.object_2.draw(
                    program,
                    &Transform::from_matrix(adjusted_transform_matrix),
                    mutable_iter.next().unwrap(),
                    mutable_iter.next().unwrap(),
                    mutable_iter.next().unwrap(),
                    mutable_iter.next().unwrap(),
                );
            }
        }

        // overlay our texture to the main framebuffers
        // the blending mode here is meant to blend the new textures with the main framebuffers aka the one passed in
        // combine the textures
        draw_mix(
            new_textures[0],
            new_textures[4],
            &self.blend,
            program,
            albedo_framebuffer,
        );

        draw_mix(
            new_textures[1],
            new_textures[5],
            &self.blend,
            program,
            height_framebuffer,
        );

        draw_mix(
            new_textures[2],
            new_textures[6],
            &self.blend,
            program,
            roughness_framebuffer,
        );

        draw_mix(
            new_textures[3],
            new_textures[7],
            &self.blend,
            program,
            normal_framebuffer,
        );
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn get_transform(&self) -> Transform {
        // this is dumb, but we need to find the camera position
        Transform::default()
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

        let render_resolution = program.get_render_resolution();

        for i in 0..8 {
            let new_texture = glium::texture::Texture2d::empty_with_format(
                &program.display,
                glium::texture::UncompressedFloatFormat::U8U8U8U8,
                glium::texture::MipmapsOption::NoMipmap,
                render_resolution[0],
                render_resolution[1],
            )
            .expect("Failed to create blending texture");

            // i think this might break if we do a blend object inside a blend object im not sure how to fix that, if you need to do that, file an issue
            program.add_texture(new_texture, &format!("blend_texture_{}", i) as &str);
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
