Here 's how you render a sprite using lumenpyx. This uses the sprite primitve to render a full scene with no moving parts.

```rust
use lumenpyx::{lights::LightDrawable, winit::event, *};
use lumenpyx::drawable_object::Drawable;
use lumenpyx::primitives::{Sprite, Normal};

fn main() {
    // setup your program at any resolution you would like
    let (mut lumen_program, event_loop) =
        LumenpyxProgram::new([(128.0 * (16.0 / 9.0)) as u32, 128], "name of your program");

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
    // alternatively we can do a solid color for any of these by passing in [r: f32, g: f32, b: f32, a: f32].into()
    let scene_drawable = Sprite::new(
        "examples/images/Demo-Scene-Albedo.png".into(),
        "examples/images/Demo-Scene-Heightmap.png".into(),
        "examples/images/Demo-Scene-Roughnessmap.png".into(),
        Normal::AutoGenerated,
        &lumen_program,
        Transform::new([0.0, 0.0, 0.0]),
    );

    // make a camera, to specify the position we would like to view everything from
    let camera = Camera::new([0.0,0.0,0.0]);

    // set this up to check performance
    let mut distance_to_60_frame = 0.0;
    let mut start_of_60_frame = std::time::Instant::now();

    // this is to run the program for forever or until returned
    lumen_program.run(event_loop, |mut program| {
        // all of the code inside this function will be run every frame

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
        draw_all(light_refs, drawable_refs, &mut program, &camera);
    });
}
```