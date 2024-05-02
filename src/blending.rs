use glium::Surface;

use crate::primitives::draw_texture;
use crate::Drawable;
use crate::LumenpyxProgram;
use crate::Transform;
pub use glium::Blend;
pub use glium::BlendingFunction;
pub use glium::LinearBlendingFactor;

pub struct BlendObject<T, U> {
    object_1: T,
    object_2: U,
    transform: Transform,
    // override the shadow strength of the object
    shadow_strength: f32,
    blend: glium::Blend,
}

impl<T, U> BlendObject<T, U> {
    pub fn new(object_1: T, object_2: U, blend: Blend) -> Self {
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

    pub fn set_blend(&mut self, blend: Blend) {
        self.blend = blend;
    }
}

impl<T, U> Drawable for BlendObject<T, U>
where
    T: Drawable,
    U: Drawable,
{
    fn draw(
        &self,
        program: &LumenpyxProgram,
        transform_matrix: [[f32; 4]; 4],
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        blend_mode: Option<glium::Blend>,
    ) {
        let display = &program.display;
        let render_resolution = albedo_framebuffer.get_dimensions();

        let mut new_textures = Vec::new();
        let mut new_framebuffers = Vec::new();

        // create textures for albdeo, height, roughness, and normal
        // not sure how inefficient this is, but probably not great
        for _ in 0..4 {
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

        for i in 0..4 {
            new_framebuffers.push(
                glium::framebuffer::SimpleFrameBuffer::new(display, &new_textures[i])
                    .expect("Failed to create blending framebuffer"),
            );
        }

        {
            // needs to be done to sneak around the borrow checker
            let mut mutable_refs = new_framebuffers.iter_mut().collect::<Vec<_>>();
            let mut mutable_iter = mutable_refs.iter_mut();

            // keep in mind the blend used here is meant to blend the two objects together
            self.object_1.draw(
                program,
                transform_matrix,
                mutable_iter.next().unwrap(),
                mutable_iter.next().unwrap(),
                mutable_iter.next().unwrap(),
                mutable_iter.next().unwrap(),
                Some(self.blend),
            );

            let mut mutable_iter = mutable_refs.iter_mut();
            self.object_2.draw(
                program,
                transform_matrix,
                mutable_iter.next().unwrap(),
                mutable_iter.next().unwrap(),
                mutable_iter.next().unwrap(),
                mutable_iter.next().unwrap(),
                Some(self.blend),
            );
        }

        // overlay our texture to the main framebuffers
        // the blending mode here is meant to blend the new textures with the main framebuffers aka the one passed in
        draw_texture(
            &new_textures[0],
            Transform::default().get_matrix(),
            program,
            albedo_framebuffer,
            blend_mode,
        );

        draw_texture(
            &new_textures[1],
            Transform::default().get_matrix(),
            program,
            height_framebuffer,
            blend_mode,
        );

        draw_texture(
            &new_textures[2],
            Transform::default().get_matrix(),
            program,
            roughness_framebuffer,
            blend_mode,
        );

        draw_texture(
            &new_textures[3],
            Transform::default().get_matrix(),
            program,
            normal_framebuffer,
            blend_mode,
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
    }
}
