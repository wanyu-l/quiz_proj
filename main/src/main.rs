use druid::{
    widget::{Align, BackgroundBrush, Button, Flex, Label, Scroll, SizedBox, TextBox},
    AppLauncher, Color, Command, Data, Env, Lens, Widget, WidgetExt, WindowDesc, UnitPoint,
};
use storage::Storage;

mod popup;
mod storage;
/*
 * Data
 * Ui Builder
 * Main
 */

#[derive(Clone, Data, Lens)]
struct AppState {
    input: String,
    expected_input: String,
    display_word: String,
    show_popup: bool,
    res: String,
}

impl AppState {}

fn check_answer(answer: String, input: String) -> bool {
    answer == input
}

fn ui_builder(val: String) -> impl Widget<AppState> {
    let word_label = Label::new(format!("{}\n", val.clone())).with_text_size(32.0);

    let text_box = TextBox::new()
        .with_placeholder("Enter text here")
        .fix_width(150.0)
        .lens(AppState::input);

    let enter = Button::new("Confirm").on_click(|ctx, data: &mut AppState, _env| -> () {
        if check_answer(data.input.clone(), data.expected_input.clone()) {
            data.res = String::from("Correct!");
        } else {
            data.res = String::from("Try Again!");
        }
        ctx.request_update();
    });

    let clear = Button::new("Clear").on_click(|ctx, data: &mut AppState, _env| -> () {
        let message = String::from("Input Cleared");
        data.input.clear();
        data.res = message;
        ctx.request_update();
    });

    let res_label = Label::dynamic(|data: &AppState, _| data.res.clone()).with_text_size(24.0);

    let inputs = Flex::row().with_child(enter).with_child(clear);

    let temp = Flex::column().with_child(word_label);
    let temp = temp
        .with_child(text_box)
        .with_spacer(20.0)
        .with_child(inputs)
        .with_child(res_label);
    temp
}

fn start_page_builder(storage: Storage) -> impl Widget<()> {
    let study_sets = storage.get_all();
    let mut list = Flex::column();
    // for set in study_sets {
    (1..55).for_each(|i| {
        let mut section = Flex::column();
        // let label = Label::new(set.get_desc());
        let label = Label::new(format!("Item {}", i)).with_text_size(24.0).center();
        let edit_button = Button::new("Edit");//.on_click(f);
        let view_button = Button::new("View");//align_right();
        section.add_child(label.padding(5.0));
        let mut row = Flex::row();
        row.add_child(view_button);
        row.add_child(edit_button);
        section.add_child(row);
        list.add_child(section.center());
    });
    let scroll = Scroll::new(list).vertical();
    let aligned_widget = Align::right(scroll);
    aligned_widget
}

fn main() {
    // Window Descriptor
    // Launch to the stars
    // let storage = storage::Storage::new();
    // let set = storage.get_study_set(String::from("L1"));
    // let card = set.get_card();
    // let word = card.get_word();
    // let ans = card.get_ans();
    // let main_window = WindowDesc::new(ui_builder(word.clone())).title("Quiz_Late");
    // AppLauncher::with_window(main_window)
    //     .log_to_console()
    //     .launch(AppState {
    //         input: String::from(""),
    //         display_word: word,
    //         expected_input: ans,
    //         show_popup: false,
    //         res: String::from(""),
    //     })
    //     .unwrap();
    let storage = Storage::new();
    let main_window = WindowDesc::new(start_page_builder(storage)).title("Quiz_Late");
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(())
        .unwrap();
}
