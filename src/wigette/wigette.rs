use macroquad::{text::measure_text, prelude::Color};
use qol::logy;

use crate::{Results, widgets::label_text::LabelText, WigetteType};


pub struct Wigette {
    pub (super) wigette_type: WigetteType,
    pub (super) desired_x: u32,
    pub (super) desired_y: u32,
    pub (super) x: i64,
    pub (super) y: i64,
    pub (super) size_y: u32,
    pub (super) size_x: u32,
    pub (super) expand_x: bool,
    pub (super) expand_y: bool,
}

impl Wigette {
    pub (super) fn get_expand_x(&self) -> bool {
        self.expand_x
    }
    pub (super) fn get_expand_y(&self) -> bool {
        self.expand_y
    }
    pub (super) fn get_min_x(&self) -> u32 {
        match &self.wigette_type {
            WigetteType::Box => {
                self.desired_x
            },
            WigetteType::HBox { children: _, distended_x, distended_y: _ } => {
                distended_x.max(&self.desired_x).clone()
            },
            WigetteType::Label(label) => {
                (measure_text(&label.text, None, label.font_size, 1.0).width as u32).max(self.desired_x.clone())
            },
            WigetteType::VBox { children: _, distended_x, distended_y:_ } => {
                distended_x.max(&self.desired_x).clone()
            }
        }
    }
    pub (super) fn get_min_y(&self) -> u32 {
        match &self.wigette_type {
            WigetteType::Box => {
                self.desired_y
            },
            WigetteType::HBox { children: _, distended_x: _, distended_y } => {
                distended_y.max(&self.desired_y).clone()
            },
            WigetteType::Label(label) => {
                (measure_text(&label.text, None, label.font_size, 1.0).height as u32).max(self.desired_y.clone())
            },
            WigetteType::VBox { children: _, distended_x: _, distended_y } => {
                distended_y.max(&self.desired_y).clone()
            }
        }
    }
    pub (super) fn set_size(&mut self, width: u32, height: u32) {
        let mut update = false;
        if width < self.desired_x {
            logy!("error", "tried to set width to less than min");
        } else {
            if self.size_x != width {
                self.size_x = width;
                update = true;
            }
        }

        if height < self.desired_y {
            logy!("error", "tried to set height to less than min");
        } else {
            if self.size_y != height {
                self.size_y = height;
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
    pub (super) fn get_height(&self) -> u32 {
        let basic_y =self.size_y.max(self.get_min_y());
        if let WigetteType::Label(label) = &self.wigette_type {
            (measure_text(&label.text, None, label.font_size, 1.0).height as u32).max(basic_y)
        } else {
            basic_y
        }
    }
    pub (super) fn get_width(&self) -> u32 {
        let basic_x=self.size_x.max(self.get_min_x());
        if let WigetteType::Label(label) = &self.wigette_type {
            (measure_text(&label.text, None, label.font_size, 1.0).width as u32).max(basic_x)
        } else {
            basic_x
        }
    }
}

// public functions
//creating functions
impl Wigette {
    pub fn new_label(width: u32, heigth: u32, expand_x: bool, expand_y: bool, text: String, font_size: u16, color:Color) -> Self {
        Wigette { 
            wigette_type: WigetteType::Label(LabelText::new(text, font_size, color)), 
            desired_x: width,
            size_x: width,
            desired_y: heigth,
            size_y: heigth,
            x: 0,
            y: 0,
            expand_x,
            expand_y,
         }
    }
    pub fn new_box(width: u32, heigth: u32, expand_x: bool, expand_y: bool) -> Self {
        Wigette {
            wigette_type: WigetteType::Box,
            desired_x: width,
            size_x: width,
            desired_y: heigth,
            size_y: heigth,
            x: 0,
            y: 0,
            expand_x,
            expand_y,
        }
    }
    pub fn new_hbox(
        width: u32,
        heigth: u32,
        expand_x: bool,
        expand_y: bool,
        children: Vec<Wigette>,
    ) -> Self {
        Wigette {
            wigette_type: WigetteType::HBox{children, distended_x: 0, distended_y: 0},
            desired_x: width,
            size_x: width,
            desired_y: heigth,
            size_y: heigth,
            x: 0,
            y: 0,
            expand_x,
            expand_y,
        }
    }
    pub fn new_vbox(
        width: u32,
        heigth: u32,
        expand_x: bool,
        expand_y: bool,
        children: Vec<Wigette>,
    ) -> Self {
        Wigette {
            wigette_type: WigetteType::VBox{children, distended_x: 0, distended_y: 0},
            desired_x: width,
            size_x: width,
            desired_y: heigth,
            size_y: heigth,
            x: 0,
            y: 0,
            expand_x,
            expand_y,
        }
    }
}

//misc public funtions
impl Wigette {
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
    pub fn get_child(&self, index: usize) -> Option<&Wigette> {
        match &self.wigette_type {
            WigetteType::HBox{children, distended_x: _, distended_y: _} => {
                children.get(index)
            }
            WigetteType::VBox{children, distended_x: _, distended_y: _} => {
                children.get(index)
            }
            _ => None
        }
    }
    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut Wigette> {
        match &mut self.wigette_type {
            WigetteType::HBox{children, distended_x: _, distended_y: _} => {
                children.get_mut(index)
            }
            WigetteType::VBox{children, distended_x: _, distended_y: _} => {
                children.get_mut(index)
            }
            _ => None
        }
    }
    pub fn external_facing_draw(&mut self) {
        self.update_size();
        self.update_childrens_pos(2);
        self.draw(0);
    }
}

//text_label
impl Wigette {
    pub fn set_text(&mut self, text: String) -> Results<()> {
        match &mut self.wigette_type {
            WigetteType::Label(inner) => {
                inner.text = text;
                Ok(())
            }
            _ => Err("tried to set the text on not a text label".into())
        }
    }
    pub fn set_text_color(&mut self, text: String, color: Color) -> Results<()> {
        match &mut self.wigette_type {
            WigetteType::Label(inner) => {
                inner.text = text;
                inner.color = color;
                Ok(())
            }
            _ => Err("tried to set the text on not a text label".into())
        }
    }
}