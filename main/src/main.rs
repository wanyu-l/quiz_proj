use druid::{
    widget::{ Align, Button, Flex, Label, Scroll, TextBox },
    AppLauncher,
    Color,
    Data,
    Lens,
    Widget,
    WidgetExt,
    WindowDesc,
};
use storage::{ Storage, Card, StudySet };

const MAIN_TITLE: &str = "Quiz Late";

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
    storage_unit: Storage,
    // for adding a word to studyset
    word_to_add: String,
    word_ans_to_add: String,
    word_remark_to_add: String,
    // for adding a studyset
    new_set_name: String,
}

fn is_valid(input_str: String) -> bool {
    println!("{}", input_str.replace(" ", "") == "");
    input_str.replace(" ", "") != ""
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
    fn default(storage: Storage) -> AppState {
        let all_sets = storage.get_all();
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
            storage_unit: storage,
            word_to_add: String::new(),
            word_ans_to_add: String::new(),
            word_remark_to_add: String::new(),
            new_set_name: String::new(),
        }
    }
}

fn test_page_builder(index: usize, test_name: String) -> impl Widget<AppState> {
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
            ctx.window().close();
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

    let return_to_main = Button::new("Return to Study Sets List").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            let new_win = WindowDesc::new(start_page_builder(data.storage_unit.clone())).title(
                MAIN_TITLE
            );
            ctx.new_window(new_win);
            ctx.window().close();
        }
    );

    let card = Flex::column()
        .with_child(index_label)
        .with_spacer(20.0)
        .with_child(word_label)
        .with_spacer(20.0)
        .with_child(remarks_label)
        .with_spacer(30.0);
    let card = card
        .with_child(text_box)
        .with_spacer(20.0)
        .with_child(inputs)
        .with_spacer(20.0)
        .with_child(res_label)
        .with_spacer(20.0)
        .with_child(eval_results);
    card.with_spacer(200.0).with_child(return_to_main)
}

// index is the id of the study set
fn learn_page_builder(index: usize, test_name: String) -> impl Widget<AppState> {
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
            ctx.window().close();
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

    let return_to_main = Button::new("Return to Study Sets List").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            let new_win = WindowDesc::new(start_page_builder(data.storage_unit.clone())).title(
                MAIN_TITLE
            );
            ctx.new_window(new_win);
            ctx.window().close();
        }
    );

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
    card.with_spacer(200.0).with_child(return_to_main)
}

fn get_scores(user_answers: Vec<String>, expected_answers: Vec<String>) -> usize {
    let mut score = 0;
    for i in 0..user_answers.len() {
        if user_answers[i] == expected_answers[i] {
            score += 1;
        }
    }
    score
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
    let score_label = Label::new(
        format!(
            "You Scored: {}/{}",
            get_scores(user_answers.clone(), expected_answers.clone()),
            user_answers.len()
        )
    ).with_text_size(32.0).with_text_color(Color::AQUA);
    let mut list: Flex<AppState> = Flex::column().with_child(lesson_label).with_spacer(30.0).with_child(score_label);
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

fn add_word_page_builder(set_id: usize) -> impl Widget<AppState> {
    let word_label = Label::new(String::from("New Word")).with_text_size(32.0);
    let word = TextBox::new()
        .with_placeholder("Enter Word")
        .with_text_size(24.0)
        .fix_width(300.0)
        .lens(AppState::word_to_add);
    let word_ans_label = Label::new(String::from("Answer for New Word")).with_text_size(32.0);
    let word_ans = TextBox::new()
        .with_placeholder("Enter Answer for Word")
        .with_text_size(24.0)
        .fix_width(300.0)
        .lens(AppState::word_ans_to_add);
    let word_remark_label = Label::new(String::from("Remarks for New Word")).with_text_size(32.0);
    let word_remark = TextBox::new()
        .with_placeholder("Enter Remark for Word")
        .with_text_size(24.0)
        .fix_width(300.0)
        .lens(AppState::word_remark_to_add);
    let save_button = Button::new("Add to Set").on_click(
        move |ctx, data: &mut AppState, _env| -> () {
            let mut target_set = data.storage_unit.get_study_set(set_id);
            let new_card = Card::new(
                target_set.get_num_of_cards(),
                data.word_to_add.clone(),
                data.word_ans_to_add.clone(),
                data.word_remark_to_add.clone()
            );
            target_set.add_card(new_card);
            let window_title = target_set.get_desc();
            let lesson_name = window_title.clone();
            let new_win = WindowDesc::new(
                view_page_builder(set_id, lesson_name, target_set.get_all_cards())
            ).title(window_title);
            // clear data
            data.word_remark_to_add.clear();
            data.word_ans_to_add.clear();
            data.word_to_add.clear();
            data.storage_unit.update_set(set_id, target_set);
            data.storage_unit.save();
            ctx.request_update();
            ctx.window().close();
            ctx.new_window(new_win);
        }
    );
    Flex::column()
        .with_child(word_label)
        .with_spacer(10.0)
        .with_child(word)
        .with_spacer(50.0)
        .with_child(word_ans_label)
        .with_spacer(10.0)
        .with_child(word_ans)
        .with_spacer(50.0)
        .with_child(word_remark_label)
        .with_spacer(10.0)
        .with_child(word_remark)
        .with_spacer(50.0)
        .with_child(save_button)
        .center()
}

fn view_page_builder(
    lesson_id: usize,
    lesson_name: String,
    cards: Vec<Card>
) -> impl Widget<AppState> {
    let return_to_main = Button::new("Return to Study Sets List").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            let new_win = WindowDesc::new(start_page_builder(data.storage_unit.clone())).title(
                MAIN_TITLE
            );
            ctx.new_window(new_win);
            ctx.window().close();
        }
    );
    let lesson_label: Align<AppState> = Label::new(lesson_name.clone())
        .with_text_size(32.0)
        .with_text_color(Color::TEAL)
        .center();
    let mut list: Flex<AppState> = Flex::column()
        .with_child(return_to_main.align_left())
        .with_spacer(30.0)
        .with_child(lesson_label);
    for i in 0..cards.len() {
        let delete_word_button = Button::new("Delete").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
                let mut target_set = data.storage_unit.get_study_set(lesson_id);
                target_set.delete_card(i);
                let window_title = target_set.get_desc();
                let new_win = WindowDesc::new(
                    view_page_builder(lesson_id, window_title.clone(), target_set.get_all_cards())
                ).title(window_title);
                data.storage_unit.update_set(lesson_id, target_set);
                data.storage_unit.save();
                ctx.new_window(new_win);
                ctx.window().close();
            }
        );
        let edit_word_button = Button::new("Edit");
        let word = format!("Word {}:\n[{}]", cards[i].get_id() + 1, cards[i].get_word());
        let word_label: Label<AppState> = Label::new(word)
            .with_text_size(24.0)
            .with_text_color(Color::FUCHSIA);
        let mut word_row: Flex<AppState> = Flex::column();
        let expected_ans = format!("Correct Answer:\n[{}]", cards[i].get_ans());
        let answer_label: Label<AppState> = Label::new(expected_ans)
            .with_text_size(24.0)
            .with_text_color(Color::SILVER);
        let remarks = format!("Remarks:\n[{}]", cards[i].get_remarks());
        let remarks_label: Label<AppState> = Label::new(remarks)
            .with_text_size(24.0)
            .with_text_color(Color::OLIVE);
        let buttons_row = Flex::row().with_child(edit_word_button).with_spacer(10.0).with_child(delete_word_button);
        word_row = word_row
            .with_child(word_label.align_left())
            .with_child(answer_label.align_left())
            .with_child(remarks_label.align_left())
            .with_child(buttons_row);
        list = list.with_child(word_row.padding(20.0).border(Color::YELLOW, 1.0).padding(5.0));
    }
    let add_word_button = Button::new("Add Word").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, _data: &mut AppState, _env| {
            let new_win = WindowDesc::new(add_word_page_builder(lesson_id)).title(
                lesson_name.clone()
            );
            ctx.new_window(new_win);
            ctx.window().close();
        }
    );
    list = list.with_spacer(30.0).with_child(add_word_button);
    let scroll = Scroll::new(list.padding(40.0)).vertical();
    scroll
}

fn start_page_builder(storage: Storage) -> impl Widget<AppState> {
    let study_sets = storage.get_all();
    let mut list = Flex::column();
    for set in study_sets {
        let id = set.get_id();
        let id_clone = id.clone();
        let set_cloned_for_view = set.clone();
        let set_cloned = set.clone();
        let mut section = Flex::column();
        let label = Label::new(set.get_desc()).with_text_size(24.0);
        let view_button = Button::new("View").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, _data: &mut AppState, _env| {
                let new_win = WindowDesc::new(
                    view_page_builder(
                        set_cloned_for_view.get_id(),
                        set_cloned_for_view.get_desc(),
                        set_cloned_for_view.get_all_cards()
                    )
                ).title(set_cloned_for_view.get_desc());
                ctx.new_window(new_win);
                ctx.window().close();
            }
        );
        let learn_button = Button::new("Learn").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, _data: &mut AppState, _env| {
                let new_win = WindowDesc::new(
                    learn_page_builder(set_cloned.get_id(), set_cloned.get_desc())
                ).title(set_cloned.get_desc());
                ctx.new_window(new_win);
                ctx.window().close();
            }
        );
        let test_button = Button::new("Test").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, _data: &mut AppState, _env| {
                let new_win = WindowDesc::new(
                    test_page_builder(set.get_id(), set.get_desc())
                ).title(set.get_desc());
                ctx.new_window(new_win);
                ctx.window().close();
            }
        );
        let delete_button = Button::new("Delete").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
                data.storage_unit.delete_set(id_clone);
                data.storage_unit.save();
                let new_win = WindowDesc::new(start_page_builder(data.storage_unit.clone())).title(
                    MAIN_TITLE
                );
                ctx.new_window(new_win);
                ctx.window().close();
            }
        );

        let edit_setname_button = Button::new("Edit").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, _data: &mut AppState, _env| {
                let new_win = WindowDesc::new(edit_setname_page_builder(id)).title("Edit Set Name");
                ctx.new_window(new_win);
                ctx.window().close();
            }
        );
        section.add_child(label);
        let mut row = Flex::row();
        row.add_child(view_button);
        row.add_child(learn_button);
        row.add_child(test_button);
        row.add_child(delete_button);
        row.add_child(edit_setname_button);
        section = section.with_spacer(20.0).with_child(row);
        list.add_child(section.padding(30.0).border(Color::OLIVE, 2.0).padding(10.0));
    }
    let add_set_button = Button::new("Add Set").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, _data: &mut AppState, _env| {
            let new_win = WindowDesc::new(add_set_page_builder()).title("Add New Set");
            ctx.new_window(new_win);
            ctx.window().close();
        }
    );
    list = list.with_spacer(10.0).with_child(add_set_button.center());
    let scroll = Scroll::new(list).vertical();
    let aligned_widget = Align::right(scroll);
    aligned_widget
}

fn add_set_page_builder() -> impl Widget<AppState> {
    let error_label = Label::dynamic(|data: &AppState, _env| {
        if is_valid(data.new_set_name.clone()) {
            return String::from("Set Name Cannot Be Empty!");
        } else {
            return String::from("Enter Set Name");
        }
    })
        .with_text_size(32.0)
        .with_text_color(Color::YELLOW);
    let input = TextBox::new().with_text_size(24.0).fix_width(300.0).lens(AppState::new_set_name);
    let save_button = Button::new("Add Set").on_click(move |ctx, data: &mut AppState, _env| {
        let set_name = &data.new_set_name;
        if is_valid(set_name.to_string()) {
            let new_set = StudySet::new(set_name.clone(), data.storage_unit.get_num_of_sets());
            data.storage_unit.add_set(new_set);
            data.storage_unit.save();
            let new_win = WindowDesc::new(start_page_builder(data.storage_unit.clone())).title(
                MAIN_TITLE
            );
            data.new_set_name.clear();
            ctx.new_window(new_win);
            ctx.window().close();
        }
    });
    Flex::column()
        .with_child(error_label)
        .with_spacer(50.0)
        .with_child(input)
        .with_spacer(50.0)
        .with_child(save_button)
        .center()
}

fn edit_setname_page_builder(set_id: usize) -> impl Widget<AppState> {
    let error_label = Label::dynamic(|data: &AppState, _env| {
        if is_valid(data.new_set_name.clone()) {
            return String::from("Set Name Cannot Be Empty!");
        } else {
            return String::from("Enter Set Name");
        }
    })
        .with_text_size(32.0)
        .with_text_color(Color::YELLOW);
    let input = TextBox::new().with_text_size(24.0).fix_width(300.0).lens(AppState::new_set_name);
    let save_button = Button::new("Add Set").on_click(move |ctx, data: &mut AppState, _env| {
        let set_name = &data.new_set_name;
        if is_valid(set_name.to_string()) {
            let mut target_set = data.storage_unit.get_study_set(set_id);
            target_set.rename_set(set_name.to_string());
            data.storage_unit.update_set(set_id, target_set);
            data.storage_unit.save();
            let new_win = WindowDesc::new(start_page_builder(data.storage_unit.clone())).title(
                MAIN_TITLE
            );
            data.new_set_name.clear();
            ctx.new_window(new_win);
            ctx.window().close();
        }
    });
    Flex::column()
        .with_child(error_label)
        .with_spacer(50.0)
        .with_child(input)
        .with_spacer(50.0)
        .with_child(save_button)
        .center()
}

pub fn main() {
    let storage_unit = storage::Storage::new();
    let main_window = WindowDesc::new(start_page_builder(storage_unit.clone())).title(MAIN_TITLE);
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppState::default(storage_unit))
        .unwrap();
}
