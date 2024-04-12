use std::time::{Duration, Instant};

use lumenpyx::drawable_object::Drawable;
use lumenpyx::primitives::Sprite;
use lumenpyx::{lights::LightDrawable, *};

fn main() {
    //let (event_loop, window, display, indices) = setup_program();
    let (mut lumen_program, event_loop) = LumenpyxProgram::new([256, 256], "fully_featured_scene");
    lumen_program.set_render_settings(RenderSettings::default().with_render_resolution([512, 256]));

    let mut lights = vec![
        Box::new(lights::PointLight::new(
            [-195.0, 72.0, 1.0],
            [1.0, 0.76, 0.52],
            1.0,
            0.05,
        )),
        Box::new(lights::PointLight::new(
            [190.0, 72.0, 1.0],
            [1.0, 0.76, 0.52],
            1.0,
            0.05,
        )),
        Box::new(lights::PointLight::new(
            [0.0, 0.0, 1.0],
            [1.0, 0.76, 0.52],
            1.0,
            0.05,
        )),
    ];

    // adapted from https://cainos.itch.io/pixel-art-platformer-village-props
    // Huge thank you to Cainos for the free assets!
    let mut scene_drawable_bottom = Sprite::new(
        "../images/Demo_Town/Demo-town-albedo-bottom.png".into(),
        "../images/Demo_Town/Demo-town-Heightmap-Bottom.png".into(),
        [0.0, 0.0, 0.0, 1.0].into(),
        [0.0, 0.96, 0.48, 1.0].into(),
        &lumen_program,
        Transform::new([0.0, 0.0, 0.0]),
    );
    scene_drawable_bottom.set_shadow_strength(0.0);

    let mut scene_drawable_top = Sprite::new(
        "../images/Demo_Town/Demo-town-albedo-Top.png".into(),
        "../images/Demo_Town/Demo-town-Heightmap-Top.png".into(),
        "../images/Demo_Town/Demo-town-roughness.png".into(),
        [0.0, 0.96, 0.48, 1.0].into(),
        &lumen_program,
        Transform::new([0.0, 0.0, 0.0]),
    );
    scene_drawable_top.set_shadow_strength(1.0);

    // credit to https://jesse-m.itch.io/skeleton-pack for the free assets
    let mut skeleton_sprites = vec![];
    for i in 0..13 {
        let path = format!(
            "../images/Skeleton Walk/Animation/Skeleton Walk{}.png",
            i + 1
        );

        let mut transform = Transform::new([0.0, 13.0, 0.0]);
        transform.set_scale(2.0, 2.0, 1.0);
        let mut sprite = Sprite::new(
            path.into(),
            0.34.into(),
            0.0.into(),
            Default::default(),
            &lumen_program,
            Transform::new([0.0, 16.0, 0.0]),
        );
        sprite.set_shadow_strength(1.0);
        skeleton_sprites.push(sprite);
    }

    let mut distance_to_60_frame = 0.0;
    let mut start_of_60_frame = std::time::Instant::now();
    let mut camera = Camera::new([0.0, 0.0, 200.0]);

    let total_time = Instant::now();
    let mut duration_already_passed = Duration::new(0, 0);
    let mut direction = 1.0;
    let mut skeleton_x = 0.0;

    lumen_program.run(event_loop, |mut program| {
        let delta_time = total_time.elapsed() - duration_already_passed;
        duration_already_passed = total_time.elapsed();

        let delta_secs = delta_time.as_secs_f32() * 2.0;
        let total_secs = total_time.elapsed().as_secs_f32() * 2.0;

        distance_to_60_frame -= 1.0;
        if distance_to_60_frame < 0.0 {
            println!("FPS: {}", 60.0 / start_of_60_frame.elapsed().as_secs_f32());
            distance_to_60_frame = 60.0;
            start_of_60_frame = std::time::Instant::now();
        }

        let displayed_sprite = &mut skeleton_sprites[(total_secs * 10.0) as usize % 13];
        // walk back and forth on the x axis
        {
            let duration_to_walk_across = 10.0;
            skeleton_x += (delta_secs / duration_to_walk_across) * direction;
            if skeleton_x > 1.0 {
                direction = -1.0;
            } else if skeleton_x < -1.0 {
                direction = 1.0;
            }
            displayed_sprite.transform.set_x(skeleton_x * 128.0);

            if direction == 1.0 {
                displayed_sprite.transform.set_scale(1.0, 1.0, 1.0);
                lights[2].set_position(skeleton_x * 128.0 + 9.0, 16.0, 1.0);
            } else {
                displayed_sprite.transform.set_scale(-1.0, 1.0, 1.0);
                lights[2].set_position(skeleton_x * 128.0 - 9.0, 16.0, 1.0);
            }
        }

        {
            camera.position = [skeleton_x * 128.0, 0.0, 5.0];

            scene_drawable_bottom
                .transform
                .set_x((skeleton_x * 0.1) * 256.0);
        }

        let drawable_refs: Vec<&dyn Drawable> = vec![
            &scene_drawable_bottom,
            &scene_drawable_top,
            displayed_sprite,
        ];

        let light_refs: Vec<&dyn LightDrawable> =
            lights.iter().map(|l| &**l as &dyn LightDrawable).collect();
        draw_all(light_refs, drawable_refs, &mut program, &camera);
    });
}
