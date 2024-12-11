use crate::{LayoutRet, Wigette};
impl Wigette {
    pub fn v_size(my_min_x: usize, my_min_y: usize, children: &mut Vec<Wigette>, padding: i64) -> LayoutRet {
        let mut needed_y = padding as usize * children.len();
        let mut expanding_children = Vec::<usize>::new();
        let mut expanding_y_count = 0_usize;

        let mut width = my_min_x;
        for (index, child) in children.iter_mut().enumerate() {
            child.update_size();
            needed_y += child.get_height() as usize + (padding * 2) as usize;
            width = width.max(child.get_width() as usize);
            if child.get_expand_y() {
                expanding_y_count += 1;
            }
            if child.get_expand_x() || child.get_expand_y() {
                expanding_children.push(index);
            }
        }

        let (height, foo_y) = {
            if expanding_y_count != 0 {
                let diff_y =  my_min_y - needed_y;
                let mut foo = diff_y / expanding_y_count;
                // round foo up
                if (diff_y % expanding_y_count) != 0 {
                    needed_y += expanding_y_count;
                    foo += 1;
                }
                //set the actual dim on this widget
                (needed_y.max(my_min_y), foo)
            } else {
                (my_min_y, 0)
            }
        };

        for index in expanding_children {
            let child = children.get_mut(index).unwrap();

            let childs_min_x = child.get_min_x();

            let childs_min_y = child.get_min_y();
            let desired_width = if child.get_expand_x() {
                (width - (padding * 2) as usize) as u32
            } else {
                childs_min_x
            };
            let desired_height = if child.get_expand_x() {
                childs_min_y + (foo_y as u32)
            } else {
                childs_min_y
            };
            child.set_size(desired_width, desired_height);
        }
        LayoutRet {
            height: height as u32,
            width: width as u32,
            distended_x: width as u32,
            distended_y: needed_y as u32
        }
    }
}
