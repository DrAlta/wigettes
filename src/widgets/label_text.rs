use macroquad::prelude::*;

use crate::Widget;

pub struct LabelText<'a> {
    pub text: String,
    pub font: &'a Font,
    pub font_size: u16,
    pub color: Color,
}
impl<'a> LabelText<'a> {
    pub fn new(text: String, font: &'a Font, font_size: u16, color: Color) -> Self {
        LabelText {
            text,
            font,
            font_size,
            color,
        }
    }
}
impl<'a> Widget for LabelText<'a> {
    fn draw(&self, x: f32, y: f32, _: f32, _: f32) {
        let offset_y = measure_text(&self.text, Some(self.font), self.font_size, 1.0).offset_y;
        draw_text(
            &self.text,
            x,
            y + offset_y,
            self.font_size as f32,
            self.color,
        );
    }

    fn get_dimensions(&self) -> (f32, f32) {
        let x = measure_text(&self.text, Some(self.font), self.font_size, 1.0);
        (x.width, x.height)
    }
}
