use std::borrow::BorrowMut;

use glium::Surface;
use image::{self, Pixel, Rgba, RgbaImage};
use parley::layout::{Alignment, Glyph, GlyphRun, Layout};
use parley::style::StyleProperty;
use parley::FontContext;
use parley::LayoutContext;
use swash::scale::image::Content;
use swash::scale::{Render, ScaleContext, Scaler, Source, StrikeWith};
use swash::zeno::{self, Vector};
use swash::FontRef;
use zeno::Format;

use crate::primitives::{draw_texture, BASE_FRAGMENT_SHADER_SRC, BASE_VERTEX_SHADER_SRC};
use crate::{Drawable, TextureHandle};

pub use parley::fontique::Collection;
pub use parley::style::{FontFamily, FontStack, GenericFamily};

// Note: make sure nothing is public in this struct or else the sprite will never be updated
/// Padding must be non zero
pub struct TextBox<'a> {
    data: TextBoxData,
    /*
    albedo_sprite: glium::texture::Texture2d,
    height_sprite: glium::texture::Texture2d,
    roughness_sprite: glium::texture::Texture2d,
    normal_sprite: glium::texture::Texture2d,
    */
    albedo_sprite: TextureHandle,
    height_sprite: TextureHandle,
    roughness_sprite: TextureHandle,
    normal_sprite: TextureHandle,
    transform: crate::Transform,
    font: Option<FontStack<'a>>,
    /// the height that will be used in the draw height
    height: f32,
    /// The roughness that will be used in the draw function
    roughness: f32,
    /// The normal that will be used in the draw function
    normal: [f32; 3],
}

impl<'a> TextBox<'a> {
    pub fn new(
        text: String,
        display_scale: f32,
        max_advance: Option<f32>,
        text_color: [u8; 4],
        padding: u32,
        lumenpyx_program: &mut crate::LumenpyxProgram,
    ) -> Self {
        if padding == 0 {
            panic!("Padding must be non zero");
        }

        let mut data = TextBoxData {
            text,
            display_scale,
            max_advance,
            text_color,
            padding,
            line_height: 1.3,
            font_size: 16.0,
        };

        let albedo_sprite = remake_text_box(&data, &None, lumenpyx_program);
        let height_sprite = remake_text_box(&data, &None, lumenpyx_program);
        let roughness_sprite = remake_text_box(&data, &None, lumenpyx_program);
        let normal_sprite = remake_text_box(&data, &None, lumenpyx_program);

        let albedo_sprite = lumenpyx_program.add_not_named_texture(albedo_sprite);
        let height_sprite = lumenpyx_program.add_not_named_texture(height_sprite);
        let roughness_sprite = lumenpyx_program.add_not_named_texture(roughness_sprite);
        let normal_sprite = lumenpyx_program.add_not_named_texture(normal_sprite);

        let mut new_self = Self {
            data,
            albedo_sprite,
            height_sprite,
            roughness_sprite,
            normal_sprite,
            transform: crate::Transform::default(),
            font: None,
            height: 0.0,
            roughness: 0.0,
            normal: [0.0, 0.0, 0.0],
        };

        // because we didn't draw the height, roughness, or normal properly
        new_self.redraw_all_textures(lumenpyx_program);

        new_self
    }

    pub fn redraw_all_textures(&mut self, lumenpyx_program: &mut crate::LumenpyxProgram) {
        let albedo_sprite = remake_text_box(&mut self.data, &self.font, lumenpyx_program);

        let mut height_data = self.data.clone();
        let height = (self.height * 255.0) as u8;
        height_data.text_color = [height, height, height, 255];
        let height_sprite = remake_text_box(&mut height_data, &self.font, lumenpyx_program);

        let mut roughness_data = self.data.clone();
        let roughness = (self.roughness * 255.0) as u8;
        roughness_data.text_color = [roughness, roughness, roughness, 255];
        let roughness_sprite = remake_text_box(&mut roughness_data, &self.font, lumenpyx_program);

        let normal = self
            .normal
            .iter()
            .map(|x| (x * 255.0) as u8)
            .collect::<Vec<u8>>();
        let mut normal_data = self.data.clone();
        normal_data.text_color = [normal[0], normal[1], normal[2], 255];
        let normal_sprite = remake_text_box(&mut normal_data, &self.font, lumenpyx_program);

        // free the old textures
        lumenpyx_program.remove_texture(&self.albedo_sprite);
        lumenpyx_program.remove_texture(&self.height_sprite);
        lumenpyx_program.remove_texture(&self.roughness_sprite);
        lumenpyx_program.remove_texture(&self.normal_sprite);

        // add the new textures
        self.albedo_sprite = lumenpyx_program.add_not_named_texture(albedo_sprite);
        self.height_sprite = lumenpyx_program.add_not_named_texture(height_sprite);
        self.roughness_sprite = lumenpyx_program.add_not_named_texture(roughness_sprite);
        self.normal_sprite = lumenpyx_program.add_not_named_texture(normal_sprite);
    }

    pub fn set_text(&mut self, text: String, lumenpyx_program: &mut crate::LumenpyxProgram) {
        self.data.text = text;
        self.redraw_all_textures(lumenpyx_program);
    }

    pub fn set_display_scale(
        &mut self,
        display_scale: f32,
        lumenpyx_program: &mut crate::LumenpyxProgram,
    ) {
        self.data.display_scale = display_scale;
        self.redraw_all_textures(lumenpyx_program);
    }

    pub fn set_max_advance(
        &mut self,
        max_advance: Option<f32>,
        lumenpyx_program: &mut crate::LumenpyxProgram,
    ) {
        self.data.max_advance = max_advance;
        self.redraw_all_textures(lumenpyx_program);
    }

    pub fn set_text_color(
        &mut self,
        text_color: [u8; 4],
        lumenpyx_program: &mut crate::LumenpyxProgram,
    ) {
        self.data.text_color = text_color;
        self.redraw_all_textures(lumenpyx_program);
    }

    pub fn set_padding(&mut self, padding: u32, lumenpyx_program: &mut crate::LumenpyxProgram) {
        if padding == 0 {
            panic!("Padding must be non zero");
        }
        self.data.padding = padding;
        self.redraw_all_textures(lumenpyx_program);
    }

    pub fn set_transform(&mut self, transform: crate::Transform) {
        self.transform = transform;
        // no need to update the sprite here since the transform is not used in the draw function
    }

    pub fn set_font_stack(
        &mut self,
        font_stack: FontStack<'a>,
        lumenpyx_program: &mut crate::LumenpyxProgram,
    ) {
        self.font = Some(font_stack);
        self.redraw_all_textures(lumenpyx_program);
    }

    pub fn get_font_stack(&self) -> Option<&FontStack<'a>> {
        self.font.as_ref()
    }

    pub fn get_padding(&self) -> u32 {
        self.data.padding
    }

    pub fn get_line_height(&self) -> f32 {
        self.data.line_height
    }

    pub fn get_font_size(&self) -> f32 {
        self.data.font_size
    }

    pub fn set_line_height(
        &mut self,
        line_height: f32,
        lumenpyx_program: &mut crate::LumenpyxProgram,
    ) {
        self.data.line_height = line_height;
        self.redraw_all_textures(lumenpyx_program);
    }

    pub fn set_font_size(&mut self, font_size: f32, lumenpyx_program: &mut crate::LumenpyxProgram) {
        self.data.font_size = font_size;
        self.redraw_all_textures(lumenpyx_program);
    }

    pub fn get_transform(&self) -> &crate::Transform {
        &self.transform
    }

    pub fn get_text(&self) -> &String {
        &self.data.text
    }

    pub fn get_display_scale(&self) -> f32 {
        self.data.display_scale
    }

    pub fn get_max_advance(&self) -> Option<f32> {
        self.data.max_advance
    }

    pub fn get_text_color(&self) -> [u8; 4] {
        self.data.text_color
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_roughness(&self) -> f32 {
        self.roughness
    }

    pub fn get_normal(&self) -> [f32; 3] {
        self.normal
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    pub fn set_roughness(&mut self, roughness: f32) {
        self.roughness = roughness;
    }

    pub fn set_normal(&mut self, normal: [f32; 3]) {
        self.normal = normal;
    }
}

impl<'a> Drawable for TextBox<'a> {
    fn draw_albedo(
        &self,
        program: &crate::LumenpyxProgram,
        transform: &crate::Transform,
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let albedo_sprite = program
            .get_texture_from_handle(&self.albedo_sprite)
            .expect("Failed to get albedo texture");
        let (width, height) = albedo_sprite.dimensions();

        // scale the transform matrix to match the size of the texture
        // check which side is longer and scale the other side to match
        let mut transform = transform.clone();
        let (width, height) = (width as f32, height as f32);

        // adjust size of the sprite to match the texture
        {
            let smallest_dimension = (albedo_framebuffer.get_dimensions().1 as f32)
                .min(albedo_framebuffer.get_dimensions().0 as f32);
            let x_scale = width / smallest_dimension;
            let y_scale = height / smallest_dimension;

            transform.set_scale(
                transform.get_scale()[0] * x_scale,
                transform.get_scale()[1] * y_scale,
                transform.get_scale()[2],
            );
        }

        draw_texture(
            &albedo_sprite,
            transform.get_matrix(),
            program,
            albedo_framebuffer,
        );
    }

    fn draw_normal(
        &self,
        program: &crate::LumenpyxProgram,
        transform: &crate::Transform,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let normal_sprite = program
            .get_texture_from_handle(&self.normal_sprite)
            .expect("Failed to get normal texture");
        let (width, height) = normal_sprite.dimensions();

        // scale the transform matrix to match the size of the texture
        // check which side is longer and scale the other side to match
        let mut transform = transform.clone();
        let (width, height) = (width as f32, height as f32);

        // adjust size of the sprite to match the texture
        {
            let smallest_dimension = (normal_framebuffer.get_dimensions().1 as f32)
                .min(normal_framebuffer.get_dimensions().0 as f32);
            let x_scale = width / smallest_dimension;
            let y_scale = height / smallest_dimension;

            transform.set_scale(
                transform.get_scale()[0] * x_scale,
                transform.get_scale()[1] * y_scale,
                transform.get_scale()[2],
            );
        }

        draw_texture(
            &normal_sprite,
            transform.get_matrix(),
            program,
            normal_framebuffer,
        );
    }

    fn draw_height(
        &self,
        program: &crate::LumenpyxProgram,
        transform: &crate::Transform,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let height_sprite = program
            .get_texture_from_handle(&self.height_sprite)
            .expect("Failed to get height texture");
        let (width, height) = height_sprite.dimensions();

        // scale the transform matrix to match the size of the texture
        // check which side is longer and scale the other side to match
        let mut transform = transform.clone();
        let (width, height) = (width as f32, height as f32);

        // adjust size of the sprite to match the texture
        {
            let smallest_dimension = (height_framebuffer.get_dimensions().1 as f32)
                .min(height_framebuffer.get_dimensions().0 as f32);
            let x_scale = width / smallest_dimension;
            let y_scale = height / smallest_dimension;

            transform.set_scale(
                transform.get_scale()[0] * x_scale,
                transform.get_scale()[1] * y_scale,
                transform.get_scale()[2],
            );
        }

        draw_texture(
            &height_sprite,
            transform.get_matrix(),
            program,
            height_framebuffer,
        );
    }

    fn draw_roughness(
        &self,
        program: &crate::LumenpyxProgram,
        transform: &crate::Transform,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let roughness_sprite = program
            .get_texture_from_handle(&self.roughness_sprite)
            .expect("Failed to get roughness texture");
        let (width, height) = roughness_sprite.dimensions();

        // scale the transform matrix to match the size of the texture
        // check which side is longer and scale the other side to match
        let mut transform = transform.clone();
        let (width, height) = (width as f32, height as f32);

        // adjust size of the sprite to match the texture
        {
            let smallest_dimension = (roughness_framebuffer.get_dimensions().1 as f32)
                .min(roughness_framebuffer.get_dimensions().0 as f32);
            let x_scale = width / smallest_dimension;
            let y_scale = height / smallest_dimension;

            transform.set_scale(
                transform.get_scale()[0] * x_scale,
                transform.get_scale()[1] * y_scale,
                transform.get_scale()[2],
            );
        }

        draw_texture(
            &roughness_sprite,
            transform.get_matrix(),
            program,
            roughness_framebuffer,
        );
    }

    fn try_load_shaders(&self, program: &mut crate::LumenpyxProgram) {
        if program.get_shader("sprite_shader").is_some() {
            return;
        }

        let new_shader = glium::Program::from_source(
            &program.display,
            BASE_VERTEX_SHADER_SRC,
            BASE_FRAGMENT_SHADER_SRC,
            None,
        )
        .expect("Failed to create sprite shader");

        program.add_shader(new_shader, "sprite_shader");
    }

    fn get_transform(&self) -> crate::Transform {
        self.transform
    }

    fn set_transform(&mut self, transform: crate::Transform) {
        self.transform = transform;
    }
}

#[derive(Clone)]
struct TextBoxData {
    text: String,
    display_scale: f32,
    max_advance: Option<f32>,
    text_color: [u8; 4],
    padding: u32,
    line_height: f32,
    font_size: f32,
    //font_collection: Option<Collection>,
}

fn remake_text_box(
    text_box_data: &TextBoxData,
    font_stack: &Option<FontStack<'_>>,
    lumenpyx_program: &mut crate::LumenpyxProgram,
) -> glium::Texture2d {
    // The text we are going to style and lay out
    let text = &text_box_data.text;

    // The display scale for HiDPI rendering
    let display_scale = text_box_data.display_scale;

    // The width for line wrapping
    let max_advance = text_box_data.max_advance;

    // Colours for rendering
    //let text_color = Color::rgb8(0, 0, 0);
    let text_color = text_box_data.text_color;
    let bg_color = Rgba([0, 0, 0, 0]);

    // Padding around the output image
    let padding = text_box_data.padding;

    // Create a FontContext, LayoutContext and ScaleContext
    //
    // These are all intended to be constructed rarely (perhaps even once per app (or once per thread))
    // and provide caches and scratch space to avoid allocations
    let font_cx_ref;
    if let Some(font_cx) = &mut lumenpyx_program.font_context {
        font_cx_ref = font_cx;
    } else {
        lumenpyx_program.font_context = Some(FontContext::default());
        font_cx_ref = lumenpyx_program.font_context.as_mut().unwrap();
    }

    let layout_cx_ref;
    if let Some(layout_cx) = &mut lumenpyx_program.layout_context {
        layout_cx_ref = layout_cx;
    } else {
        lumenpyx_program.layout_context = Some(LayoutContext::default());
        layout_cx_ref = lumenpyx_program.layout_context.as_mut().unwrap();
    }

    let scale_cx_ref;
    if let Some(scale_cx) = &mut lumenpyx_program.scale_context {
        scale_cx_ref = scale_cx;
    } else {
        lumenpyx_program.scale_context = Some(ScaleContext::default());
        scale_cx_ref = lumenpyx_program.scale_context.as_mut().unwrap();
    }

    // Create a RangedBuilder
    let mut builder = layout_cx_ref.ranged_builder(font_cx_ref, &text, display_scale);

    // Set default text colour styles (set foreground text color)
    let brush_style = StyleProperty::Brush(text_color);
    builder.push_default(&brush_style);

    if let Some(font_stack) = font_stack {
        // Set default font family
        let font_stack_style = StyleProperty::FontStack(*font_stack);
        builder.push_default(&font_stack_style);
    } else {
        let font_stack = FontStack::Source("system-ui");
        let font_stack_style = StyleProperty::FontStack(font_stack);
        builder.push_default(&font_stack_style);
    }

    // Set default font family
    builder.push_default(&StyleProperty::LineHeight(text_box_data.line_height));
    builder.push_default(&StyleProperty::FontSize(text_box_data.font_size));

    // Build the builder into a Layout
    let mut layout: Layout<[u8; 4]> = builder.build();

    // Perform layout (including bidi resolution and shaping) with start alignment
    layout.break_all_lines(max_advance, Alignment::Start);

    // Create image to render into
    let width = layout.width().ceil() as u32 + (padding * 2);
    let height = layout.height().ceil() as u32 + (padding * 2);
    let mut img = RgbaImage::from_pixel(width, height, bg_color);

    // Iterate over laid out lines
    for line in layout.lines() {
        // Iterate over GlyphRun's within each line
        for glyph_run in line.glyph_runs() {
            render_glyph_run(scale_cx_ref, &glyph_run, &mut img, padding);
        }
    }

    // make a texture from the image
    let (width, height) = img.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&img, (width, height));
    let display = &lumenpyx_program.display;
    let texture = glium::texture::Texture2d::new(display, image).unwrap();

    texture
}

fn render_glyph_run(
    context: &mut ScaleContext,
    glyph_run: &GlyphRun<[u8; 4]>,
    img: &mut RgbaImage,
    padding: u32,
) {
    // Resolve properties of the GlyphRun
    let mut run_x = glyph_run.offset();
    let run_y = glyph_run.baseline();
    let style = glyph_run.style();
    let color = style.brush;

    // Get the "Run" from the "GlyphRun"
    let run = glyph_run.run();

    // Resolve properties of the Run
    let font = run.font();
    let font_size = run.font_size();
    let normalized_coords = run.normalized_coords();

    // Convert from parley::Font to swash::FontRef
    let font_ref = FontRef::from_index(font.data.as_ref(), font.index as usize).unwrap();

    // Build a scaler. As the font properties are constant across an entire run of glyphs
    // we can build one scaler for the run and reuse it for each glyph.
    let mut scaler = context
        .builder(font_ref)
        .size(font_size)
        .hint(true)
        .normalized_coords(normalized_coords)
        .build();

    // Iterates over the glyphs in the GlyphRun
    for glyph in glyph_run.glyphs() {
        let glyph_x = run_x + glyph.x + (padding as f32);
        let glyph_y = run_y - glyph.y + (padding as f32);
        run_x += glyph.advance;

        render_glyph(img, &mut scaler, color, glyph, glyph_x, glyph_y);
    }
}

fn render_glyph(
    img: &mut RgbaImage,
    scaler: &mut Scaler,
    color: [u8; 4],
    glyph: Glyph,
    glyph_x: f32,
    glyph_y: f32,
) {
    // Render the glyph using swash
    let rendered_glyph = Render::new(
        // Select our source order
        &[
            Source::ColorOutline(0),
            Source::ColorBitmap(StrikeWith::BestFit),
            Source::Outline,
        ],
    )
    // Select the simple alpha (non-subpixel) format
    .format(Format::Alpha)
    // Render the image
    .render(scaler, glyph.id)
    .unwrap();

    let glyph_width = rendered_glyph.placement.width;
    let glyph_height = rendered_glyph.placement.height;
    let glyph_x = (glyph_x.floor() as i32 + rendered_glyph.placement.left) as u32;
    let glyph_y = (glyph_y.floor() as i32 - rendered_glyph.placement.top) as u32;

    match rendered_glyph.content {
        Content::Mask => {
            let mut i = 0;
            for pixel_y in 0..glyph_height {
                for pixel_x in 0..glyph_width {
                    let x = glyph_x + pixel_x;
                    let y = glyph_y + pixel_y;
                    if x >= img.width() || y >= img.height() {
                        continue;
                    }
                    let alpha = rendered_glyph.data[i];
                    // this might be too big of an assumption, but i assume people want fully opaque text
                    let alpha = if alpha > 128 { 255 } else { 0 };
                    let color = Rgba([
                        color[0],
                        color[1],
                        color[2],
                        (alpha as f32 * (color[3] as f32 / 255.0)) as u8,
                    ]);
                    img.get_pixel_mut(x, y).blend(&color);
                    i += 1;
                }
            }
        }
        Content::SubpixelMask => unimplemented!(),
        Content::Color => {
            let row_size = glyph_width as usize * 4;
            for (pixel_y, row) in rendered_glyph.data.chunks_exact(row_size).enumerate() {
                for (pixel_x, pixel) in row.chunks_exact(4).enumerate() {
                    let x = glyph_x + pixel_x as u32;
                    let y = glyph_y + pixel_y as u32;
                    let color = Rgba(pixel.try_into().expect("Not RGBA"));
                    img.get_pixel_mut(x, y).blend(&color);
                }
            }
        }
    };
}
