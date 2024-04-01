This is not necessary to do, the default drawables that come with the library should be enough for most. However, if you have a solid understanding of OpenGL here is how you make a custom drawable.

```rust
use lumenpyx::drawable_object::Drawable;
use lumenpyx::LumenpyxProgram;
use glium::framebuffer::SimpleFrameBuffer;
use glium::uniform;
use glium::Surface;
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
    fn draw(
        &self,
        program: &LumenpyxProgram,
        matrix_transform: [[f32; 4]; 4],
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        // we won't use this as we are making a 2d circle so the height will be constant
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        // the normal and roughness are only needed if this drawable is going to be reflecting things,
        // otherwise just don't draw to them
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

        // if we wanted a constant height for our heightmap we could do
        // height is normally from 0-1
        let height = 0.5;
        // last term is alpha so it should always be 1.0
        height_framebuffer.clear_color(height, height, height, 1.0)
    }

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
    fn get_position(&self) -> [[f32; 4]; 4] {
        self.transform.get_matrix()
    }
}
```