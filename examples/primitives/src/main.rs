use lumenpyx::primitives::*;
use lumenpyx::*;

fn main() {
    let (event_loop, window, display, indices) = setup_program();

    let mut drawables = vec![];
    let mut lights = vec![Light::new([0.5, 1.0, 0.5], [1.0, 1.0, 1.0], 2.0, 0.01)];

    drawables.push(Sphere::new([0.7, 0.3, 0.0, 1.0], 0.5, Transform::new()));

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
                        t += 0.001;
                        lights[0].set_position((t.sin() + 1.0) / 2.0, 0.5, 1.0);
                    }

                    let drawable_refs: Vec<&Sphere> = drawables.iter().collect();
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