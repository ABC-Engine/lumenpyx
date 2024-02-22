use lumenpyx::*;
use rand::Rng;

fn main() {
    let (event_loop, window, display, indices) = setup_program();

    let paths = vec![
        "../images/Demo-Scene.png",
    ];

    let mut lights = vec![Light::new(
        [0.78, 0.28, 0.5],
        [1.0, 0.76, 0.52],
        2.0,
        0.02,
    ), 
    /*Light::new(
        [0.22, 0.28, 0.5],
        [1.0, 0.76, 0.52],
        2.0,
        0.02,
    )*/];
    
    let scene_drawable = DrawableObject::new(
        "../images/Demo-Scene.png",
        "../images/Demo-Scene-Heightmap.png",
        "../images/Demo-Scene.png",
        &display,
        Transform::new(),
    );

    // TODO: make this a little more elegant for the user
    let mut t: f32 = 0.0;
    event_loop
        .run(move |ev, window_target| match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                winit::event::WindowEvent::Resized(physical_size) => {
                    display.resize(physical_size.into());
                }
                winit::event::WindowEvent::RedrawRequested => {
                    {
                        let mut rng = rand::thread_rng();
                        t += 0.001 * rng.gen_range(0.0..1.0);
                for light in lights.iter_mut() {
                            light.set_position((t.sin() / 2.0) + 0.5, 0.28, 0.5);
                            light.set_intensity(1.0 + (t.sin() * 0.5));
                        }
                    }

                    let drawable_refs = vec![&scene_drawable];
                    let light_refs: Vec<&Light> = lights.iter().collect();
                    draw_all(&display, drawable_refs, light_refs, &indices);
                }
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => (),
        })
        .unwrap();
}
