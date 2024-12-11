use macroquad::prelude::*;

use crate::{Wigette, WigetteType};

impl Wigette {
    pub const COLORS: [Color; 8] = [RED, PINK, BLUE, GREEN, PURPLE, YELLOW, ORANGE, MAGENTA];
    pub fn draw(&self, lvl : usize) {
        match &self.wigette_type {
            WigetteType::HBox{children, distended_x: _, distended_y: _} => {
                for child in children {
                    child.draw( lvl + 1 );
                }
            }
            WigetteType::VBox{children, distended_x: _, distended_y: _} => {
                for child in children {
                    child.draw( lvl + 1 );
                }
            }
            WigetteType::Label(label) => {
                //draw_rectangle(self.x as f32, self.y as f32, self.get_width() as f32, self.get_height() as f32, Self::COLORS[lvl]);
                let offset_y = measure_text(&label.text, None, label.font_size, 1.0).offset_y ;
                draw_text(&label.text, self.x as f32, self.y as f32 + offset_y, label.font_size as f32, label.color);
            }
            _ => {}
        }
    }
}    