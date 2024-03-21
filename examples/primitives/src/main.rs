use lumenpyx::lights::LightDrawable;
use lumenpyx::primitives::*;
use lumenpyx::*;

fn main() {
    let (mut lumen_program, event_loop) = LumenpyxProgram::new([128, 128], "primitives");

    let mut drawables: Vec<Box<dyn Drawable>> = vec![];
    let mut lights = vec![Box::new(lights::PointLight::new(
        [0.5, 1.0, 0.5],
        [1.0, 1.0, 1.0],
        2.0,
        0.02,
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
    drawables.push(Box::new(Cylinder::new(
        [1.0, 0.0, 0.0, 1.0],
        0.1,
        0.4,
        Transform::new([0.4, -0.2, 0.0]),
    )));

    let mut t: f32 = 0.0;
    lumen_program.run(event_loop, |mut program| {
        {
            t += 0.001;
            lights[0].set_position(t.sin(), 0.5, 0.5);
        }

        let drawable_refs: Vec<&dyn Drawable> = drawables.iter().map(|d| d.as_ref()).collect();
        let light_refs: Vec<&dyn LightDrawable> =
            lights.iter().map(|l| &**l as &dyn LightDrawable).collect();

        draw_all(
            light_refs,
            drawable_refs,
            &mut program,
            &Camera::new([0.0, 0.0, 0.0]),
        );
    });
}
