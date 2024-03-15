use lumenpyx::{lights::LightDrawable, winit::event, *};
use rand::Rng;

fn main() {
    //let (event_loop, window, display, indices) = setup_program();
    let (mut lumen_program, event_loop) = LumenpyxProgram::new([128, 128]);

    let mut lights: Vec<Box<dyn LightDrawable>> = vec![];

    let scene_drawable = Sprite::new(
        "../images/Demo-Scene-Albedo.png".into(),
        "../images/Demo-Scene-Heightmap.png".into(),
        "../images/Demo-Scene-Roughnessmap.png".into(),
        &lumen_program.display,
        &lumen_program.indices,
        Transform::new([0.0, 0.0, 0.0]),
    );

    let mut distance_to_60_frame = 0.0;
    let mut start_of_60_frame = std::time::Instant::now();

    let mut t: f32 = 0.0;
    lumen_program.run(event_loop, |mut program| {
        distance_to_60_frame -= 1.0;
        if distance_to_60_frame < 0.0 {
            println!(
                "FPS: {} with {} lights",
                (60.0 / start_of_60_frame.elapsed().as_secs_f32()).round(),
                lights.len()
            );
            distance_to_60_frame = 60.0;

            let random_position = [
                rand::thread_rng().gen_range(0.0..1.0),
                rand::thread_rng().gen_range(0.0..1.0),
                rand::thread_rng().gen_range(0.0..1.0),
            ];

            /*lights.push(Box::new(lights::PointLight::new(
                random_position,
                [1.0, 0.76, 0.52],
                0.1,
                0.02,
            )));*/

            lights.push(Box::new(lights::AreaLight::new(
                random_position,
                [1.0, 0.76, 0.52],
                0.1,
                0.02,
                0.1,
                0.1,
            )));

            // this needs to be after the push so that the time is accurate
            start_of_60_frame = std::time::Instant::now();
        }

        let drawable_refs: Vec<&dyn Drawable> = vec![&scene_drawable];
        let light_refs: Vec<&dyn LightDrawable> = lights.iter().map(|l| l.as_ref()).collect();
        draw_all(
            light_refs,
            drawable_refs,
            &mut program,
            &Camera::new([0.0, 0.0, 0.0]),
        );
    });
}
