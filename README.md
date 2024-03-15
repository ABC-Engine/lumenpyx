# lumenpyx
A unique 2D-pixel art renderer with heightmaps to simulate 3d shadows.

# Examples
This is the best example I have for now, the renderer is capable of much more than this, but I'm not a great artist.
![image](https://github.com/NoodlesOfWrath/lumenpyx/assets/76850177/def2e27c-ffaf-4a3c-908e-d65e376b5600)

# Getting Started
## Rendering a Sprite
```rust
use lumenpyx::{lights::LightDrawable, winit::event, *};

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
    // alternatively we can do a solid color for any of these by passing in [r: f32, g: f32, b: f32, a: f32].into()
    let scene_drawable = Sprite::new(
        "examples/images/Demo-Scene-Albedo.png".into(),
        "examples/images/Demo-Scene-Heightmap.png".into(),
        "examples/images/Demo-Scene-Roughnessmap.png".into(),
        &lumen_program.display,
        &lumen_program.indices,
        Transform::new([0.0, 0.0, 0.0]),
    );

    // make a camera, to specify the position we would like to view everything from
    let camera = Camera::new([0.0,0.0,0.0]);

    // set this up to check performance
    let mut distance_to_60_frame = 0.0;
    let mut start_of_60_frame = std::time::Instant::now();

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
                    draw_all(light_refs, drawable_refs, &mut lumen_program, &camera);
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

## Creating custom drawable objects
This is definitely not neccesary to do, the default drawables that come with the library should be enough for most. However, if you have a solid understanding of opengl here is how you make a custom drawable.

```rust
use lumenpyx::Drawable;
use lumenpyx::LumenpyxProgram;
use glium::framebuffer::SimpleFrameBuffer;
use glium::uniform;
use glium::Surface;
use lumenpyx::Transform;
use lumenpyx::Vertex;

// put your glsl shader in a file and reference it here
// if you don't do this the shader won't be including in the binary and it will panic
const GENERATE_CIRCLE_VERTEX_SHADER_SRC: &str =
    include_str!(r"examples\shaders\ahr_shaders\circle_ahr_shader.vert");
const GENERATE_CIRCLE_FRAGMENT_SHADER_SRC: &str =
    include_str!(r"examples\shaders\ahr_shaders\circle_ahr_shader.frag");


pub struct Circle {
    color: [f32; 4],
    radius: f32,
    transform: Transform,
}

impl Drawable for Circle {
    fn draw(
        &self,
        program: &LumenpyxProgram,
        matrix_transform: [[f32; 4]; 4],
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let color = self.color;
        let radius = self.radius;
        let transform = self.transform;

        let display = &program.display;
        let indices = &program.indices;

        // attempt to load the shader
        // as long as the load_shaders function was setup correctly, this shouldn't panic
        let shader = program.get_shader("circle_ahr_shader").unwrap();

        // this is a whole screen shape
        let shape = lumenpyx::shaders::FULL_SCREEN_QUAD;

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

        // these are setup by name in the glsl shader file at the top of the file
        // ex.
        // ```glsl
        // uniform vec4 circle color
        // ```
        let uniforms = &uniform! {
            circle_color: color,
            radius_squared: radius.powi(2),
            matrix: matrix_transform,
        };

        albedo_framebuffer
            .draw(
                &vertex_buffer,
                indices,
                &shader,
                uniforms,
                &Default::default(),
            )
            .unwrap();
    }

    // this is called every frame, so make sure to check if the shader is already loaded
    fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
        // check if the shader is loaded
        if program.get_shader("circle_ahr_shader").is_some() {
            // if it is we are done
            return;
        }

        // if not we create the shader
        let shader = glium::Program::from_source(
            &program.display,
            GENERATE_CIRCLE_VERTEX_SHADER_SRC,
            GENERATE_CIRCLE_FRAGMENT_SHADER_SRC,
            None,
        )
        .unwrap();

        // then we add the shader to the program to be accessed later
        program.add_shader(shader, "circle_ahr_shader");
    }

    // this is so that objects scale properly with camera movement and dimensions
    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.get_matrix()
    }
}
```

