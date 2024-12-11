use crate::{Wigette, widgets::label_text::LabelText};


#[allow(dead_code)]
pub enum WigetteType {
    Box,
    HBox{children:Vec<Wigette>, distended_x: u32, distended_y: u32},
    VBox{children:Vec<Wigette>, distended_x: u32, distended_y: u32},
    Label(LabelText),
}
