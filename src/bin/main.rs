use macroquad::prelude::*;

use wigettes::widgets::button::draw_button;

#[macroquad::main("Widget test")]
pub async fn main() {
    let camera = Camera2D {
        zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()) * 5.0,
        target: Vec2::ZERO,
        ..Default::default()
    };
    set_camera(&camera);

    loop {
        clear_background(WHITE);

        //let mouse_pos: Vec2 = mouse_position().into();
        draw_button(0.0, 0.0, 50.0, 20.0);
        next_frame().await;
    }
}

/*

...*****
.**....
.*
*.
*.
*.

*/