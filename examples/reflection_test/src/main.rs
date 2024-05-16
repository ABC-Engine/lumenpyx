use lumenpyx::drawable_object::Drawable;
use lumenpyx::primitives::{Normal, NormalInput, Sprite, Texture};
use lumenpyx::{lights::LightDrawable, winit::event, *};

fn main() {
    //let (event_loop, window, display, indices) = setup_program();
    let (mut lumen_program, event_loop) =
        LumenpyxProgram::new([(128.0 * (16.0 / 9.0)) as u32, 128], "reflection_test");
    //lumen_program.set_debug(DebugOption::Roughness);

    let lights = vec![Box::new(lights::AreaLight::new(
        [0.0, 0.0, 0.0],
        [1.0, 1.0, 1.0],
        1.5,
        0.01,
        20.0,
        20.0,
    ))];

    let scene_drawable = Sprite::new(
        "../images/Test Grid Color.png".into(),
        "../images/Test Grid Color Heightmap.png".into(),
        "../images/Test Grid Color Roughnessmap.png".into(),
        NormalInput::default(),
        &mut lumen_program,
        Transform::new([0.0, 0.0, 0.0]),
    )
    .0;

    let mut distance_to_60_frame = 0.0;
    let mut start_of_60_frame = std::time::Instant::now();

    let mut t: f32 = 0.0;
    lumen_program.run(event_loop, |mut program| {
        distance_to_60_frame -= 1.0;
        if distance_to_60_frame < 0.0 {
            println!(
                "FPS: {}",
                (60.0 / start_of_60_frame.elapsed().as_secs_f32()) as u32
            );
            distance_to_60_frame = 60.0;
            start_of_60_frame = std::time::Instant::now();
        }
        t += 0.01;

        let drawable_refs: Vec<&dyn Drawable> = vec![&scene_drawable];
        let light_refs: Vec<&dyn LightDrawable> =
            lights.iter().map(|l| &**l as &dyn LightDrawable).collect();

        draw_all(
            light_refs,
            drawable_refs,
            &mut program,
            &Camera::new([t.sin() * 128.0, 0.0, 1.0]),
        );
    });
}
