use macroquad::{
    prelude::Color,
    text::{measure_text, Font},
};
use qol::logy;

use crate::{widgets::label_text::LabelText, Results, Widget, WigetteType};

pub struct Wigette<'a> {
    pub(super) wigette_type: WigetteType<'a>,
    pub(super) desired_width: u32,
    pub(super) desired_height: u32,
    pub(super) x: i64,
    pub(super) y: i64,
    pub(super) height: u32,
    pub(super) width: u32,
    pub(super) expand_width: bool,
    pub(super) expand_height: bool,
}

impl<'a> Wigette<'a> {
    pub(super) fn get_expand_width(&self) -> bool {
        self.expand_width
    }
    pub(super) fn get_expand_height(&self) -> bool {
        self.expand_height
    }
    pub(super) fn get_min_width(&self) -> u32 {
        match &self.wigette_type {
            WigetteType::Box => self.desired_width,
            WigetteType::HBox {
                children: _,
                distended_width,
                distended_height: _,
            } => distended_width.max(&self.desired_width).clone(),
            WigetteType::Label(label) => {
                (label.get_width().ceil() as u32).max(self.desired_width.clone())
            }
            WigetteType::VBox {
                distended_width, ..
            } => distended_width.max(&self.desired_width).clone(),
        }
    }
    pub(super) fn get_min_height(&self) -> u32 {
        match &self.wigette_type {
            WigetteType::Box => self.desired_height,
            WigetteType::HBox {
                distended_height, ..
            } => distended_height.max(&self.desired_height).clone(),
            WigetteType::Label(label) => {
                (label.get_height().ceil() as u32).max(self.desired_height.clone())
            }
            WigetteType::VBox {
                distended_height, ..
            } => distended_height.max(&self.desired_height).clone(),
        }
    }
    pub(super) fn set_size(&mut self, width: u32, height: u32) {
        let mut update = false;
        if width < self.desired_width {
            logy!("error", "tried to set width to less than min");
        } else {
            if self.width != width {
                self.width = width;
                update = true;
            }
        }

        if height < self.desired_height {
            logy!("error", "tried to set height to less than min");
        } else {
            if self.height != height {
                self.height = height;
                update = true;
            }
        }
        if update {
            self.update_size();
        }
    }
    /*
    fn set_size_y(&mut self, size: u32) {
        if size < self.min_y {
            logy!("error", "tried to set heigh to less than min");
        } else {
            if self.size_y != size {
                self.size_y = size;
                self.update_layout()
            }
        }
    }
    */
    pub(super) fn get_height(&self) -> u32 {
        let basic_y = self.height.max(self.get_min_height());
        if let WigetteType::Label(label) = &self.wigette_type {
            (measure_text(&label.text, None, label.font_size, 1.0).height as u32).max(basic_y)
        } else {
            basic_y
        }
    }
    pub(super) fn get_width(&self) -> u32 {
        let basic_width = self.width.max(self.get_min_width());
        if let WigetteType::Label(label) = &self.wigette_type {
            (measure_text(&label.text, None, label.font_size, 1.0).width as u32).max(basic_width)
        } else {
            basic_width
        }
    }
}

// public functions
//creating functions
impl<'a> Wigette<'a> {
    pub fn new_label(
        width: u32,
        heigth: u32,
        expand_width: bool,
        expand_height: bool,
        text: String,
        font: &'a Font,
        font_size: u16,
        color: Color,
    ) -> Self {
        Wigette {
            wigette_type: WigetteType::Label(LabelText::new(text, font, font_size, color)),
            desired_width: width,
            width,
            desired_height: heigth,
            height: heigth,
            x: 0,
            y: 0,
            expand_width,
            expand_height,
        }
    }
    pub fn new_box(width: u32, heigth: u32, expand_width: bool, expand_height: bool) -> Self {
        Wigette {
            wigette_type: WigetteType::Box,
            desired_width: width,
            width,
            desired_height: heigth,
            height: heigth,
            x: 0,
            y: 0,
            expand_width,
            expand_height,
        }
    }
    pub fn new_hbox(
        width: u32,
        heigth: u32,
        expand_width: bool,
        expand_height: bool,
        children: Vec<Wigette<'a>>,
    ) -> Self {
        Wigette {
            wigette_type: WigetteType::HBox {
                children,
                distended_width: 0,
                distended_height: 0,
            },
            desired_width: width,
            width,
            desired_height: heigth,
            height: heigth,
            x: 0,
            y: 0,
            expand_width,
            expand_height,
        }
    }
    pub fn new_vbox(
        width: u32,
        height: u32,
        expand_width: bool,
        expand_height: bool,
        children: Vec<Wigette<'a>>,
    ) -> Self {
        Wigette {
            wigette_type: WigetteType::VBox {
                children,
                distended_width: 0,
                distended_height: 0,
            },
            desired_width: width,
            width,
            desired_height: height,
            height,
            x: 0,
            y: 0,
            expand_width,
            expand_height,
        }
    }
}

//misc public funtions
impl<'a> Wigette<'a> {
    pub fn set_pos(&mut self, x: i64, y: i64) {
        /*
        if let WigetteType::Box = self.wigette_type  {
            logy!("debug", "x: {x} Y: {y}");
        }
        */
        let mut update = false;
        if self.x != x {
            update = true;
            self.x = x;
        }
        if self.y != y {
            update = true;
            self.y = y;
        }
        if update {
            self.update_childrens_pos(2)
        }
    }
    pub fn get_child(&'a self, index: usize) -> Option<&'a Wigette<'a>> {
        match &self.wigette_type {
            WigetteType::HBox { children, .. } => children.get(index),
            WigetteType::VBox { children, .. } => children.get(index),
            _ => None,
        }
    }
    pub fn get_child_mut(&'a mut self, index: usize) -> Option<&'a mut Wigette<'a>> {
       let x: Option<&mut Wigette<'a>>=  match &mut self.wigette_type {
            WigetteType::HBox { children, .. } => children.get_mut(index),
            WigetteType::VBox { children, .. } => children.get_mut(index),
            _ => None,
        };
        x
    }
    pub fn external_facing_draw(&mut self) {
        self.update_size();
        self.update_childrens_pos(2);
        self.draw(0);
    }
}

//text_label
impl<'a> Wigette<'a> {
    pub fn set_text(&mut self, text: String) -> Results<()> {
        match &mut self.wigette_type {
            WigetteType::Label(inner) => {
                inner.text = text;
                Ok(())
            }
            _ => Err("tried to set the text on not a text label".into()),
        }
    }
    pub fn set_text_color(&mut self, text: String, color: Color) -> Results<()> {
        match &mut self.wigette_type {
            WigetteType::Label(inner) => {
                inner.text = text;
                inner.color = color;
                Ok(())
            }
            _ => Err("tried to set the text on not a text label".into()),
        }
    }
}
