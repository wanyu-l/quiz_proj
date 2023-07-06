use druid::{
    widget::{ Align, BackgroundBrush, Button, Flex, Label, Scroll, SizedBox, TextBox },
    AppLauncher,
    Color,
    Command,
    Data,
    Env,
    Lens,
    Widget,
    WidgetExt,
    WindowDesc,
    UnitPoint,
};
use storage::Storage;

mod storage;
/*
 * Data
 * Ui Builder
 * Main
 */

#[derive(Clone, Lens, Debug)]
struct AppState {
    input_str: Vec<Vec<String>>,
    expected_input: Vec<Vec<String>>,
    display_word: Vec<Vec<String>>,
    res: Vec<Vec<String>>,
    curr_indexes: Vec<usize>,
    // storage: Storage,
    str: String,
    is_sub_open: bool,
    label_text: i32,
}

impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        if self.input_str.len() != other.input_str.len() {
            return false;
        }
        if self.expected_input.len() != other.expected_input.len() {
            return false;
        }
        if self.display_word.len() != other.display_word.len() {
            return false;
        }
        if self.res.len() != other.res.len() {
            return false;
        }
        if self.curr_indexes.len() != other.curr_indexes.len() {
            return false;
        }

        return true;
    }
}

impl AppState {
    fn default() -> AppState {
        let storage_unit = Storage::new();
        let all_sets = storage_unit.get_all();
        let mut display: Vec<Vec<String>> = Vec::new();
        let mut expected: Vec<Vec<String>> = Vec::new();
        let mut input_all: Vec<Vec<String>> = Vec::new();
        let mut res_all: Vec<Vec<String>> = Vec::new();
        let mut indexes = Vec::new();
        for set in all_sets {
            let cards = set.get_all_cards();
            let mut card_set_answers = Vec::new();
            let mut card_set_displays = Vec::new();
            let mut card_set_inputs = Vec::new();
            let mut card_set_res = Vec::new();
            for card in cards {
                card_set_answers.push(card.get_ans());
                card_set_displays.push(card.get_word());
                card_set_inputs.push("".to_string());
                card_set_res.push("".to_string());
            }
            expected.push(card_set_answers);
            display.push(card_set_displays);
            input_all.push(card_set_inputs);
            res_all.push(card_set_res);
            indexes.push(0);
        }
        AppState {
            input_str: input_all,
            expected_input: expected,
            display_word: display,
            res: res_all,
            curr_indexes: indexes,
            str: String::new(),
            is_sub_open: false,
            label_text: 1,
        }
    }
}

fn test() -> impl Widget<AppState> {
    let res_label = Label::dynamic(|data: &AppState, _| {
        data.label_text.to_string()
    }).with_text_size(24.0);
    let test = Button::new("test").on_click(
        |ctx, data: &mut AppState, _env| {
            data.label_text += 1;
            ctx.request_update();
        }
    );
    Flex::column().with_child(res_label).with_child(test)
}

// index is the id of the study set
fn ui_builder(id: usize) -> impl Widget<AppState> {
    let index = id - 1;
    let word_label = Label::dynamic(move |data: &AppState, _env| -> String {
        let word_index = data.curr_indexes[index];
        println!("{:#?}", data.display_word[index][word_index]);
        data.display_word[index][word_index].to_string()
    }).with_text_size(32.0);

    let text_box = TextBox::new()
        .with_placeholder("Enter text here")
        .fix_width(150.0)
        .lens(AppState::str);
    let enter = Button::new("Confirm").on_click(
        move |ctx, data: &mut AppState, _env| -> () {
            let word_index = data.curr_indexes[index];
            println!("{}", data.expected_input[index][word_index]);
            if data.str == data.expected_input[index][word_index] {
                data.res[index][word_index] = String::from("Correct!");
            } else {
                data.res[index][word_index] = String::from("Try Again!");
            }
            ctx.request_update();
        }
    );

    let clear = Button::new("Clear").on_click(
        move |ctx, data: &mut AppState, _env| -> () {
            let message = String::from("Input Cleared");
            data.str.clear();
            let word_index = data.curr_indexes[index];
            data.res[index][word_index] = message;
            ctx.request_update();
        }
    );
    let prev = Button::new("Prev").on_click(
        move |ctx, data: &mut AppState, _env| -> () {
            println!("{:#?}", data.curr_indexes[index]);
            if data.curr_indexes[index] >= 1 {
                data.curr_indexes[index] -= 1;
            }
            ctx.request_update();
        }
    );
    let next = Button::new("Next").on_click(
        move |ctx, data: &mut AppState, _env| -> () {
            println!("{:#?}", data.curr_indexes[index]);
            if data.curr_indexes[index] < data.display_word[index].len() - 1 {
                data.curr_indexes[index] += 1;
            }
            ctx.request_update();
        }
    );

    let res_label = Label::dynamic(move |data: &AppState, _| {
        let word_index = data.curr_indexes[index];
        data.res[index][word_index].clone()
    }).with_text_size(24.0);
    let index_label = Label::dynamic(move |data: &AppState, _|
        (data.curr_indexes[index] + 1).to_string()
    ).with_text_size(24.0);

    let inputs = Flex::row().with_child(prev).with_child(enter).with_child(clear).with_child(next);
    // let inputs = inputs.with_child(test);

    let temp = Flex::column().with_child(word_label);
    let temp = temp
        .with_child(text_box)
        .with_spacer(20.0)
        .with_child(inputs)
        .with_child(res_label)
        .with_child(index_label);
    temp
}

fn start_page_builder(storage: Storage) -> impl Widget<AppState> {
    let study_sets = storage.get_all();
    let mut list = Flex::column();
    
    let res_label = Label::dynamic(|data: &AppState, _| {
        data.label_text.to_string()
    }).with_text_size(24.0);
    let test_button = Button::new("test").on_click(
        |ctx, data: &mut AppState, _env| {
            data.label_text += 1;
            ctx.request_update();
        }
    );
    for set in study_sets {
        let mut section = Flex::column();
        let label = Label::new(format!("Item {}", set.get_desc())).with_text_size(24.0).center();
        let edit_button = Button::new("Edit"); //todo: functionality to edit items
        let view_button = Button::new("View"); //todo: functionality to view all items
        let test_button = Button::new("Test").on_click(|ctx: &mut druid::EventCtx<'_, '_>, _data: &mut AppState, _env| {
            // if !data.is_sub_open {
            // data.is_sub_open = true;
            // let new_win = WindowDesc::new(ui_builder(set.get_id()));
            let new_win = WindowDesc::new(test());
            ctx.new_window(new_win);
            // }
        });
        section.add_child(label.padding(5.0));
        let mut row = Flex::row();
        row.add_child(view_button);
        row.add_child(edit_button);
        row.add_child(test_button);
        section.add_child(row);
        list.add_child(section.center());
    }
    list.add_child(res_label);
    list.add_child(test_button);

    let scroll = Scroll::new(list).vertical();
    let aligned_widget = Align::right(scroll);
    aligned_widget
}

pub fn main() {
    // Window Descriptor
    // Launch to the stars
    let storage = storage::Storage::new();
    let main_window = WindowDesc::new(start_page_builder(storage)).title("Quiz_Late");
    AppLauncher::with_window(main_window).log_to_console().launch(AppState::default()).unwrap();
}
