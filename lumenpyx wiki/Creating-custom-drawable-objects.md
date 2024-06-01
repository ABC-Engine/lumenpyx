This is not necessary to do, the default drawables that come with the library should be enough for most. However, if you have a solid understanding of OpenGL here is how you make a custom drawable.

```rust
use glium::uniform;
use glium::Surface;
use lumenpyx::drawable_object::Drawable;
use lumenpyx::LumenpyxProgram;
use lumenpyx::Transform;
use lumenpyx::Vertex;

// put your glsl shader in a file and reference it here
// if you don't do this the shader won't be including in the binary and it will panic
const GENERATE_CIRCLE_VERTEX_SHADER_SRC: &str =
    include_str!(r"..\shaders\primitives\circle_ahr_shader.vert");
const GENERATE_CIRCLE_FRAGMENT_SHADER_SRC: &str =
    include_str!(r"..\shaders\primitives\circle_ahr_shader.frag");

pub struct Circle {
    color: [f32; 4],
    radius: f32,
    transform: Transform,
}

impl Drawable for Circle {
    fn draw_albedo(
        &self,
        program: &LumenpyxProgram,
        transform: &Transform,
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
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
            matrix: transform.get_matrix(), // notice we use the transform passed in, not the one in the struct (the one passed in is correctly scaled and positioned based on the camera)
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

    // there are virtually identical functions for the other framebuffers (height, normal, and roughness)

    // this is called every frame, so make sure to check if the shader is already loaded
    fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
        // check if the shader is loaded
        if program.get_shader("circle_ahr_shader").is_none() {
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
    }

    // this is so that objects scale properly with camera movement and dimensions
    fn get_transform(&self) -> Transform {
        self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}
```