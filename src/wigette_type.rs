use crate::{widgets::label_text::LabelText, Wigette};

#[allow(dead_code)]
pub enum WigetteType<'a> {
    Box,
    HBox {
        children: Vec<Wigette<'a>>,
        distended_width: u32,
        distended_height: u32,
    },
    VBox {
        children: Vec<Wigette<'a>>,
        distended_width: u32,
        distended_height: u32,
    },
    Label(LabelText<'a>),
}
