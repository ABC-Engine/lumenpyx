use lumenpyx::lights::LightDrawable;
use lumenpyx::primitives::*;
use lumenpyx::*;

fn main() {
    let (mut lumen_program, event_loop) = LumenpyxProgram::new();

    let mut drawables: Vec<Box<dyn Drawable>> = vec![];
    let mut lights: Vec<Box<dyn LightDrawable>> = vec![Box::new(lights::PointLight::new(
        [0.5, 1.0, 0.5],
        [1.0, 1.0, 1.0],
        2.0,
        0.01,
    ))];

    // this hightlights an issue, the radius seems to be 2x what it should be
    drawables.push(Box::new(Sphere::new(
        [0.7, 0.3, 0.0, 1.0],
        0.1,
        Transform::new([-0.2, 0.0, 0.0]),
    )));
    drawables.push(Box::new(Circle::new(
        [0.0, 0.0, 1.0, 1.0],
        0.1,
        Transform::new([0.0, 0.0, 0.0]),
    )));
    drawables.push(Box::new(Rectangle::new(
        [1.0, 1.0, 1.0, 1.0],
        0.2,
        0.2,
        Transform::new([0.2, 0.0, 0.0]),
    )));

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
                    {
                        t += 0.001;
                        //lights[0].set_position((t.sin() + 1.0) / 2.0, 0.5, 1.0);
                    }

                    let drawable_refs: Vec<&dyn Drawable> =
                        drawables.iter().map(|d| d.as_ref()).collect();
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
