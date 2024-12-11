pub mod widgets;
mod layout_rec;
pub use layout_rec::LayoutRet;
mod wigette_type;
pub use wigette_type::WigetteType;
mod wigette;
pub use wigette::Wigette;

type Results<T> = Result<T, String>;