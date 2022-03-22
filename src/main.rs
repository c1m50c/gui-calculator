use druid::{AppLauncher, Widget, WindowDesc, PlatformError, Data, WidgetExt, Env, Lens};
use druid::widget::{Button, Flex, Label, CrossAxisAlignment};

mod widgets;


#[derive(Clone, Data, Lens)]
pub struct State {
    /// Displayed result, current output value.
    value: String,
    
    /// Number to be used with the `operator` field to apply the `operator` to the current `value`.
    number: f64,

    /// Operator to be used with the `number` field to calculate the new `value`.
    operator: String,
}


fn build_ui() -> impl Widget<State> {
    let value_label = Label::new(|data: &String, _: &Env| data.clone())
        .with_text_size(24.0)
        .lens(State::value)
        .padding(0.5);
    
    return Flex::column()
        .with_flex_spacer(0.25)
        .with_child(value_label)
        .with_flex_spacer(0.25)
        .cross_axis_alignment(CrossAxisAlignment::End);
}


fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(build_ui)
        .window_size((400.0, 600.0))
        .title("GUI Calculator");
    
    let state = State {
        value: "0".to_string(),
        number: 0.0,
        operator: "Clear".to_string(),
    };

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(state)
}
