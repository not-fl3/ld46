use macroquad::*;

#[macroquad::main("LD46!!!!")]
async fn main() {
    let texture = load_texture("rocket.png").await;

    let mut rocket_pos = vec2(0., 0.);

    loop {
        clear_background(BLACK);

        begin_mode_2d(Camera2D {
            target: rocket_pos,
            zoom: 0.003 - get_time() as f32 / 2000.,
            ..Default::default()
        });
        *rocket_pos.y_mut() += get_time() as f32 / 10.;

        draw_texture_ex(
            texture,
            rocket_pos.x() - texture.width() / 2.,
            rocket_pos.y() + texture.height(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(texture.width(), -texture.height())),
                ..Default::default()
            },
        );
        draw_circle_lines(0., -500., 500., 2., WHITE);

        end_mode_2d();

        next_frame().await
    }
}
