use druid::widget::Flex;
use druid::{Data, Widget};


/// Creates a flex row that can contain four items.
pub fn flex_row_4<T: Data>(
    item_1: impl Widget<T> + 'static,
    item_2: impl Widget<T> + 'static,
    item_3: impl Widget<T> + 'static,
    item_4: impl Widget<T> + 'static,
) -> impl Widget<T> {
    return Flex::row()
        .with_flex_child(item_1, 1.0)
        .with_spacer(1.0)
        .with_flex_child(item_2, 1.0)
        .with_spacer(1.0)
        .with_flex_child(item_3, 1.0)
        .with_spacer(1.0)
        .with_flex_child(item_4, 1.0)
        .with_spacer(1.0);
}