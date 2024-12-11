use macroquad::prelude::*;

pub fn draw_button(x: f32, y: f32, w:f32, h: f32) {
    let y2 = y + h;
    let x2 = x + w;

    //top
    draw_rectangle(x + 1.0, y + 1.0, 2.0, 1.0, BLACK);
    draw_rectangle(x + 3.0, y, w - 5.0, 1.0, BLACK);
    draw_rectangle(x2 - 2.0, y + 1.0, 2.0, 1.0, BLACK);

    // left
    draw_rectangle(x + 1.0, y + 2.0, 1.0, 1.0, BLACK);
    draw_rectangle(x , y + 3.0, 1.0, h - 5.0, BLACK);
    draw_rectangle(x + 1.0, y2 - 2.0, 1.0, 2.0, BLACK);

    // bottom
    draw_rectangle(x + 1.0, y2 - 1.0, 2.0, 1.0, BLACK);
    draw_rectangle(x + 3.0, y2, w - 5.0, 1.0, BLACK);
    draw_rectangle(x + w - 2.0, y2 - 1.0, 2.0, 1.0, BLACK);

    // right
    draw_rectangle(x2 - 1.0, y + 2.0, 1.0, 1.0, BLACK);
    draw_rectangle(x2 , y + 3.0, 1.0, h - 5.0, BLACK);
    draw_rectangle(x2 - 1.0, y2 - 2.0, 1.0, 2.0, BLACK);

}
