// with the layout system needs to keep track of which chilren have responed to 
// the layout_requests and collect them all before retrying the to layout the parent
//  is the parent assumes that it is geiven all the responses it requested last time it was 
// asked to layout it self.

pub mod foo;
mod layout_response;
pub use layout_response::LayoutResponse;
mod rect;
pub use rect::Rect;
mod layout;
pub use layout::Layout;
mod resolution;
pub use resolution::Resolution;
mod widget;
pub use widget::Widget;