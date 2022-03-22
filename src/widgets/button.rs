use super::super::State;

use druid::widget::{Widget, Label};
use druid::{Color, WidgetExt};


pub fn button<T: core::fmt::Display>(value: T, clr: Color) -> impl Widget<State> {
    return Label::new(format!("{}", value))
        .with_text_size(24.0)
        .background(clr)
        .expand()
        .center();
}


pub fn digit_button(digit: u8) -> impl Widget<State> {
    return button(digit, Color::rgb8(64, 64, 64))
        .on_click(move |_, data: &mut State, _|
            data.value.push(digit as char)
        );
}


// TODO: Create Function in `State` to handle operator changes.
// pub fn operator_button(operator: String) -> impl Widget<State> {
//     return button(operator, Color::rgb8(64, 248, 64))
//         .on_click(move |_, data: &mut State, _|
//             data.operator = operator
//         );
// }