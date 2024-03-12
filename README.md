# lumenpyx
A unique 2D-pixel art renderer with heightmaps to simulate 3d shadows.

# Examples
This is the best example I have for now, the renderer is capable of much more than this, but I'm not a great artist.
![image](https://github.com/NoodlesOfWrath/lumenpyx/assets/76850177/def2e27c-ffaf-4a3c-908e-d65e376b5600)

# Getting Started
## Rendering a Sprite
```rust
use lumenpyx::{lights::LightDrawable, winit::event, *};
use rand::Rng;

fn main() {
    // setup your program at any resolution you would like
    let (mut lumen_program, event_loop) =
        LumenpyxProgram::new([(128.0 * (16.0 / 9.0)) as u32, 128]);

    // put a light in the scene to illuminate things, [0.0, 0.0, 1.0] will be in the middle
    // the falloff is also specified here we want virtually no falloff for this example so we do 0.02 with an intensity of 2.0
    let mut lights = vec![
        Box::new(lights::PointLight::new(
            [0.0, 0.0, 1.0],
            [1.0, 0.76, 0.52],
            2.0,
            0.02,
        )),
    ];

    // We make a new sprite passing in the display and indices
    let scene_drawable = Sprite::new(
        "../images/Demo-Scene-Albedo.png",
        "../images/Demo-Scene-Heightmap.png",
        "../images/Demo-Scene-Roughnessmap.png",
        &lumen_program.display,
        &lumen_program.indices,
        Transform::new([0.0, 0.0, 0.0]),
    );

    // set this up to check performance
    let mut distance_to_60_frame = 0.0;
    let mut start_of_60_frame = std::time::Instant::now();

    let mut t: f32 = 0.0;
    // all of this will mostly be unchanged, I will probably change this to be more elegant later
    event_loop
        .run(move |ev, window_target| match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    // don't change this
                    window_target.exit();
                }
                winit::event::WindowEvent::Resized(physical_size) => {
                    // don't change this
                    lumen_program.display.resize(physical_size.into());
                }
                winit::event::WindowEvent::RedrawRequested => {
                    // This is the part we want to change, it is called every frame.

                    // This part is optional for measuring performance
                    distance_to_60_frame -= 1.0;
                    if distance_to_60_frame < 0.0 {
                        println!("FPS: {}", 60.0 / start_of_60_frame.elapsed().as_secs_f32());
                        distance_to_60_frame = 60.0;
                        start_of_60_frame = std::time::Instant::now();
                    }

                    // We turn the lights and the drawables into their respective traits.
                    // I will show you how to create these later.
                    let drawable_refs: Vec<&dyn Drawable> = vec![&scene_drawable];
                    let light_refs: Vec<&dyn LightDrawable> =
                        lights.iter().map(|l| &**l as &dyn LightDrawable).collect();

                    // Finally, we draw all of them, this needs to happen every frame
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
```