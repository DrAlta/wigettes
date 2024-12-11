use crate::{LayoutRet, Wigette};
impl Wigette {
    pub fn h_size(my_min_x: u32, my_min_y: u32 , children: &mut Vec<Wigette>, padding: u32) -> LayoutRet {
        let my_min_x = my_min_x as i64;
        let my_min_y = my_min_y as i64;
        let padding = padding as i64;

        let mut needed_x = padding * children.len() as i64;
        let mut expanding_children = Vec::<usize>::new();
        let mut expanding_x_count = 0_i64;

        let mut height = my_min_y;
        for (index, child) in children.iter_mut().enumerate() {
            child.update_size();
            needed_x += child.get_min_x() as i64;
            height = height.max(child.get_min_y() as i64 + (padding * 2));
            if child.get_expand_x() {
                expanding_x_count += 1;
            }
            if child.get_expand_x() || child.get_expand_y() {
                expanding_children.push(index);
            }
        }
        /*
        if needed_x >= my_min_x {
            return LayoutRet {
                height: height as u32,
                width: needed_x as u32,
            }
        }
        */
        // need to check is needed is larger than min then skip expanding chidren
        let foo_x = {
            if expanding_x_count != 0 {
                let diff_x =  my_min_x as i64 - needed_x as i64;
                let mut foo = diff_x / expanding_x_count;
                // round foo up
                if (diff_x % expanding_x_count) != 0 {
                    needed_x += expanding_x_count;
                    foo += 1;
                }
                // set the actual dim on this widget
                foo.max(0)
            } else {
                0
            }
        };
        let width = needed_x.max(my_min_x);

        for index in expanding_children {
            let child = children.get_mut(index).unwrap();

            let childs_min_x = child.get_min_x();

            let childs_min_y = child.get_min_y();
            let desired_width = if child.expand_x { 
                childs_min_x + (foo_x as u32)
            } else { 
                childs_min_x
            };
            let desired_heigth = if child.expand_y { 
                (height - (padding * 2)) as u32
            } else { 
                childs_min_y
            };
            child.set_size(desired_width, desired_heigth);
        }
        LayoutRet {
            height: height as u32,
            width: width as u32,
            distended_x: needed_x as u32,
            distended_y: height as u32,
        }
    }
}