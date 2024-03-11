use lumenpyx::{lights::LightDrawable, winit::event, *};
use rand::Rng;

fn main() {
    //let (event_loop, window, display, indices) = setup_program();
    let (mut lumen_program, event_loop) = LumenpyxProgram::new([128, 128]);

    let mut lights: Vec<Box<dyn LightDrawable>> = vec![];

    let scene_drawable = Sprite::new(
        "../images/Demo-Scene-Albedo.png",
        "../images/Demo-Scene-Heightmap.png",
        "../images/Demo-Scene-Roughnessmap.png",
        &lumen_program.display,
        &lumen_program.indices,
        Transform::new([0.0, 0.0, 0.0]),
    );

    let mut distance_to_60_frame = 0.0;
    let mut start_of_60_frame = std::time::Instant::now();

    // TODO: make this a little more elegant for the user
    let mut t: f32 = 0.0;
    event_loop
        .run(move |ev, window_target| match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                winit::event::WindowEvent::Resized(physical_size) => {
                    lumen_program.display.resize(physical_size.into());
                }
                winit::event::WindowEvent::RedrawRequested => {
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

                    {
                        let mut rng = rand::thread_rng();
                        t += 0.01 * rng.gen_range(0.0..1.0);
                        for light in lights.iter_mut() {
                            //light.set_position((t.sin() / 2.0) + 0.5, 0.28, 0.5);
                            // light.set_position(0.78, 0.28, ((t.sin() / 2.0) + 0.5) * 5.0);
                            //light.set_intensity(1.0 + (t.sin() * 0.5));
                        }
                    }

                    let drawable_refs: Vec<&dyn Drawable> = vec![&scene_drawable];
                    let light_refs: Vec<&dyn LightDrawable> =
                        lights.iter().map(|l| l.as_ref()).collect();
                    draw_all(light_refs, drawable_refs, &mut lumen_program);
                }
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                lumen_program.window.request_redraw();
            }
            _ => (),
        })
        .unwrap();
}
