use lumenpyx::animation::Animation;
use lumenpyx::drawable_object::Drawable;
use lumenpyx::primitives::{Normal, Sprite, Texture};
use lumenpyx::{lights::LightDrawable, winit::event, *};

fn main() {
    //let (event_loop, window, display, indices) = setup_program();
    let (mut lumen_program, event_loop) =
        LumenpyxProgram::new([(128.0 * (16.0 / 9.0)) as u32, 128], "animation test");

    let lights = vec![Box::new(lights::PointLight::new(
        [0.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        1.0,
        0.00,
    ))];

    let mut animation = Animation::new_from_images(
        "../images/Skeleton Walk/Animation/Skeleton Walk.png".into(),
        0.0.into(),
        0.0.into(),
        Default::default(),
        13,
        std::time::Duration::from_millis(100),
        Transform::new([28.0, 0.0, 0.0]),
        &lumen_program,
    );

    let animation_2 = Animation::new_from_spritesheet(
        "../images/Skeleton Walk/Skeleton Walk.png".into(),
        0.0.into(),
        0.0.into(),
        [0.0, 0.0, 0.0, 0.0].into(),
        13,
        std::time::Duration::from_millis(100),
        Transform::new([-28.0, 0.0, 0.0]),
        &lumen_program,
    );

    let camera = Camera::new([0.0, 0.0, 1.5]);

    lumen_program.run(event_loop, |mut program| {
        let light_refs = lights.iter().map(|l| &**l as &dyn LightDrawable).collect();
        let drawable_refs: Vec<&dyn Drawable> = vec![&animation, &animation_2];
        draw_all(light_refs, drawable_refs, &mut program, &camera);
    });
}
