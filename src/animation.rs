use glium::texture;

use crate::load_image;
use crate::primitives::{Normal, Sprite, Texture};
use crate::Transform;
use crate::{drawable_object::Drawable, LumenpyxProgram};
use glium::Surface;
use std::iter::zip;
use std::time::{Duration, Instant};

pub struct Animation {
    sprites: Vec<Sprite>,
    time_between_frames: Duration,
    start_time: Instant,
    shadow_strength: f32,
    pub transform: Transform,
}

impl Animation {
    /// Takes a path to a series of images in format `path1.png`, `path2.png`, etc.
    pub fn new_from_images(
        albedo: Texture,
        height: Texture,
        roughness: Texture,
        normal: Normal,
        num_frames: usize,
        time_between_frames: Duration,
        transform: Transform,
        program: &LumenpyxProgram,
    ) -> Self {
        let mut albedo_textures = load_tex_from_images_albedo(albedo, num_frames, program);
        let mut height_textures =
            load_tex_from_images_non_albedo(&albedo_textures, height, program);
        let mut roughness_textures =
            load_tex_from_images_non_albedo(&albedo_textures, roughness, program);
        let mut normal_textures =
            load_tex_from_images_normal(&albedo_textures, &height_textures, normal, program);

        if albedo_textures.len() != num_frames
            || height_textures.len() != num_frames
            || roughness_textures.len() != num_frames
            || normal_textures.len() != num_frames
        {
            panic!("The number of frames in the images must be the same");
        }

        let mut sprites = vec![];
        for _ in 0..num_frames {
            let albedo_texture = albedo_textures.remove(0);
            let height_texture = height_textures.remove(0);
            let roughness_texture = roughness_textures.remove(0);
            let normal_texture = normal_textures.remove(0);

            let sprite = Sprite::new(
                albedo_texture.into(),
                height_texture.into(),
                roughness_texture.into(),
                normal_texture.into(),
                &program,
                transform,
            );
            sprites.push(sprite);
        }

        Self {
            sprites,
            time_between_frames,
            start_time: Instant::now(),
            shadow_strength: 0.5,
            transform,
        }
    }

    pub fn new_from_spritesheet(
        albedo: Texture,
        height: Texture,
        roughness: Texture,
        normal: Normal,
        num_frames: usize,
        time_between_frames: Duration,
        transform: Transform,
        program: &LumenpyxProgram,
    ) -> Self {
        let mut albedo_textures = load_albedo_from_spritesheet(albedo, num_frames, program);
        let mut height_textures =
            load_non_albedo_from_spritesheet(&albedo_textures, height, program);
        let mut roughness_textures =
            load_non_albedo_from_spritesheet(&albedo_textures, roughness, program);
        let mut normal_textures =
            load_normal_from_spritesheet(&albedo_textures, &height_textures, normal, program);

        if albedo_textures.len() != num_frames
            || height_textures.len() != num_frames
            || roughness_textures.len() != num_frames
            || normal_textures.len() != num_frames
        {
            panic!("The number of frames in the spritesheets must be the same");
        }

        let mut sprites = vec![];
        for _ in 0..num_frames {
            let albedo_texture = albedo_textures.remove(0);
            let height_texture = height_textures.remove(0);
            let roughness_texture = roughness_textures.remove(0);
            let normal_texture = normal_textures.remove(0);

            let sprite = Sprite::new(
                albedo_texture.into(),
                height_texture.into(),
                roughness_texture.into(),
                normal_texture.into(),
                &program,
                transform,
            );
            sprites.push(sprite);
        }

        Self {
            sprites,
            time_between_frames,
            start_time: Instant::now(),
            shadow_strength: 0.5,
            transform,
        }
    }
}

impl Drawable for Animation {
    fn draw(
        &self,
        program: &LumenpyxProgram,
        transform_matrix: [[f32; 4]; 4],
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let current_frame_num = self
            .start_time
            .elapsed()
            .as_nanos()
            .checked_div(self.time_between_frames.as_nanos())
            .expect("time between frames on an animation cannot be set to 0")
            % self.sprites.len() as u128;

        let current_frame = &self.sprites[current_frame_num as usize];

        current_frame.draw(
            program,
            transform_matrix,
            albedo_framebuffer,
            height_framebuffer,
            roughness_framebuffer,
            normal_framebuffer,
        );
    }

    fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
        for sprite in &self.sprites {
            sprite.try_load_shaders(program);
        }
    }

    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.get_matrix()
    }

    fn get_recieve_shadows_strength(&self) -> f32 {
        self.shadow_strength
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

/// splits a texture into multiple textures, one for each frame
fn load_textures_from_spritesheet_tex(
    texture: glium::Texture2d,
    num_frames: usize,
    program: &LumenpyxProgram,
) -> Vec<glium::Texture2d> {
    let texture_framebuffer =
        glium::framebuffer::SimpleFrameBuffer::new(&program.display, &texture).expect(
            "failed to create texture framebuffer when creating animation from spritesheet",
        );

    // split the image into frames
    let frame_width = texture.width() / num_frames as u32;
    let frame_height = texture.height();

    let mut textures = Vec::new();

    for i in 0..num_frames {
        let new_texture = texture::Texture2d::empty_with_format(
            &program.display,
            texture::UncompressedFloatFormat::U8U8U8U8,
            texture::MipmapsOption::NoMipmap,
            frame_width,
            frame_height,
        )
        .expect("failed to create texture when creating animation from spritesheet");

        let new_texture_framebuffer =
            glium::framebuffer::SimpleFrameBuffer::new(&program.display, &new_texture).expect(
                "failed to create texture framebuffer when creating animation from spritesheet",
            );

        let dest_rect = &glium::Rect {
            left: (i as i32 * frame_width as i32) as u32,
            bottom: 0,
            width: frame_width as u32,
            height: frame_height as u32,
        };

        let target_rect = &glium::BlitTarget {
            left: 0,
            bottom: 0,
            width: frame_width as i32,
            height: frame_height as i32,
        };

        texture_framebuffer.blit_color(
            dest_rect,
            &new_texture_framebuffer,
            target_rect,
            glium::uniforms::MagnifySamplerFilter::Nearest,
        );

        textures.push(new_texture);
    }

    textures
}

fn load_textures_from_spritesheet_path(
    path: &str,
    num_frames: usize,
    program: &LumenpyxProgram,
) -> Vec<glium::Texture2d> {
    let image = load_image(path);
    let texture = texture::Texture2d::new(&program.display, image)
        .expect("failed to create texture when creating animation from spritesheet");

    return load_textures_from_spritesheet_tex(texture, num_frames, program);
}

fn load_albedo_from_spritesheet(
    texture: Texture,
    num_frames: usize,
    program: &LumenpyxProgram,
) -> Vec<glium::Texture2d> {
    match texture {
        Texture::Path(path) => load_textures_from_spritesheet_path(&path, num_frames, program),
        _ => panic!("The albedo texture must be a path to a spritesheet"),
    }
}

fn load_non_albedo_from_spritesheet(
    albedo_textures: &Vec<glium::Texture2d>,
    texture: Texture,
    program: &LumenpyxProgram,
) -> Vec<glium::Texture2d> {
    match texture {
        Texture::Path(path) => {
            load_textures_from_spritesheet_path(&path, albedo_textures.len(), program)
        }
        Texture::Texture(texture) => {
            load_textures_from_spritesheet_tex(texture, albedo_textures.len(), program)
        }
        _ => {
            let mut textures = vec![];
            for albedo_texture in albedo_textures {
                let new_texture = crate::primitives::new_non_albedo_texture(
                    program,
                    texture.try_clone(),
                    &albedo_texture,
                );

                textures.push(new_texture);
            }
            textures
        }
    }
}

fn load_normal_from_spritesheet(
    albedo_textures: &Vec<glium::Texture2d>,
    height_textures: &Vec<glium::Texture2d>,
    normal: Normal,
    program: &LumenpyxProgram,
) -> Vec<glium::Texture2d> {
    match normal {
        Normal::Path(path) => {
            load_textures_from_spritesheet_path(&path, albedo_textures.len(), program)
        }
        _ => {
            let mut textures = vec![];
            for (albedo_texture, height_texture) in zip(albedo_textures, height_textures) {
                let new_texture = crate::primitives::new_normal_texture(
                    program,
                    normal.try_clone(),
                    height_texture,
                    &albedo_texture,
                );

                textures.push(new_texture);
            }
            textures
        }
    }
}

/// Loads multiple images from a path in format `path1.png`, `path2.png`, etc.
fn load_tex_from_images_path(
    albedo_path: &str,
    num_frames: usize,
    program: &LumenpyxProgram,
) -> Vec<glium::Texture2d> {
    let path_parts;
    {
        let mut path_parts_fully_split = albedo_path.split_inclusive('.').collect::<Vec<&str>>();
        let mut path_parts_new: [String; 2] = ["".to_string(), "".to_string()];

        path_parts_new[1] = path_parts_fully_split
            .pop()
            .expect("Path must have a file extension")
            .to_string();

        for part in path_parts_fully_split.iter() {
            path_parts_new[0] += part;
        }

        path_parts = path_parts_new;
    }

    if path_parts.len() != 2 {
        panic!("Path must be in format `path1.png`, `path2.png`, etc.");
    }

    let mut textures = Vec::new();
    for i in 0..num_frames {
        let mut actual_path = path_parts[0].clone();
        actual_path.remove(actual_path.len() - 1);
        let file_extension = path_parts[1].clone();
        let full_path = format!(
            "{}{}.{}",
            actual_path, // remove the last character which is the .
            i + 1,
            file_extension
        );

        let image = load_image(&full_path);
        let texture = texture::Texture2d::new(&program.display, image)
            .expect("failed to create texture when creating animation from images");

        textures.push(texture);
    }

    textures
}

fn load_tex_from_images_albedo(
    albedo: Texture,
    num_frames: usize,
    program: &LumenpyxProgram,
) -> Vec<glium::Texture2d> {
    match albedo {
        Texture::Path(path) => load_tex_from_images_path(&path, num_frames, program),
        _ => panic!("The albedo texture must be a path to a series of images"),
    }
}

fn load_tex_from_images_non_albedo(
    albedo_textures: &Vec<glium::Texture2d>,
    texture: Texture,
    program: &LumenpyxProgram,
) -> Vec<glium::Texture2d> {
    match texture {
        Texture::Path(path) => load_tex_from_images_path(&path, albedo_textures.len(), program),
        Texture::Texture(texture) => panic!("Not sure how to handle this yet as the meaning is sort of ambiguous, if you need this feature please open an issue on the github page"),
        _ => {
            let mut textures = vec![];
            for albedo_texture in albedo_textures {
                let new_texture = crate::primitives::new_non_albedo_texture(
                    program,
                    texture.try_clone(),
                    &albedo_texture,
                );

                textures.push(new_texture);
            }
            textures
        }
    }
}

fn load_tex_from_images_normal(
    albedo_textures: &Vec<glium::Texture2d>,
    height_textures: &Vec<glium::Texture2d>,
    normal: Normal,
    program: &LumenpyxProgram,
) -> Vec<glium::Texture2d> {
    match normal {
        Normal::Path(path) => load_tex_from_images_path(&path, albedo_textures.len(), program),
        _ => {
            let mut textures = vec![];
            for (albedo_texture, height_texture) in zip(albedo_textures, height_textures) {
                let new_texture = crate::primitives::new_normal_texture(
                    program,
                    normal.try_clone(),
                    height_texture,
                    &albedo_texture,
                );

                textures.push(new_texture);
            }
            textures
        }
    }
}
