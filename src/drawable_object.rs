use crate::load_image;
use crate::shaders::draw_generate_normals;
use crate::shaders::FULL_SCREEN_QUAD;
use crate::LumenpyxProgram;
use crate::Transform;
use crate::DEFAULT_BEHAVIOR;
use glium;
use glium::uniform;
use glium::Surface;

/// A trait for objects that can be drawn to the screen.
/// Every primitive implements this trait.
/// If you want to create a custom object, you will need to implement this trait.
/// If you do, use the tutorial here: https://github.com/ABC-Engine/lumenpyx/wiki/Creating-custom-drawable-objects
pub trait Drawable {
    /// Draw the object to the screen
    fn draw(
        &self,
        program: &LumenpyxProgram,
        transform_matrix: [[f32; 4]; 4],
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    );

    /// Load the shaders for the object
    /// This is called every frame, so make sure to check
    /// if the shader is already loaded or your performance will suffer
    fn try_load_shaders(&self, program: &mut LumenpyxProgram);

    fn get_position(&self) -> [[f32; 4]; 4];

    fn get_recieve_shadows_strength(&self) -> f32 {
        0.5
    }

    fn set_transform(&mut self, transform: Transform);
}
