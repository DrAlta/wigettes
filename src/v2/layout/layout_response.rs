use std::collections::HashMap;

use crate::v2::layout::{Layout, Rect};

pub enum LayoutResponse<InterSpaxel, Spaxel, ChildId: Eq + std::hash::Hash> {
    Layout{
        rect:Rect<i32>, 
        childrens_layouts: HashMap<ChildId, Layout<InterSpaxel, Spaxel, ChildId>>,
    },
    RequestLayoutOfChildren{
        callback: Layout<InterSpaxel, Spaxel, ChildId>,
        children_to_layout: HashMap<ChildId, Rect<Spaxel>>,
    },
}