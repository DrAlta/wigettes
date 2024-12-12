mod layout_rec;
pub mod widgets;
pub use layout_rec::LayoutRet;
mod widget;
pub use widget::Widget;
mod wigette_type;
pub use wigette_type::WigetteType;
mod wigette;
pub use wigette::Wigette;

type Results<T> = Result<T, String>;

#[cfg(test)]
mod tests {
    use macroquad::prelude::*;
    use super::*;

    #[async_std::test]
    async fn test_simple_case() {
        let font = load_ttf_font("./FreeSans.ttf").await.unwrap();

        let mut a = Wigette::new_vbox(
            10, 
            10, 
            true, 
            true,
            vec![
                Wigette::new_label(
                    5, 
                    5, 
                    true, 
                    true, 
                    "text".into(), 
                    &font, 
                    10, 
                    BLANK
                )
            ]
        );
        let _ = a.get_child_mut(0);
        let _ = a.get_child_mut(0).unwrap();

    }
}