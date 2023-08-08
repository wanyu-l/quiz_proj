use druid::{
    widget::{ Align, Button, Flex, Label, Scroll, TextBox, Controller },
    AppLauncher,
    Color,
    Data,
    Lens,
    Widget,
    WidgetExt,
    WindowDesc,
    EventCtx,
    Event,
    Env,
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
    display_word_remarks: Vec<Vec<String>>,
    res: Vec<Vec<String>>,
    curr_indexes: Vec<usize>,
    str: String,
}

impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        if !is_same_2_d_vec_string(self.input_str.clone(), other.input_str.clone()) {
            return false;
        }
        if !is_same_2_d_vec_string(self.expected_input.clone(), other.expected_input.clone()) {
            return false;
        }
        if !is_same_2_d_vec_string(self.display_word.clone(), other.display_word.clone()) {
            return false;
        }
        if
            !is_same_2_d_vec_string(
                self.display_word_remarks.clone(),
                other.display_word_remarks.clone()
            )
        {
            return false;
        }
        if !is_same_2_d_vec_string(self.res.clone(), other.res.clone()) {
            return false;
        }
        if self.curr_indexes.len() != other.curr_indexes.len() {
            return false;
        }
        for i in 0..self.curr_indexes.len() {
            if self.curr_indexes[i] != other.curr_indexes[i] {
                return false;
            }
        }
        if self.str != other.str {
            return false;
        }
        return true;
    }
}

fn is_same_vec_string(arr: Vec<String>, brr: Vec<String>) -> bool {
    if arr.len() != brr.len() {
        return false;
    }
    for i in 0..arr.len() {
        if arr[i] != brr[i] {
            return false;
        }
    }
    true
}

fn is_same_2_d_vec_string(arr: Vec<Vec<String>>, brr: Vec<Vec<String>>) -> bool {
    if arr.len() != brr.len() {
        return false;
    }
    for i in 0..arr.len() {
        if !is_same_vec_string(arr[i].clone(), brr[i].clone()) {
            return false;
        }
    }
    true
}

impl AppState {
    fn default() -> AppState {
        let storage_unit = Storage::new();
        let all_sets = storage_unit.get_all();
        let mut display: Vec<Vec<String>> = Vec::new();
        let mut remarks: Vec<Vec<String>> = Vec::new();
        let mut expected: Vec<Vec<String>> = Vec::new();
        let mut input_all: Vec<Vec<String>> = Vec::new();
        let mut res_all: Vec<Vec<String>> = Vec::new();
        let mut indexes = Vec::new();
        for set in all_sets {
            let cards = set.get_all_cards();
            let mut card_set_answers = Vec::new();
            let mut card_set_displays = Vec::new();
            let mut card_set_word_remarks = Vec::new();
            let mut card_set_inputs = Vec::new();
            let mut card_set_res = Vec::new();
            for card in cards {
                card_set_answers.push(card.get_ans());
                card_set_displays.push(card.get_word());
                card_set_word_remarks.push(card.get_remarks());
                card_set_inputs.push("".to_string());
                card_set_res.push("".to_string());
            }
            expected.push(card_set_answers);
            display.push(card_set_displays);
            remarks.push(card_set_word_remarks);
            input_all.push(card_set_inputs);
            res_all.push(card_set_res);
            indexes.push(0);
        }
        AppState {
            input_str: input_all,
            expected_input: expected,
            display_word: display,
            display_word_remarks: remarks,
            res: res_all,
            curr_indexes: indexes,
            str: String::new(),
        }
    }
}

// index is the id of the study set
fn test_page_builder(id: usize, test_name: String) -> impl Widget<AppState> {
    let index = id - 1;
    let word_label = Label::dynamic(move |data: &AppState, _env| -> String {
        let word_index = data.curr_indexes[index];
        data.display_word[index][word_index].to_string()
    }).with_text_size(32.0);
    let remarks_label = Label::dynamic(move |data: &AppState, _env| -> String {
        let remark_index = data.curr_indexes[index];
        data.display_word_remarks[index][remark_index].to_string()
    }).with_text_size(32.0);

    let text_box = TextBox::new()
        .with_placeholder("Enter text here")
        .with_text_size(24.0)
        .fix_width(300.0)
        .lens(AppState::str);

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
            let ind = data.curr_indexes[index];
            data.input_str[index][ind] = data.str.clone();
            if data.curr_indexes[index] >= 1 {
                data.curr_indexes[index] -= 1;
                data.str.clear();
            }
            ctx.request_update();
        }
    );
    let next = Button::new("Next").on_click(
        move |ctx, data: &mut AppState, _env| -> () {
            let ind = data.curr_indexes[index];
            data.input_str[index][ind] = data.str.clone();
            if data.curr_indexes[index] < data.display_word[index].len() - 1 {
                data.curr_indexes[index] += 1;
                data.str.clear();
            }
            ctx.request_update();
        }
    );

    let eval_results = Button::new("Submit Test").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            let ind = data.curr_indexes[index];
            data.input_str[index][ind] = data.str.clone();
            let results_window = WindowDesc::new(
                result_page_builder(
                    test_name.clone(),
                    data.input_str[index].clone(),
                    data.expected_input[index].clone(),
                    data.display_word[index].clone()
                )
            )
                .title("Resuts")
                .set_always_on_top(true);
            ctx.new_window(results_window);
        }
    );

    let res_label = Label::dynamic(move |data: &AppState, _| {
        let word_index = data.curr_indexes[index];
        data.res[index][word_index].clone()
    }).with_text_size(24.0);
    let index_label = Label::dynamic(move |data: &AppState, _|
        format!("{} / {}\n", data.curr_indexes[index] + 1, data.display_word[index].len())
    ).with_text_size(24.0);

    let inputs = Flex::row().with_child(prev).with_child(clear).with_child(next);

    let card = Flex::column()
        .with_child(index_label)
        .with_spacer(20.0)
        .with_child(word_label)
        .with_spacer(20.0)
        .with_child(remarks_label);
    let card = card
        .with_child(text_box)
        .with_spacer(20.0)
        .with_child(inputs)
        .with_spacer(20.0)
        .with_child(res_label)
        .with_spacer(20.0)
        .with_child(eval_results);
    card
}

// index is the id of the study set
fn learn_page_builder(id: usize, test_name: String) -> impl Widget<AppState> {
    let index = id - 1;
    let word_label = Label::dynamic(move |data: &AppState, _env| -> String {
        let word_index = data.curr_indexes[index];
        data.display_word[index][word_index].to_string()
    }).with_text_size(32.0);
    let remarks_label = Label::dynamic(move |data: &AppState, _env| -> String {
        let remark_index = data.curr_indexes[index];
        data.display_word_remarks[index][remark_index].to_string()
    }).with_text_size(32.0);
    let text_box = TextBox::new()
        .with_placeholder("Enter text here")
        .with_text_size(24.0)
        .fix_width(300.0)
        .lens(AppState::str);
    let enter = Button::new("Confirm").on_click(
        move |ctx, data: &mut AppState, _env| -> () {
            let word_index = data.curr_indexes[index];
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
            let ind = data.curr_indexes[index];
            data.input_str[index][ind] = data.str.clone();
            if data.curr_indexes[index] >= 1 {
                data.curr_indexes[index] -= 1;
                data.str.clear();
            }
            ctx.request_update();
        }
    );
    let next = Button::new("Next").on_click(
        move |ctx, data: &mut AppState, _env| -> () {
            let ind = data.curr_indexes[index];
            data.input_str[index][ind] = data.str.clone();
            if data.curr_indexes[index] < data.display_word[index].len() - 1 {
                data.curr_indexes[index] += 1;
                data.str.clear();
            }
            ctx.request_update();
        }
    );

    let eval_results = Button::new("Submit Test").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            let ind = data.curr_indexes[index];
            data.input_str[index][ind] = data.str.clone();
            let results_window = WindowDesc::new(
                result_page_builder(
                    test_name.clone(),
                    data.input_str[index].clone(),
                    data.expected_input[index].clone(),
                    data.display_word[index].clone()
                )
            )
                .title("Resuts")
                .set_always_on_top(true);
            ctx.new_window(results_window);
        }
    );

    let res_label = Label::dynamic(move |data: &AppState, _| {
        let word_index = data.curr_indexes[index];
        data.res[index][word_index].clone()
    }).with_text_size(24.0);

    let index_label = Label::dynamic(move |data: &AppState, _|
        format!("{} / {}\n", data.curr_indexes[index] + 1, data.display_word[index].len())
    ).with_text_size(24.0);

    let inputs = Flex::row().with_child(prev).with_child(enter).with_child(clear).with_child(next);

    let card = Flex::column()
        .with_child(index_label)
        .with_child(word_label)
        .with_spacer(10.0)
        .with_child(remarks_label)
        .with_spacer(20.0);
    let card = card
        .with_child(text_box)
        .with_spacer(20.0)
        .with_child(inputs)
        .with_spacer(20.0)
        .with_child(res_label)
        .with_spacer(10.0)
        .with_child(eval_results);
    card
}

fn result_page_builder(
    test_name: String,
    user_answers: Vec<String>,
    expected_answers: Vec<String>,
    display_words: Vec<String>
) -> impl Widget<AppState> {
    let lesson_label: Align<AppState> = Label::new(test_name.clone())
        .with_text_size(32.0)
        .with_text_color(Color::TEAL)
        .center();
    let mut list: Flex<AppState> = Flex::column().with_child(lesson_label);
    for i in 0..display_words.len() {
        let word = format!("Word:\n[{}]", display_words[i]);
        let word_label: Label<AppState> = Label::new(word)
            .with_text_size(24.0)
            .with_text_color(Color::FUCHSIA);
        let mut word_row: Flex<AppState> = Flex::row().with_child(word_label.padding(25.0));
        let user_ans = format!("Your Answer:\n[{}]", user_answers[i]);
        if user_answers[i] == expected_answers[i] {
            let correct_label: Label<AppState> = Label::new(user_ans)
                .with_text_size(24.0)
                .with_text_color(Color::LIME);
            word_row = word_row.with_child(correct_label.padding(25.0));
        } else {
            let wrong_label: Label<AppState> = Label::new(user_ans)
                .with_text_size(24.0)
                .with_text_color(Color::MAROON);
            word_row = word_row.with_child(wrong_label.padding(25.0));
        }
        let expected_ans = format!("Correct Answer:\n[{}]", expected_answers[i]);
        let answer_label: Label<AppState> = Label::new(expected_ans)
            .with_text_size(24.0)
            .with_text_color(Color::SILVER);
        word_row = word_row.with_child(answer_label);
        list = list.with_child(word_row);
    }
    let scroll = Scroll::new(list.padding(20.0)).vertical();
    scroll
}

fn start_page_builder(storage: Storage) -> impl Widget<AppState> {
    let study_sets = storage.get_all();
    let mut list = Flex::column();
    for set in study_sets {
        let set_cloned = set.clone();
        let mut section = Flex::column();
        let label = Label::new(set.get_desc()).with_text_size(24.0).center();
        let edit_button = Button::new("Edit"); //todo: functionality to edit items
        let view_button = Button::new("View"); //todo: functionality to view all items
        let learn_button = Button::new("Learn").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, _data: &mut AppState, _env| {
                let new_win = WindowDesc::new(
                    learn_page_builder(set_cloned.get_id(), set_cloned.get_desc())
                ).title(set_cloned.get_desc());
                ctx.new_window(new_win);
                ctx.request_update();
            }
        ); //.controller(NewWindowController);
        let test_button = Button::new("Test").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, _data: &mut AppState, _env| {
                let new_win = WindowDesc::new(
                    test_page_builder(set.get_id(), set.get_desc())
                ).title(set.get_desc());
                ctx.new_window(new_win);
            }
        );
        section.add_child(label.padding(5.0));
        let mut row = Flex::row();
        row.add_child(view_button);
        row.add_child(edit_button);
        row.add_child(learn_button);
        row.add_child(test_button);
        section.add_child(row);
        list.add_child(section.center());
    }
    let scroll = Scroll::new(list).vertical();
    let aligned_widget = Align::right(scroll);
    aligned_widget
}

// struct NewWindowController;

// impl<W: Widget<AppState>> Controller<AppState, W> for NewWindowController {
//     fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
//         if let Event::WindowCloseRequested = event {
//             println!("New window is closing");
//         }
//         ctx.request_update();
//         child.event(ctx, event, data, env);
//         // todo: save data
//     }
// }

pub fn main() {
    // Window Descriptor
    // Launch to the stars
    let storage_unit = storage::Storage::new();
    // storage_unit.save();
    let main_window = WindowDesc::new(start_page_builder(storage_unit)).title("Quiz_Late");
    AppLauncher::with_window(main_window).log_to_console().launch(AppState::default()).unwrap();
}
