use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc};
use druid::widget::{Button, Click, ControllerHost, Flex, Label, Slider, TextBox};

mod state;

static WINDOW_WIDTH: f64 = 1500.0;
static WINDOW_HEIGHT: f64 = 1500.0;
static INPUT_BOX_WIDTH: f64 = 1000.0;
static INPUT_BOX_HEIGHT: f64 = 400.0;

// TODO: improving layout, spacing, and padding in frontend
// TODO: make unit tests

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder).title("Rust Caesar Cipher").window_size((WINDOW_WIDTH, WINDOW_HEIGHT));

    let app_state: state::AppState = state::get_initial_state();

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(app_state)
}

fn ui_builder() -> impl Widget<state::AppState> {
    let title: Label<state::AppState> = Label::new("Rust Caesar Cipher").with_text_size(40.0);
    let first_row: Flex<state::AppState> = Flex::row().with_child(title).with_spacer(20.0);

    let choose_mode_title_label: Label<state::AppState> = Label::new("Mode:").with_text_size(20.0);
    let change_mode_button: ControllerHost<Button<state::AppState>, Click<state::AppState>> = Button::new("Change").on_click(|_ctx, state: &mut state::AppState, _env| {
        state.set_encrypting(!state.get_encrypting())
    });
    let choose_mode_value_label: Label<state::AppState> = Label::new(|state: &state::AppState, _env: &_| format!("{}", if state.get_encrypting() { "Encrypt" } else { "Decrypt" })).with_text_size(15.0);
    let second_row: Flex<state::AppState> = Flex::row().with_child(choose_mode_title_label).with_child(choose_mode_value_label).with_child(change_mode_button).with_spacer(20.0);

    let shift_mode_title_label: Label<state::AppState> = Label::new("Shift mode:").with_text_size(20.0);
    let change_shift_mode_button: ControllerHost<Button<state::AppState>, Click<state::AppState>> = Button::new("Change").on_click(|_ctx, state: &mut state::AppState, _env| {
        state.set_shift_size_automatic(!state.get_shift_size_automatic())
    });
    let shift_mode_value_label: Label<state::AppState> = Label::new(|state: &state::AppState, _env: &_| format!("{}", if state.get_shift_size_automatic() { "Automatic" } else { "Manual" })).with_text_size(15.0);
    let third_row: Flex<state::AppState> = Flex::row().with_child(shift_mode_title_label).with_child(change_shift_mode_button).with_child(shift_mode_value_label).with_spacer(20.0);

    let shift_size_title_label: Label<state::AppState> = Label::new("Shift size:").with_text_size(20.0);
    let shift_size_slider = Slider::new().with_range(1.0, 26.0).lens(state::AppState::shift_size);
    let shift_size_value_label: Label<state::AppState> = Label::new(|state: &state::AppState, _env: &_| format!("{}", state.get_shift_size() as i64)).with_text_size(15.0);
    let fourth_row: Flex<state::AppState> = Flex::row().with_child(shift_size_title_label).with_child(shift_size_slider).with_child(shift_size_value_label).with_spacer(20.0);

    let input_title_label: Label<state::AppState> = Label::new("Input:").with_text_size(20.0);
    let input_value_text_box = TextBox::new()
        .with_placeholder("Enter input here.")
        .fix_width(INPUT_BOX_WIDTH)
        .fix_height(INPUT_BOX_HEIGHT)
        .lens(state::AppState::input);
    let fifth_row: Flex<state::AppState> = Flex::row().with_child(input_title_label).with_child(input_value_text_box).with_spacer(20.0);

    let submit_button = Button::new("Submit")
        .on_click(|_ctx, state: &mut state::AppState, _env| {
            state.update_output();
        });
    let sixth_row: Flex<state::AppState> = Flex::row().with_child(submit_button).with_spacer(20.0);

    let output_title_label: Label<state::AppState> = Label::new("Output:").with_text_size(20.0);
    let output_value_label: Label<state::AppState> = Label::new(|state: &state::AppState, _env: &_| format!("{}", state.get_output())).with_text_size(15.0);
    let seventh_row: Flex<state::AppState> = Flex::row().with_child(output_title_label).with_child(output_value_label).with_spacer(20.0);

    let col: Flex<state::AppState> = Flex::column().with_child(first_row).with_child(second_row).with_child(third_row).with_child(fourth_row).with_child(fifth_row).with_child(sixth_row).with_child(seventh_row).with_flex_spacer(40.0);
    col
}