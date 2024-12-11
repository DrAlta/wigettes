use macroquad::prelude::Color;



pub struct LabelText {
    pub text: String,
    pub font_size: u16,
    pub color: Color,
}
impl LabelText {
    pub fn new(text: String, font_size: u16, color:Color) -> Self {
        LabelText { text, font_size, color}
    }
}
