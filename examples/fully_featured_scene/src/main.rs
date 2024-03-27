use lumenpyx::drawable_object::Drawable;
use lumenpyx::primitives::{Normal, Sprite, Texture};
use lumenpyx::{lights::LightDrawable, winit::event, *};

fn main() {
    //let (event_loop, window, display, indices) = setup_program();
    let (mut lumen_program, event_loop) = LumenpyxProgram::new([256, 256], "fully_featured_scene");

    let lights = vec![
        Box::new(lights::PointLight::new(
            [-1.52, 0.545, 1.0],
            [1.0, 0.76, 0.52],
            1.0,
            0.01,
        )),
        Box::new(lights::PointLight::new(
            [1.49, 0.545, 1.0],
            [1.0, 0.76, 0.52],
            1.0,
            0.01,
        )),
    ];

    // adapted from https://cainos.itch.io/pixel-art-platformer-village-props
    // Huge thank you to Cainos for the free assets!
    let mut scene_drawable_bottom = Sprite::new(
        "../images/Demo_Town/Demo-town-albedo-bottom.png".into(),
        "../images/Demo_Town/Demo-town-Heightmap-Bottom.png".into(),
        [0.0, 0.0, 0.0, 1.0].into(),
        [0.0, 0.96, 0.48, 1.0].into(),
        &lumen_program,
        Transform::new([0.0, 0.0, 0.0]),
    );
    scene_drawable_bottom.set_shadow_strength(0.0);

    let scene_drawable_top = Sprite::new(
        "../images/Demo_Town/Demo-town-albedo-Top.png".into(),
        "../images/Demo_Town/Demo-town-Heightmap-Top.png".into(),
        "../images/Demo_Town/Demo-town-roughness.png".into(),
        [0.0, 0.96, 0.48, 1.0].into(),
        &lumen_program,
        Transform::new([0.0, 0.0, 0.0]),
    );

    let mut distance_to_60_frame = 0.0;
    let mut start_of_60_frame = std::time::Instant::now();
    let mut camera = Camera::new([0.0, 0.0, 200.0]);

    let mut t: f32 = 0.0;
    lumen_program.run(event_loop, |mut program| {
        distance_to_60_frame -= 1.0;
        if distance_to_60_frame < 0.0 {
            println!("FPS: {}", 60.0 / start_of_60_frame.elapsed().as_secs_f32());
            distance_to_60_frame = 60.0;
            start_of_60_frame = std::time::Instant::now();
        }

        {
            t += 0.005;
            camera.position = [(t * 0.1).sin(), 0.0, 200.0];

            scene_drawable_bottom.transform.set_x((t * 0.1).sin() * 0.1)
        }

        let drawable_refs: Vec<&dyn Drawable> = vec![&scene_drawable_bottom, &scene_drawable_top];
        let light_refs: Vec<&dyn LightDrawable> =
            lights.iter().map(|l| &**l as &dyn LightDrawable).collect();
        draw_all(
            light_refs,
            drawable_refs,
            &mut program,
            &camera,
            DebugOption::None,
        );
    });
}
