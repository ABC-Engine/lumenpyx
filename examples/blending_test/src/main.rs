use lumenpyx::blending::{BlendMode, BlendObject};
use lumenpyx::drawable_object::Drawable;
use lumenpyx::primitives::{Normal, NormalInput, Rectangle, Sprite, Texture};
use lumenpyx::{lights::LightDrawable, winit::event, *};

fn main() {
    //let (event_loop, window, display, indices) = setup_program();
    let (mut lumen_program, event_loop) =
        LumenpyxProgram::new([(128.0 * (16.0 / 9.0)) as u32, 128], "dungeon");

    lumen_program.set_render_settings(
        RenderSettings::default()
            .with_shadows(false)
            .with_reflections(false),
    );

    let mut lights = vec![
        Box::new(lights::PointLight::new(
            [0.56, -0.44, 1.0],
            [1.0, 0.76, 0.52],
            2.0,
            0.02,
        )),
        Box::new(lights::PointLight::new(
            [-0.545, -0.44, 1.0],
            [1.0, 0.76, 0.52],
            2.0,
            0.02,
        )),
    ];

    let mut scene_drawable = Sprite::new(
        "../images/Demo-Scene-Albedo.png".into(),
        "../images/Demo-Scene-Heightmap.png".into(),
        "../images/Demo-Scene-Roughnessmap.png".into(),
        NormalInput::default(),
        &mut lumen_program,
        Transform::new([0.0, 0.0, 0.0]),
    )
    .0;

    let mut square = Rectangle::new([0.0, 0.0, 0.0, 0.5], 20.0, 20.0, Transform::default());

    let background = Rectangle::new([0.0, 1.0, 0.0, 1.0], 128.0, 128.0, Transform::default());

    let mut distance_to_60_frame = 0;
    let mut start_of_60_frame = std::time::Instant::now();
    let time_elapsed = std::time::Instant::now();
    let mut camera = Camera::new([0.0, 0.0, 1.5]);

    lumen_program.run(event_loop, |mut program| {
        distance_to_60_frame -= 1;
        if distance_to_60_frame < 0 {
            println!("FPS: {}", 60.0 / start_of_60_frame.elapsed().as_secs_f32());
            distance_to_60_frame = 60;
            start_of_60_frame = std::time::Instant::now();
        }
        {
            let mut new_transform = Transform::new([
                (time_elapsed.elapsed().as_secs_f32() / 4.0).sin() * 64.0,
                0.0,
                0.0,
            ]);
            new_transform.set_rotation(time_elapsed.elapsed().as_secs_f32());
            square.set_transform(new_transform);
            camera.position[0] = (time_elapsed.elapsed().as_secs_f32()).sin() * 64.0;
        }

        let blend_object = BlendObject::new(
            &scene_drawable,
            &square,
            // the square is a mask, so it should set the alpha of that area to 0.5 subtracted from the original alpha
            BlendMode::Subtractive,
        );
        let drawable_refs: Vec<&dyn Drawable> = vec![&background, &blend_object];
        let light_refs: Vec<&dyn LightDrawable> =
            lights.iter().map(|l| &**l as &dyn LightDrawable).collect();
        draw_all(light_refs, drawable_refs, &mut program, &camera);
    });
}
