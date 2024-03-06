use lumenpyx::{winit::event, *};
use rand::Rng;

fn main() {
    //let (event_loop, window, display, indices) = setup_program();
    let (mut lumen_program, event_loop) = LumenpyxProgram::new();

    let mut lights = vec![
        Light::new([0.78, 0.28, 1.0], [1.0, 0.76, 0.52], 3.0, 0.02),
        //Light::new([0.22, 0.28, 1.0], [1.0, 0.76, 0.52], 2.0, 0.02),
    ];

    let scene_drawable = Sprite::new(
        "../images/Demo-Scene-Albedo.png",
        "../images/Demo-Scene-Heightmap.png",
        "../images/Demo-Scene-Roughnessmap.png",
        &lumen_program.display,
        &lumen_program.indices,
        Transform::new(),
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
                        println!("FPS: {}", 60.0 / start_of_60_frame.elapsed().as_secs_f32());
                        distance_to_60_frame = 60.0;
                        start_of_60_frame = std::time::Instant::now();
                    }

                    {
                        let mut rng = rand::thread_rng();
                        t += 0.01 * rng.gen_range(0.0..1.0);
                        for light in lights.iter_mut() {
                            //light.set_position((t.sin() / 2.0) + 0.5, 0.28, 0.5);
                            light.set_position(0.78, 0.28, ((t.sin() / 2.0) + 0.5) * 5.0);
                            //light.set_intensity(1.0 + (t.sin() * 0.5));
                        }
                    }

                    let drawable_refs = vec![&scene_drawable];
                    let light_refs: Vec<&Light> = lights.iter().collect();
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
