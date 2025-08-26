use std::collections::HashMap;
use super::Rect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout<InterSpaxel, Spaxel, ChildId: Eq+ PartialEq+ std::hash::Hash> {
    pub left: InterSpaxel,
    pub top: InterSpaxel,
    pub area: Rect<Spaxel>,
    pub children: HashMap<ChildId, Self>,
}

