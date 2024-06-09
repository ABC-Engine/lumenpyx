use glium::texture;

use crate::load_image;
use crate::primitives::{Normal, Sprite, Texture, TextureInput};
use crate::TextureHandle;
use crate::Transform;
use crate::{drawable_object::Drawable, LumenpyxProgram};
use glium::Surface;
use std::iter::zip;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Animation {
    sprites: Vec<Sprite>,
    time_between_frames: Duration,
    time: AnimationTimeElapsed,
    shadow_strength: f32,
    pub transform: Transform,
    /// If true, the animation will loop, if false, the animation will not draw after the last frame
    loop_animation: bool,
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
        program: &mut LumenpyxProgram,
        loop_animation: bool,
    ) -> (
        Self,
        Vec<TextureHandle>,
        Vec<TextureHandle>,
        Vec<TextureHandle>,
        Vec<TextureHandle>,
    ) {
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
        let mut albedo_handles = vec![];
        let mut height_handles = vec![];
        let mut roughness_handles = vec![];
        let mut normal_handles = vec![];
        for _ in 0..num_frames {
            let albedo_texture = albedo_textures.remove(0);
            let height_texture = height_textures.remove(0);
            let roughness_texture = roughness_textures.remove(0);
            let normal_texture = normal_textures.remove(0);

            let (sprite, albedo_handle, height_handle, roughness_handle, normal_handle) =
                Sprite::new(
                    albedo_texture.into(),
                    height_texture.into(),
                    roughness_texture.into(),
                    normal_texture.into(),
                    program,
                    transform,
                );
            albedo_handles.push(albedo_handle);
            height_handles.push(height_handle);
            roughness_handles.push(roughness_handle);
            normal_handles.push(normal_handle);
            sprites.push(sprite);
        }

        (
            Self {
                sprites,
                time_between_frames,
                time: Instant::now().into(),
                shadow_strength: 0.5,
                transform,
                loop_animation,
            },
            albedo_handles,
            height_handles,
            roughness_handles,
            normal_handles,
        )
    }

    /// Takes a path to a spritesheet
    /// returns an Animation object and the handles to the textures in the order of albedo, height, roughness, normal
    pub fn new_from_spritesheet(
        albedo: Texture,
        height: Texture,
        roughness: Texture,
        normal: Normal,
        num_frames: usize,
        time_between_frames: Duration,
        transform: Transform,
        program: &mut LumenpyxProgram,
        loop_animation: bool,
    ) -> (
        Self,
        Vec<TextureHandle>,
        Vec<TextureHandle>,
        Vec<TextureHandle>,
        Vec<TextureHandle>,
    ) {
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
        let mut albedo_handles = vec![];
        let mut height_handles = vec![];
        let mut roughness_handles = vec![];
        let mut normal_handles = vec![];
        for _ in 0..num_frames {
            let albedo_texture = albedo_textures.remove(0);
            let height_texture = height_textures.remove(0);
            let roughness_texture = roughness_textures.remove(0);
            let normal_texture = normal_textures.remove(0);

            let (sprite, albedo_handle, height_handle, roughness_handle, normal_handle) =
                Sprite::new(
                    albedo_texture.into(),
                    height_texture.into(),
                    roughness_texture.into(),
                    normal_texture.into(),
                    program,
                    transform,
                );

            albedo_handles.push(albedo_handle);
            height_handles.push(height_handle);
            roughness_handles.push(roughness_handle);
            normal_handles.push(normal_handle);
            sprites.push(sprite);
        }

        (
            Self {
                sprites,
                time_between_frames,
                time: Instant::now().into(),
                shadow_strength: 0.5,
                transform,
                loop_animation,
            },
            albedo_handles,
            height_handles,
            roughness_handles,
            normal_handles,
        )
    }

    pub fn new_from_handles(
        albedo: Vec<TextureHandle>,
        height: Vec<TextureHandle>,
        roughness: Vec<TextureHandle>,
        normal: Vec<TextureHandle>,
        program: &mut LumenpyxProgram,
        time_between_frames: Duration,
        transform: Transform,
        loop_animation: bool,
    ) -> Self {
        let mut sprites = vec![];
        for i in 0..albedo.len() {
            let (sprite, _, _, _, _) = Sprite::new(
                albedo[i].clone().into(),
                height[i].clone().into(),
                roughness[i].clone().into(),
                normal[i].clone().into(),
                program,
                transform,
            );
            sprites.push(sprite);
        }

        Self {
            sprites,
            time_between_frames,
            time: Instant::now().into(),
            shadow_strength: 0.5,
            transform,
            loop_animation,
        }
    }

    pub fn restart_animation(&mut self) {
        self.time = Instant::now().into();
    }

    pub fn set_time(&mut self, time: AnimationTimeElapsed) {
        self.time = time;
    }
}

#[derive(Clone)]
pub enum AnimationTimeElapsed {
    Time(Duration),
    SecondsSinceStart(f32),
    TimeSinceInstant(Instant),
}

impl AnimationTimeElapsed {
    pub fn as_nanos(&self) -> u128 {
        match self {
            AnimationTimeElapsed::Time(time) => time.as_nanos(),
            AnimationTimeElapsed::SecondsSinceStart(seconds) => {
                (*seconds as f64 * 1_000_000_000.0) as u128
            }
            AnimationTimeElapsed::TimeSinceInstant(instant) => instant.elapsed().as_nanos(),
        }
    }

    pub fn as_secs_f32(&self) -> f32 {
        match self {
            AnimationTimeElapsed::Time(time) => time.as_secs_f32(),
            AnimationTimeElapsed::SecondsSinceStart(seconds) => *seconds,
            AnimationTimeElapsed::TimeSinceInstant(instant) => instant.elapsed().as_secs_f32(),
        }
    }
}

impl Into<AnimationTimeElapsed> for Duration {
    fn into(self) -> AnimationTimeElapsed {
        AnimationTimeElapsed::Time(self)
    }
}

impl Into<AnimationTimeElapsed> for f32 {
    fn into(self) -> AnimationTimeElapsed {
        AnimationTimeElapsed::SecondsSinceStart(self)
    }
}

impl Into<AnimationTimeElapsed> for Instant {
    fn into(self) -> AnimationTimeElapsed {
        AnimationTimeElapsed::TimeSinceInstant(self)
    }
}

impl Drawable for Animation {
    fn draw_albedo(
        &self,
        program: &LumenpyxProgram,
        transform: &Transform,
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let mut current_frame_num = self
            .time
            .as_nanos()
            .checked_div(self.time_between_frames.as_nanos())
            .expect("time between frames on an animation cannot be set to 0");

        if current_frame_num as usize >= self.sprites.len() {
            if self.loop_animation {
                current_frame_num = current_frame_num % self.sprites.len() as u128;
            } else {
                return;
            }
        }

        let current_frame = &self.sprites[current_frame_num as usize];

        current_frame.draw_albedo(program, transform, albedo_framebuffer);
    }

    fn draw_height(
        &self,
        program: &LumenpyxProgram,
        transform: &Transform,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let mut current_frame_num = self
            .time
            .as_nanos()
            .checked_div(self.time_between_frames.as_nanos())
            .expect("time between frames on an animation cannot be set to 0");

        if current_frame_num as usize >= self.sprites.len() {
            if self.loop_animation {
                current_frame_num = current_frame_num % self.sprites.len() as u128;
            } else {
                return;
            }
        }

        let current_frame = &self.sprites[current_frame_num as usize];

        current_frame.draw_height(program, transform, height_framebuffer);
    }

    fn draw_roughness(
        &self,
        program: &LumenpyxProgram,
        transform: &Transform,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let mut current_frame_num = self
            .time
            .as_nanos()
            .checked_div(self.time_between_frames.as_nanos())
            .expect("time between frames on an animation cannot be set to 0");

        if current_frame_num as usize >= self.sprites.len() {
            if self.loop_animation {
                current_frame_num = current_frame_num % self.sprites.len() as u128;
            } else {
                return;
            }
        }

        let current_frame = &self.sprites[current_frame_num as usize];

        current_frame.draw_roughness(program, transform, roughness_framebuffer);
    }

    fn draw_normal(
        &self,
        program: &LumenpyxProgram,
        transform: &Transform,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let mut current_frame_num = self
            .time
            .as_nanos()
            .checked_div(self.time_between_frames.as_nanos())
            .expect("time between frames on an animation cannot be set to 0");

        if current_frame_num as usize >= self.sprites.len() {
            if self.loop_animation {
                current_frame_num = current_frame_num % self.sprites.len() as u128;
            } else {
                return;
            }
        }

        let current_frame = &self.sprites[current_frame_num as usize];

        current_frame.draw_normal(program, transform, normal_framebuffer);
    }

    fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
        for sprite in &self.sprites {
            sprite.try_load_shaders(program);
        }
    }

    fn get_transform(&self) -> Transform {
        self.transform
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
    texture: &glium::Texture2d,
    num_frames: usize,
    program: &LumenpyxProgram,
) -> Vec<glium::Texture2d> {
    let texture_framebuffer = glium::framebuffer::SimpleFrameBuffer::new(&program.display, texture)
        .expect("failed to create texture framebuffer when creating animation from spritesheet");

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

    return load_textures_from_spritesheet_tex(&texture, num_frames, program);
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
            load_textures_from_spritesheet_tex(&texture, albedo_textures.len(), program)
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

pub struct AnimationStateMachine {
    transform: Transform, // not the most efficient way to do this, but it works
    animations: Vec<Animation>,
    current_animation: usize,
}

impl AnimationStateMachine {
    pub fn new(animations: Vec<Animation>) -> Self {
        Self {
            transform: Transform::default(),
            animations,
            current_animation: 0,
        }
    }

    pub fn set_current_animation(&mut self, index: usize) {
        self.current_animation = index;
    }

    /// sets the time of all animations in the state machine
    pub fn set_time(&mut self, time: AnimationTimeElapsed) {
        for animation in &mut self.animations {
            animation.set_time(time.clone());
        }
    }

    pub fn restart_current_animation(&mut self) {
        self.animations[self.current_animation].restart_animation();
    }
}

impl Drawable for AnimationStateMachine {
    fn draw_albedo(
        &self,
        program: &LumenpyxProgram,
        transform: &Transform,
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        self.animations[self.current_animation].draw_albedo(program, transform, albedo_framebuffer);
    }

    fn draw_height(
        &self,
        program: &LumenpyxProgram,
        transform: &Transform,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        self.animations[self.current_animation].draw_height(program, transform, height_framebuffer);
    }

    fn draw_roughness(
        &self,
        program: &LumenpyxProgram,
        transform: &Transform,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        self.animations[self.current_animation].draw_roughness(
            program,
            transform,
            roughness_framebuffer,
        );
    }

    fn draw_normal(
        &self,
        program: &LumenpyxProgram,
        transform: &Transform,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        self.animations[self.current_animation].draw_normal(program, transform, normal_framebuffer);
    }

    fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
        for animation in &self.animations {
            animation.try_load_shaders(program);
        }
    }

    fn get_transform(&self) -> Transform {
        self.transform
    }

    fn get_recieve_shadows_strength(&self) -> f32 {
        self.animations[self.current_animation].get_recieve_shadows_strength()
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}
