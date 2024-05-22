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

    fn draw_single(
        &self,
        program: &LumenpyxProgram,
        transform: &Transform,
        framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        texture_1_name: &str,
        texture_2_name: &str,
    ) {
        let display = &program.display;

        let texture_1 = program
            .get_texture(texture_1_name)
            .expect("Failed to get blending texture");
        let texture_2 = program
            .get_texture(texture_2_name)
            .expect("Failed to get blending texture");

        let mut framebuffer_1 = glium::framebuffer::SimpleFrameBuffer::new(display, texture_1)
            .expect("Failed to create blending framebuffer");

        framebuffer_1.clear_color(0.0, 0.0, 0.0, 0.0);

        let mut framebuffer_2 = glium::framebuffer::SimpleFrameBuffer::new(display, texture_2)
            .expect("Failed to create blending framebuffer");

        framebuffer_2.clear_color(0.0, 0.0, 0.0, 0.0);

        {
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

                self.object_1.draw_albedo(
                    program,
                    &Transform::from_matrix(adjusted_transform_matrix),
                    &mut framebuffer_1,
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

                self.object_2.draw_albedo(
                    program,
                    &Transform::from_matrix(adjusted_transform_matrix),
                    &mut framebuffer_2,
                );
            }
        }

        // overlay our texture to the main framebuffers
        // the blending mode here is meant to blend the new textures with the main framebuffers aka the one passed in
        // combine the textures
        draw_mix(texture_1, texture_2, &self.blend, program, framebuffer);
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
    fn draw_albedo(
        &self,
        program: &LumenpyxProgram,
        transform: &Transform,
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        self.draw_single(
            program,
            transform,
            albedo_framebuffer,
            "albedo_texture_0",
            "albedo_texture_1",
        );
    }

    fn draw_height(
        &self,
        program: &LumenpyxProgram,
        transform: &Transform,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        self.draw_single(
            program,
            transform,
            height_framebuffer,
            "height_texture_0",
            "height_texture_1",
        );
    }

    fn draw_roughness(
        &self,
        program: &LumenpyxProgram,
        transform: &Transform,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        self.draw_single(
            program,
            transform,
            roughness_framebuffer,
            "roughness_texture_0",
            "roughness_texture_1",
        );
    }

    fn draw_normal(
        &self,
        program: &LumenpyxProgram,
        transform: &Transform,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        self.draw_single(
            program,
            transform,
            normal_framebuffer,
            "normal_texture_0",
            "normal_texture_1",
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

        let mut new_textures = vec![];
        for _ in 0..8 {
            let new_texture = glium::texture::Texture2d::empty_with_format(
                &program.display,
                glium::texture::UncompressedFloatFormat::U8U8U8U8,
                glium::texture::MipmapsOption::NoMipmap,
                render_resolution[0],
                render_resolution[1],
            )
            .expect("Failed to create blending texture");

            new_textures.push(new_texture);
        }

        // i think this might break if we do a blend object inside a blend object im not sure how to fix that, if you need to do that, file an issue
        program.add_texture(new_textures.pop().unwrap(), "albedo_texture_0");
        program.add_texture(new_textures.pop().unwrap(), "albedo_texture_1");
        program.add_texture(new_textures.pop().unwrap(), "height_texture_0");
        program.add_texture(new_textures.pop().unwrap(), "height_texture_1");
        program.add_texture(new_textures.pop().unwrap(), "roughness_texture_0");
        program.add_texture(new_textures.pop().unwrap(), "roughness_texture_1");
        program.add_texture(new_textures.pop().unwrap(), "normal_texture_0");
        program.add_texture(new_textures.pop().unwrap(), "normal_texture_1");
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
