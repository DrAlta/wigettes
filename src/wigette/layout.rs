use crate::{LayoutRet, Wigette, WigetteType};

impl Wigette {
    pub fn update_childrens_pos(&mut self, padding: i64 ) {
        let mut turtle = 0_i64;
        let x = self.x + (padding / 2);
        let y = self.y + (padding / 2);
        let half_p_width = self.get_width() as i64 / 2;
        let half_p_height = self.get_height() as i64 / 2;
        match &mut self.wigette_type {
            WigetteType::HBox{children, distended_x: _, distended_y: _} => {
                for (index, child) in children.into_iter().enumerate() {
                    let c_height = child.get_height() as i64;
                    let c_width = child.get_width() as i64;
                    let my_padding = index as i64 * padding;
                    child.set_pos(
                        x + turtle + (index as i64 * padding),
                        (y + half_p_height) - ((padding + c_height) / 2),
                    );
                    turtle += c_width + my_padding;
                }
            }
            WigetteType::VBox{children, distended_x: _, distended_y: _} => {
                for (index, child) in children.into_iter().enumerate() {
                    let c_height = child.get_height() as i64;
                    let c_width = child.get_width() as i64;
                    let my_padding = index as i64 * padding;
                    child.set_pos((x + half_p_width) - ((padding + c_width) / 2), y + turtle + my_padding);
                    turtle += c_height + my_padding;
                }
            }
            WigetteType::Box => {}
            WigetteType::Label(_) => {}
        }
    }
    pub fn update_size(&mut self) {
        let my_min_x = self.get_min_x();
        let my_min_y = self.get_min_y();
        match &mut self.wigette_type {
            WigetteType::HBox{children, distended_x: _, distended_y: _} => {
                let LayoutRet {
                    height: y,
                    width: x,
                    distended_x: _,
                    distended_y: _
                } = Self::h_size(
                    my_min_x, my_min_y,
                    children,
                    2
                    //&Self::get_width,
                    //&Self::get_min_x,
                    //&Self::set_size_x,
                    //&Self::get_expand_x
                );
                self.set_size(x, y);
            }
            WigetteType::VBox{children, distended_x: _, distended_y: _} => {
                let LayoutRet {
                    height: y,
                    width: x,
                    distended_x: _,
                    distended_y: _
                } = Self::v_size(
                    my_min_x as usize, my_min_y as usize,
                    children,
                    2
                    //&Self::get_height,
                    //&Self::get_min_y,
                    //&Self::set_size_y,
                    //&Self::get_expand_y
                );
                self.set_size(x, y);
            }
            _ => {}
        }
    }
}// public functions