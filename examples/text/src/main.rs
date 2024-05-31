use lumenpyx::drawable_object::Drawable;
use lumenpyx::primitives::Rectangle;
use lumenpyx::*;
use text::Collection;

fn main() {
    //let (event_loop, window, display, indices) = setup_program();
    let (mut lumen_program, event_loop) = LumenpyxProgram::new([256, 256], "text test");

    lumen_program.set_render_settings(
        RenderSettings::default()
            .with_shadows(false)
            .with_reflections(false),
    );

    let mut text = text::TextBox::new(
        "Hello, World!!!!!!!!!!!!!!!".to_string(),
        1.0,
        Some(50.0),
        [0, 0, 0, 255],
        1,
        &mut lumen_program,
    );

    lumen_program.add_font_to_collection(include_bytes!("../m3x6.ttf").to_vec());
    text.set_font_stack(
        text::FontStack::Single(text::FontFamily::Named("m3x6")),
        &mut lumen_program,
    );

    text.set_font_size(16.3, &mut lumen_program);
    let mut new_transform = Transform::new([0.0, 0.0, 0.0]);
    new_transform.set_scale(1.0, 1.0, 1.0);
    text.set_transform(new_transform);

    let background = Rectangle::new([0.0, 1.0, 0.0, 1.0], 256.0, 256.0, Transform::default());

    let mut distance_to_60_frame = 0;
    let mut start_of_60_frame = std::time::Instant::now();
    let camera = Camera::new([0.0, 0.0, 1.5]);

    lumen_program.run(event_loop, |mut program| {
        distance_to_60_frame -= 1;
        if distance_to_60_frame < 0 {
            println!("FPS: {}", 60.0 / start_of_60_frame.elapsed().as_secs_f32());
            distance_to_60_frame = 60;
            start_of_60_frame = std::time::Instant::now();
        }

        let drawable_refs: Vec<&dyn Drawable> = vec![&background, &text];
        draw_all(vec![], drawable_refs, &mut program, &camera);
    });
}
