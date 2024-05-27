use image::{self, Pixel, Rgba, RgbaImage};
use parley::layout::{Alignment, Glyph, GlyphRun, Layout};
use parley::style::{FontWeight, StyleProperty};
use parley::FontContext;
use parley::LayoutContext;
use swash::scale::image::Content;
use swash::scale::{Render, ScaleContext, Scaler, Source, StrikeWith};
use swash::zeno;
use swash::FontRef;
use zeno::{Format, Vector};

use crate::primitives::{draw_texture, BASE_FRAGMENT_SHADER_SRC, BASE_VERTEX_SHADER_SRC};
use crate::Drawable;

pub use parley::style::{FontFamily, FontStack, GenericFamily};

// Note: make sure nothing is public in this struct or else the sprite will never be updated
/// Padding must be non zero
pub struct TextBox<'a> {
    data: TextBoxData,
    sprite: glium::texture::Texture2d,
    transform: crate::Transform,
    font: Option<FontStack<'a>>,
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

        let sprite = remake_text_box(&mut data, &None, lumenpyx_program);

        Self {
            data,
            sprite,
            transform: crate::Transform::default(),
            font: None,
        }
    }

    pub fn set_text(&mut self, text: String, lumenpyx_program: &mut crate::LumenpyxProgram) {
        self.data.text = text;
        self.sprite = remake_text_box(&mut self.data, &self.font, lumenpyx_program);
    }

    pub fn set_display_scale(
        &mut self,
        display_scale: f32,
        lumenpyx_program: &mut crate::LumenpyxProgram,
    ) {
        self.data.display_scale = display_scale;
        self.sprite = remake_text_box(&mut self.data, &self.font, lumenpyx_program);
    }

    pub fn set_max_advance(
        &mut self,
        max_advance: Option<f32>,
        lumenpyx_program: &mut crate::LumenpyxProgram,
    ) {
        self.data.max_advance = max_advance;
        self.sprite = remake_text_box(&mut self.data, &self.font, lumenpyx_program);
    }

    pub fn set_text_color(
        &mut self,
        text_color: [u8; 4],
        lumenpyx_program: &mut crate::LumenpyxProgram,
    ) {
        self.data.text_color = text_color;
        self.sprite = remake_text_box(&mut self.data, &self.font, lumenpyx_program);
    }

    pub fn set_padding(&mut self, padding: u32, lumenpyx_program: &mut crate::LumenpyxProgram) {
        if padding == 0 {
            panic!("Padding must be non zero");
        }
        self.data.padding = padding;
        self.sprite = remake_text_box(&mut self.data, &self.font, lumenpyx_program);
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
        self.sprite = remake_text_box(&mut self.data, &self.font, lumenpyx_program);
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
        self.sprite = remake_text_box(&mut self.data, &self.font, lumenpyx_program);
    }

    pub fn set_font_size(&mut self, font_size: f32, lumenpyx_program: &mut crate::LumenpyxProgram) {
        self.data.font_size = font_size;
        self.sprite = remake_text_box(&mut self.data, &self.font, lumenpyx_program);
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
}

impl<'a> Drawable for TextBox<'a> {
    fn draw_albedo(
        &self,
        program: &crate::LumenpyxProgram,
        transform: &crate::Transform,
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        draw_texture(
            &self.sprite,
            transform.get_matrix(),
            program,
            albedo_framebuffer,
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
        crate::Transform::default()
    }

    fn set_transform(&mut self, transform: crate::Transform) {
        self.transform = transform;
    }
}

struct TextBoxData {
    text: String,
    display_scale: f32,
    max_advance: Option<f32>,
    text_color: [u8; 4],
    padding: u32,
    line_height: f32,
    font_size: f32,
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
    let mut font_cx = FontContext::default();
    let mut layout_cx = LayoutContext::new();
    let mut scale_cx = ScaleContext::new();

    // Create a RangedBuilder
    let mut builder = layout_cx.ranged_builder(&mut font_cx, &text, display_scale);

    // Set default text colour styles (set foreground text color)
    let brush_style = StyleProperty::Brush(text_color);
    builder.push_default(&brush_style);

    if let Some(font_stack) = font_stack {
        println!("Font stack: {:?}", font_stack);
        // Set default font family
        let font_stack_style = StyleProperty::FontStack(*font_stack);
        builder.push_default(&font_stack_style);
    } else {
        let font_stack = FontStack::Source("system-ui");
        let font_stack_style = StyleProperty::FontStack(font_stack);
        builder.push_default(&font_stack_style);
    }

    // Set default font family
    builder.push_default(&StyleProperty::LineHeight(1.3));
    builder.push_default(&StyleProperty::FontSize(16.0));

    // Set the first 4 characters to bold
    let bold = FontWeight::new(600.0);
    let bold_style = StyleProperty::FontWeight(bold);
    builder.push(&bold_style, 0..4);

    // Build the builder into a Layout
    let mut layout: Layout<[u8; 4]> = builder.build();

    // Perform layout (including bidi resolution and shaping) with start alignment
    layout.break_all_lines(max_advance, Alignment::Start);

    // Create image to render into
    let width = layout.width().ceil() as u32 + (padding * 2);
    let height = layout.height().ceil() as u32 + (padding * 2);
    println!("Width: {}, Height: {}", width, height);
    let mut img = RgbaImage::from_pixel(width, height, bg_color);

    // Iterate over laid out lines
    for line in layout.lines() {
        // Iterate over GlyphRun's within each line
        for glyph_run in line.glyph_runs() {
            render_glyph_run(&mut scale_cx, &glyph_run, &mut img, padding);
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
    // Compute the fractional offset
    // You'll likely want to quantize this in a real renderer
    let offset = Vector::new(glyph_x.fract(), glyph_y.fract());

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
    // Apply the fractional offset
    .offset(offset)
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
                    let alpha = rendered_glyph.data[i];
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
