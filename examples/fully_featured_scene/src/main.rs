use lumenpyx::Texture;
use lumenpyx::{lights::LightDrawable, winit::event, *};
use rand::Rng;

fn main() {
    //let (event_loop, window, display, indices) = setup_program();
    let (mut lumen_program, event_loop) = LumenpyxProgram::new([256, 256], "fully_featured_scene");

    let mut lights = vec![
        Box::new(lights::PointLight::new(
            [0.56, 0.5, 2.0],
            [1.0, 0.76, 0.52],
            1.0,
            0.01,
        )),
        Box::new(lights::PointLight::new(
            [-0.545, 0.5, 2.0],
            [1.0, 0.76, 0.52],
            1.0,
            0.01,
        )),
    ];

    // adapted from https://cainos.itch.io/pixel-art-platformer-village-props
    // Huge thank you to Cainos for the free assets!
    let scene_drawable = Sprite::new(
        "../images/Demo_Town/Demo-town-albedo.png".into(),
        "../images/Demo_Town/Demo-town-Heightmap.png".into(),
        "../images/Demo_Town/Demo-town-roughness.png".into(),
        [0.01, 0.96, 0.48, 1.0].into(),
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
            for light in lights.iter_mut() {
                let mut rng = rand::thread_rng();
                t += rng.gen_range(0.0..0.01);
                light.set_intensity(1.0 + ((t * 0.1).sin() * 0.5) as f32);
                //light.set_position((t.sin() * 0.5) as f32, (t.cos() * 0.5) as f32, 1.0);
                camera.position = [(t * 0.1).sin(), 0.0, 200.0];
                println!("{:?}", camera.position);
            }
        }

        let drawable_refs: Vec<&dyn Drawable> = vec![&scene_drawable];
        let light_refs: Vec<&dyn LightDrawable> =
            lights.iter().map(|l| &**l as &dyn LightDrawable).collect();
        draw_all(light_refs, drawable_refs, &mut program, &camera);
    });
}
