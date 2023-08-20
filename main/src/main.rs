#![windows_subsystem = "windows"]

use std::collections::HashSet;

use druid::{
    theme,
    widget::{Align, Button, Flex, Label, Painter, Scroll, TextBox},
    AppLauncher, Color, Data, Lens, RenderContext, Widget, WidgetExt, WindowDesc,
};
use storage::{Card, Storage, StudySet};

const MAIN_TITLE: &str = "Quiz Late";
const SELECTED_TAG_COLOR: druid::Color = Color::rgba8(52, 222, 235, 255);
const UNSELECTED_TAG_COLOR: druid::Color = Color::rgba8(52, 222, 235, 0);

mod storage;
/*
 * Data
 * Ui Builder
 * Main
 */

#[derive(Clone, Lens, Debug)]
struct AppState {
    // store inputs by user
    input_str: Vec<Vec<String>>,
    // message to show on clicking submit
    res: Vec<Vec<String>>,
    // indexes for all studysets
    curr_indexes: Vec<usize>,
    str: String,
    storage_unit: Storage,
    // for adding a word to studyset
    word_to_add: String,
    word_ans_to_add: String,
    word_remark_to_add: String,
    // for adding a studyset
    new_set_name: String,
    new_set_tag: String,
    // for learn function
    answer_to_show: String,
    current_filter: HashSet<String>,
}

fn is_valid(input_str: String) -> bool {
    input_str.replace(" ", "") != ""
}

// todo: color coding of tags
// enum Module {}

// fn get_color_code(tag: String) -> Color {
//     Color::RED
// }

impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        if !is_same_2_d_vec_string(self.input_str.clone(), other.input_str.clone()) {
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
        if self.answer_to_show != other.answer_to_show {
            return false;
        }
        for elem in &self.current_filter {
            if !other.current_filter.contains(elem) {
                return false;
            }
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
        let mut input_all: Vec<Vec<String>> = Vec::new();
        let mut res_all: Vec<Vec<String>> = Vec::new();
        let mut indexes = Vec::new();
        for i in 0..storage.get_num_of_sets() {
            let mut card_set_inputs = Vec::new();
            let mut card_set_res = Vec::new();
            (0..storage.get_study_set(i).get_num_of_cards()).for_each(|_i| {
                card_set_inputs.push("".to_string());
                card_set_res.push("".to_string());
            });
            input_all.push(card_set_inputs);
            res_all.push(card_set_res);
            indexes.push(0);
        }
        AppState {
            input_str: input_all,
            res: res_all,
            curr_indexes: indexes,
            str: String::new(),
            storage_unit: storage,
            word_to_add: String::new(),
            word_ans_to_add: String::new(),
            word_remark_to_add: String::new(),
            new_set_name: String::new(),
            new_set_tag: String::new(),
            answer_to_show: String::new(),
            current_filter: HashSet::new(),
        }
    }
}

fn test_page_builder(set_index: usize, test_name: String) -> impl Widget<AppState> {
    let word_label = Label::dynamic(move |data: &AppState, _env| -> String {
        let word_index = data.curr_indexes[set_index];
        data.storage_unit
            .get_study_set(set_index)
            .get_card(word_index)
            .get_word()
    })
    .with_text_size(32.0);
    let remarks_label = Label::dynamic(move |data: &AppState, _env| -> String {
        let remark_index = data.curr_indexes[set_index];
        data.storage_unit
            .get_study_set(set_index)
            .get_card(remark_index)
            .get_remarks()
    })
    .with_text_size(32.0);

    let text_box = TextBox::new()
        .with_placeholder("Enter text here")
        .with_text_size(24.0)
        .fix_width(300.0)
        .lens(AppState::str);

    let clear = Button::new("Clear").on_click(move |ctx, data: &mut AppState, _env| -> () {
        let message = String::from("Input Cleared");
        data.str.clear();
        let word_index = data.curr_indexes[set_index];
        data.res[set_index][word_index] = message;
        ctx.request_update();
    });
    let prev = Button::new("Prev").on_click(move |ctx, data: &mut AppState, _env| -> () {
        let ind = data.curr_indexes[set_index];
        data.input_str[set_index][ind] = data.str.clone();
        if data.curr_indexes[set_index] >= 1 {
            data.curr_indexes[set_index] -= 1;
            data.str.clear();
        }
        ctx.request_update();
    });
    let next = Button::new("Next").on_click(move |ctx, data: &mut AppState, _env| -> () {
        let ind = data.curr_indexes[set_index];
        data.input_str[set_index][ind] = data.str.clone();
        if data.curr_indexes[set_index]
            < data
                .storage_unit
                .get_study_set(set_index)
                .get_num_of_cards()
                - 1
        {
            data.curr_indexes[set_index] += 1;
            data.str.clear();
        }
        ctx.request_update();
    });

    let eval_results = Button::new("Submit Test").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            let ind = data.curr_indexes[set_index];
            data.input_str[set_index][ind] = data.str.clone();
            let results_window = WindowDesc::new(result_page_builder(
                test_name.clone(),
                data.input_str[set_index].clone(),
                data.storage_unit.get_study_set(set_index).clone(),
            ))
            .title("Resuts");
            ctx.window().close();
            ctx.new_window(results_window);
        },
    );

    let res_label = Label::dynamic(move |data: &AppState, _| {
        let word_index = data.curr_indexes[set_index];
        data.res[set_index][word_index].clone()
    })
    .with_text_size(24.0);
    let index_label = Label::dynamic(move |data: &AppState, _| {
        format!(
            "{} / {}\n",
            data.curr_indexes[set_index] + 1,
            data.storage_unit
                .get_study_set(set_index)
                .get_num_of_cards()
        )
    })
    .with_text_size(24.0);

    let inputs = Flex::row()
        .with_child(prev)
        .with_child(clear)
        .with_child(next);

    let return_to_main = Button::new("Return to Study Sets List").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            let new_win = WindowDesc::new(start_page_builder(
                data.storage_unit.get_all_study_sets(),
                data.storage_unit.get_all_tags(),
            ))
            .title(MAIN_TITLE);
            ctx.window().close();
            ctx.new_window(new_win);
        },
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
    card.with_spacer(20.0).with_child(return_to_main)
}

// index is the id of the study set
fn learn_page_builder(set_index: usize, test_name: String) -> impl Widget<AppState> {
    let word_label = Label::dynamic(move |data: &AppState, _env| -> String {
        let word_index = data.curr_indexes[set_index];
        data.storage_unit
            .get_study_set(set_index)
            .get_card(word_index)
            .get_word()
    })
    .with_text_size(32.0);
    let remarks_label = Label::dynamic(move |data: &AppState, _env| -> String {
        let remark_index = data.curr_indexes[set_index];
        data.storage_unit
            .get_study_set(set_index)
            .get_card(remark_index)
            .get_remarks()
    })
    .with_text_size(32.0);
    let text_box = TextBox::new()
        .with_placeholder("Enter text here")
        .with_text_size(24.0)
        .fix_width(300.0)
        .lens(AppState::str);
    let enter = Button::new("Confirm").on_click(move |ctx, data: &mut AppState, _env| -> () {
        let word_index = data.curr_indexes[set_index];
        if data.str
            == data
                .storage_unit
                .get_study_set(set_index)
                .get_card(word_index)
                .get_ans()
        {
            data.res[set_index][word_index] = String::from("Correct!");
        } else {
            data.res[set_index][word_index] = String::from("Try Again!");
        }
        ctx.request_update();
    });

    let clear = Button::new("Clear").on_click(move |ctx, data: &mut AppState, _env| -> () {
        let message = String::from("Input Cleared");
        data.str.clear();
        let word_index = data.curr_indexes[set_index];
        data.res[set_index][word_index] = message;
        ctx.request_update();
    });
    let prev = Button::new("Prev").on_click(move |ctx, data: &mut AppState, _env| -> () {
        data.answer_to_show.clear();
        let ind = data.curr_indexes[set_index];
        data.input_str[set_index][ind] = data.str.clone();
        if data.curr_indexes[set_index] >= 1 {
            data.curr_indexes[set_index] -= 1;
            data.str.clear();
        }
        ctx.request_update();
    });
    let next = Button::new("Next").on_click(move |ctx, data: &mut AppState, _env| -> () {
        data.answer_to_show.clear();
        let ind = data.curr_indexes[set_index];
        data.input_str[set_index][ind] = data.str.clone();
        if data.curr_indexes[set_index]
            < data
                .storage_unit
                .get_study_set(set_index)
                .get_num_of_cards()
                - 1
        {
            data.curr_indexes[set_index] += 1;
            data.str.clear();
        }
        ctx.request_update();
    });

    let show_answer =
        Button::new("Show Answer").on_click(move |ctx, data: &mut AppState, _env| -> () {
            let word_index = data.curr_indexes[set_index];
            data.answer_to_show = data
                .storage_unit
                .get_study_set(set_index)
                .get_card(word_index)
                .get_ans();
            ctx.request_update();
        });

    let hide_answer =
        Button::new("Hide Answer").on_click(move |ctx, data: &mut AppState, _env| -> () {
            data.answer_to_show.clear();
            ctx.request_update();
        });

    let eval_results = Button::new("Calculate Score").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            data.answer_to_show.clear();
            let ind = data.curr_indexes[set_index];
            data.input_str[set_index][ind] = data.str.clone();
            let results_window = WindowDesc::new(result_page_builder(
                test_name.clone(),
                data.input_str[set_index].clone(),
                data.storage_unit.get_study_set(set_index),
            ))
            .title("Resuts");
            ctx.window().close();
            ctx.new_window(results_window);
        },
    );

    let res_label = Label::dynamic(move |data: &AppState, _| {
        let word_index = data.curr_indexes[set_index];
        data.res[set_index][word_index].clone()
    })
    .with_text_size(24.0);

    let show_answer_label = Label::dynamic(|data: &AppState, _| data.answer_to_show.clone())
        .with_text_size(24.0)
        .with_text_color(Color::AQUA);

    let index_label = Label::dynamic(move |data: &AppState, _| {
        format!(
            "{} / {}\n",
            data.curr_indexes[set_index] + 1,
            data.storage_unit
                .get_study_set(set_index)
                .get_num_of_cards()
        )
    })
    .with_text_size(24.0);

    let inputs = Flex::row()
        .with_child(prev)
        .with_child(enter)
        .with_child(clear)
        .with_child(next);

    let answer_toggle = Flex::row().with_child(show_answer).with_child(hide_answer);

    let return_to_main = Button::new("Return to Study Sets List").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            data.answer_to_show.clear();
            let new_win = WindowDesc::new(start_page_builder(
                data.storage_unit.get_all_study_sets(),
                data.storage_unit.get_all_tags(),
            ))
            .title(MAIN_TITLE);
            ctx.window().close();
            ctx.new_window(new_win);
        },
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
        .with_child(answer_toggle)
        .with_spacer(20.0)
        .with_child(show_answer_label)
        .with_spacer(20.0)
        .with_child(res_label)
        .with_spacer(10.0)
        .with_child(eval_results);
    card.with_spacer(20.0).with_child(return_to_main)
}

fn get_scores(user_answers: Vec<String>, study_set: StudySet) -> usize {
    let mut score = 0;
    for i in 0..user_answers.len() {
        if user_answers[i] == study_set.get_card(i).get_ans() {
            score += 1;
        }
    }
    score
}

fn result_page_builder(
    test_name: String,
    user_answers: Vec<String>,
    study_set: StudySet,
) -> impl Widget<AppState> {
    let lesson_label: Align<AppState> = Label::new(test_name.clone())
        .with_text_size(32.0)
        .with_text_color(Color::TEAL)
        .center();
    let score_label = Label::new(format!(
        "You Scored: {}/{}",
        get_scores(user_answers.clone(), study_set.clone()),
        user_answers.len()
    ))
    .with_text_size(32.0)
    .with_text_color(Color::AQUA);
    let mut list: Flex<AppState> = Flex::column()
        .with_child(lesson_label)
        .with_spacer(30.0)
        .with_child(score_label);
    for i in 0..user_answers.len() {
        let word = format!("Word:\n[{}]", study_set.get_card(i).get_word());
        let word_label: Label<AppState> = Label::new(word)
            .with_text_size(24.0)
            .with_text_color(Color::FUCHSIA);
        let mut word_row: Flex<AppState> = Flex::row().with_child(word_label.padding(25.0));
        let user_ans = format!("Your Answer:\n[{}]", user_answers[i]);
        if user_answers[i] == study_set.get_card(i).get_ans() {
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
        let expected_ans = format!("Correct Answer:\n[{}]", study_set.get_card(i).get_ans());
        let answer_label: Label<AppState> = Label::new(expected_ans)
            .with_text_size(24.0)
            .with_text_color(Color::SILVER);
        word_row = word_row.with_child(answer_label);
        list = list.with_child(word_row);
    }
    let return_to_main = Button::new("Return to Study Sets List").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            let new_win = WindowDesc::new(start_page_builder(
                data.storage_unit.get_all_study_sets(),
                data.storage_unit.get_all_tags(),
            ))
            .title(MAIN_TITLE);
            ctx.request_update();
            ctx.window().close();
            ctx.new_window(new_win);
        },
    );
    list = list.with_spacer(30.0).with_child(return_to_main);
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
    let save_button =
        Button::new("Add to Set").on_click(move |ctx, data: &mut AppState, _env| -> () {
            let mut target_set = data.storage_unit.get_study_set(set_id);
            let new_card = Card::new(
                target_set.get_num_of_cards(),
                data.word_to_add.clone(),
                data.word_ans_to_add.clone(),
                data.word_remark_to_add.clone(),
            );
            target_set.add_card(new_card);
            let window_title = target_set.get_set_name();
            let lesson_name = window_title.clone();
            let new_win = WindowDesc::new(view_page_builder(
                set_id,
                lesson_name,
                target_set.get_all_cards(),
                target_set.get_all_tags(),
            ))
            .title(window_title);
            // clear data
            data.word_remark_to_add.clear();
            data.word_ans_to_add.clear();
            data.word_to_add.clear();
            data.storage_unit.update_set(set_id, target_set);
            if set_id == data.res.len() {
                data.res.push(Vec::new());
                data.input_str.push(Vec::new());
            }
            data.res[set_id].push(String::new());
            data.input_str[set_id].push(String::new());
            ctx.request_update();
            ctx.window().close();
            ctx.new_window(new_win);
        });
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

fn place_holder_helper(prev: String, input: String) -> String {
    if is_valid(input.clone()) {
        let temp = &input;
        return temp.trim().to_string();
    }
    return prev;
}

fn edit_word_page_builder(
    set_id: usize,
    word_id: usize,
    curr_word: String,
    curr_ans: String,
    curr_remarks: String,
) -> impl Widget<AppState> {
    let word_label = Label::new(String::from(format!("Word {}", word_id + 1))).with_text_size(32.0);
    let word = TextBox::new()
        .with_placeholder(curr_word.clone())
        .with_text_size(24.0)
        .fix_width(300.0)
        .lens(AppState::word_to_add);
    let word_ans_label = Label::new(String::from("Answer for Word")).with_text_size(32.0);
    let word_ans = TextBox::new()
        .with_placeholder(curr_ans.clone())
        .with_text_size(24.0)
        .fix_width(300.0)
        .lens(AppState::word_ans_to_add);
    let word_remark_label = Label::new(String::from("Remarks for Word")).with_text_size(32.0);
    let word_remark = TextBox::new()
        .with_placeholder(curr_remarks.clone())
        .with_text_size(24.0)
        .fix_width(300.0)
        .lens(AppState::word_remark_to_add);
    let save_button =
        Button::new("Save Changes").on_click(move |ctx, data: &mut AppState, _env| -> () {
            let mut target_set = data.storage_unit.get_study_set(set_id);
            let new_card = Card::new(
                word_id,
                place_holder_helper(curr_word.clone(), data.word_to_add.clone()),
                place_holder_helper(curr_ans.clone(), data.word_ans_to_add.clone()),
                place_holder_helper(curr_remarks.clone(), data.word_remark_to_add.clone()),
            );
            target_set.replace_card(word_id, new_card);
            let window_title = target_set.get_set_name();
            let lesson_name = window_title.clone();
            let new_win = WindowDesc::new(view_page_builder(
                set_id,
                lesson_name,
                target_set.get_all_cards(),
                target_set.get_all_tags(),
            ))
            .title(window_title);
            // clear data
            data.word_remark_to_add.clear();
            data.word_ans_to_add.clear();
            data.word_to_add.clear();
            data.storage_unit.update_set(set_id, target_set);
            if set_id == data.res.len() {
                data.res.push(Vec::new());
                data.input_str.push(Vec::new());
            }
            data.res[set_id].push(String::new());
            data.input_str[set_id].push(String::new());
            ctx.request_update();
            ctx.window().close();
            ctx.new_window(new_win);
        });
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
    cards: Vec<Card>,
    tags: Vec<String>,
) -> impl Widget<AppState> {
    let return_to_main = Button::new("Return to Study Sets List").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            let new_storage = data.storage_unit.clone();
            let new_win = WindowDesc::new(start_page_builder(
                new_storage.get_all_study_sets(),
                new_storage.get_all_tags(),
            ))
            .title(MAIN_TITLE);
            ctx.window().close();
            ctx.new_window(new_win);
        },
    );

    let prev_set = Button::new("Prev Set").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            if lesson_id >= 1 {
                let new_win = WindowDesc::new(view_page_builder(
                    lesson_id - 1,
                    data.storage_unit
                        .get_study_set(lesson_id - 1)
                        .get_set_name(),
                    data.storage_unit
                        .get_study_set(lesson_id - 1)
                        .get_all_cards(),
                    data.storage_unit
                        .get_study_set(lesson_id - 1)
                        .get_all_tags(),
                ))
                .title(MAIN_TITLE);
                ctx.window().close();
                ctx.new_window(new_win);
            }
        },
    );

    let next_set = Button::new("Next Set").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            if lesson_id + 1 < data.storage_unit.get_num_of_sets() {
                let new_win = WindowDesc::new(view_page_builder(
                    lesson_id + 1,
                    data.storage_unit
                        .get_study_set(lesson_id + 1)
                        .get_set_name(),
                    data.storage_unit
                        .get_study_set(lesson_id + 1)
                        .get_all_cards(),
                    data.storage_unit
                        .get_study_set(lesson_id + 1)
                        .get_all_tags(),
                ))
                .title(MAIN_TITLE);
                ctx.window().close();
                ctx.new_window(new_win);
            }
        },
    );

    let set_nav = Flex::row()
        .with_child(prev_set)
        .with_flex_spacer(10.0)
        .with_child(next_set);

    let lesson_label: Align<AppState> = Label::new(lesson_name.clone())
        .with_text_size(32.0)
        .with_text_color(Color::TEAL)
        .center();
    let mut list: Flex<AppState> = Flex::column()
        .with_child(set_nav)
        .with_spacer(30.0)
        .with_child(return_to_main.align_left())
        .with_spacer(30.0)
        .with_child(lesson_label);

    let mut tag_row = Flex::row();
    for tag in tags {
        let tag_label = Label::new(tag)
            .with_text_size(20.0)
            .with_text_color(Color::rgba(0.5, 0.3, 0.7, 1.0))
            .padding(3.0)
            .border(Color::YELLOW, 1.0);
        tag_row = tag_row.with_child(tag_label).with_spacer(5.0);
    }
    list = list
        .with_spacer(20.0)
        .with_child(tag_row);

    let add_word_button = Button::new("Add Word").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, _data: &mut AppState, _env| {
            let new_win =
                WindowDesc::new(add_word_page_builder(lesson_id)).title(lesson_name.clone());
            ctx.window().close();
            ctx.new_window(new_win);
        },
    );
    list = list.with_spacer(30.0).with_child(add_word_button);

    for i in 0..cards.len() {
        let ind = cards.len() - i - 1;
        let delete_word_button = Button::new("Delete").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
                let mut target_set = data.storage_unit.get_study_set(lesson_id);
                target_set.delete_card(ind);
                let window_title = target_set.get_set_name();
                let new_win = WindowDesc::new(view_page_builder(
                    lesson_id,
                    window_title.clone(),
                    target_set.get_all_cards(),
                    target_set.get_all_tags(),
                ))
                .title(window_title);
                data.storage_unit.update_set(lesson_id, target_set);
                ctx.window().close();
                ctx.new_window(new_win);
            },
        );
        let edit_word_button = Button::new("Edit").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
                let curr_card = data.storage_unit.get_study_set(lesson_id).get_card(ind);
                let new_win = WindowDesc::new(edit_word_page_builder(
                    lesson_id,
                    ind,
                    curr_card.get_word(),
                    curr_card.get_ans(),
                    curr_card.get_remarks(),
                ))
                .title("Edit Word");
                ctx.window().close();
                ctx.new_window(new_win);
            },
        );
        let word = format!(
            "Word {}:\n[{}]",
            cards[ind].get_id() + 1,
            cards[ind].get_word()
        );
        let word_label: Label<AppState> = Label::new(word)
            .with_text_size(24.0)
            .with_text_color(Color::FUCHSIA);
        let mut word_row: Flex<AppState> = Flex::column();
        let expected_ans = format!("Correct Answer:\n[{}]", cards[ind].get_ans());
        let answer_label: Label<AppState> = Label::new(expected_ans)
            .with_text_size(24.0)
            .with_text_color(Color::SILVER);
        let remarks = format!("Remarks:\n[{}]", cards[ind].get_remarks());
        let remarks_label: Label<AppState> = Label::new(remarks)
            .with_text_size(24.0)
            .with_text_color(Color::OLIVE);
        let buttons_row = Flex::row()
            .with_child(edit_word_button)
            .with_spacer(10.0)
            .with_child(delete_word_button);
        word_row = word_row
            .with_child(word_label.align_left())
            .with_child(answer_label.align_left())
            .with_child(remarks_label.align_left())
            .with_child(buttons_row);
        list = list.with_child(
            word_row
                .padding(20.0)
                .border(Color::YELLOW, 1.0)
                .padding(5.0),
        );
    }
    let scroll = Scroll::new(list.padding(40.0)).vertical();
    scroll
}

fn start_page_builder(study_sets: Vec<StudySet>, tags: Vec<String>) -> impl Widget<AppState> {
    let num_of_sets = study_sets.len();
    let mut list: Flex<AppState> = Flex::column();
    let filter_label = Label::new("Filter by tags")
        .with_text_size(32.0)
        .with_text_color(Color::PURPLE);
    let mut tags_list: Flex<AppState> = Flex::row().with_spacer(10.0);
    for i in 0..tags.len() {
        let curr_tag = tags[i].clone();
        let curr_tag_for_filter = curr_tag.clone();
        let filter_button = Button::new(curr_tag.clone())
            .background(Painter::new(move |ctx, data: &AppState, _env| {
                let bounds = ctx.size().to_rect();
                if data.current_filter.contains(&curr_tag_for_filter) {
                    ctx.fill(bounds, &SELECTED_TAG_COLOR);
                } else {
                    ctx.fill(bounds, &UNSELECTED_TAG_COLOR);
                }
            }))
            .on_click(
                move |_ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
                    if data.current_filter.contains(&curr_tag) {
                        data.current_filter.remove(&curr_tag);
                    } else {
                        data.current_filter.insert(curr_tag.clone());
                    }
                },
            );
        tags_list = tags_list.with_child(filter_button).with_spacer(10.0);
    }
    let inner_tags_list = Scroll::new(tags_list.padding(20.0).center()).horizontal();
    list.add_child(filter_label);
    list.add_child(inner_tags_list);

    let match_all = Button::new("Match All").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            if !data.current_filter.is_empty() {
                let new_win = WindowDesc::new(start_page_builder(
                    data.storage_unit
                        .get_study_set_by_tags(data.current_filter.clone(), false),
                    data.storage_unit.get_all_tags(),
                ))
                .title(MAIN_TITLE);
                data.current_filter.clear();
                ctx.window().close();
                ctx.new_window(new_win);
            }
        },
    );
    let match_any = Button::new("Match Any").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            if !data.current_filter.is_empty() {
                let new_win = WindowDesc::new(start_page_builder(
                    data.storage_unit
                        .get_study_set_by_tags(data.current_filter.clone(), true),
                    data.storage_unit.get_all_tags(),
                ))
                .title(MAIN_TITLE);
                data.current_filter.clear();
                ctx.window().close();
                ctx.new_window(new_win);
            }
        },
    );

    let all_sets = Button::new("See All Sets").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            if data.storage_unit.get_num_of_sets() != num_of_sets {
                let new_win = WindowDesc::new(start_page_builder(
                    data.storage_unit.get_all_study_sets(),
                    data.storage_unit.get_all_tags(),
                ))
                .title(MAIN_TITLE);
                data.current_filter.clear();
                ctx.window().close();
                ctx.new_window(new_win);
            }
        },
    );

    let mut filter_buttons = Flex::row();
    filter_buttons = filter_buttons
        .with_child(match_all)
        .with_spacer(10.0)
        .with_child(match_any)
        .with_spacer(10.0)
        .with_child(all_sets);

    list.add_child(filter_buttons);
    for set in study_sets {
        let id = set.get_id();
        let id_clone = id.clone();
        let set_cloned_for_view = set.clone();
        let set_cloned = set.clone();
        let mut section = Flex::column();
        let set_name_label = Label::new(set.get_set_name()).with_text_size(24.0);
        section.add_child(set_name_label);
        for tag in set_cloned.get_all_tags() {
            let tag_label = Label::new(tag).with_text_color(Color::LIME);
            section = section.with_spacer(5.0).with_child(tag_label);
        }
        let view_button = Button::new("View").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, _data: &mut AppState, _env| {
                let new_win = WindowDesc::new(view_page_builder(
                    set_cloned_for_view.get_id(),
                    set_cloned_for_view.get_set_name(),
                    set_cloned_for_view.get_all_cards(),
                    set_cloned_for_view.get_all_tags(),
                ))
                .title(set_cloned_for_view.get_set_name());
                ctx.window().close();
                ctx.new_window(new_win);
            },
        );
        let learn_button = Button::new("Learn").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, _data: &mut AppState, _env| {
                if set_cloned.get_num_of_cards() > 0 {
                    let new_win = WindowDesc::new(learn_page_builder(
                        set_cloned.get_id(),
                        set_cloned.get_set_name(),
                    ))
                    .title(set_cloned.get_set_name());
                    ctx.window().close();
                    ctx.new_window(new_win);
                }
            },
        );
        let test_button = Button::new("Test").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, _data: &mut AppState, _env| {
                if set.get_num_of_cards() > 0 {
                    let new_win =
                        WindowDesc::new(test_page_builder(set.get_id(), set.get_set_name()))
                            .title(set.get_set_name());
                    ctx.window().close();
                    ctx.new_window(new_win);
                }
            },
        );
        let delete_button = Button::new("Delete").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
                data.storage_unit.delete_set(id_clone);
                let new_win = WindowDesc::new(start_page_builder(
                    data.storage_unit.get_all_study_sets(),
                    data.storage_unit.get_all_tags(),
                ))
                .title(MAIN_TITLE);
                ctx.window().close();
                ctx.new_window(new_win);
            },
        );

        let edit_setname_button = Button::new("Edit").on_click(
            move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
                let new_win = WindowDesc::new(edit_set_page_builder(
                    id,
                    data.storage_unit.get_study_set(id).get_set_name(),
                    data.storage_unit.get_study_set(id).get_all_tags(),
                ))
                .title("Edit Set Name & Tags");
                ctx.window().close();
                ctx.new_window(new_win);
            },
        );
        let mut row = Flex::row();
        row.add_child(view_button);
        row.add_child(learn_button);
        row.add_child(test_button);
        row.add_child(delete_button);
        row.add_child(edit_setname_button);
        section = section.with_spacer(20.0).with_child(row);
        list.add_child(
            section
                .padding(30.0)
                .border(Color::OLIVE, 2.0)
                .padding(10.0),
        );
    }
    let add_set_button = Button::new("Add Set").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, _data: &mut AppState, _env| {
            let new_win = WindowDesc::new(add_set_page_builder()).title("Add New Set");
            ctx.window().close();
            ctx.new_window(new_win);
        },
    );
    list = list
        .with_spacer(10.0)
        .with_child(add_set_button.center())
        .with_spacer(30.0);
    let scroll = Scroll::new(list).vertical();
    let aligned_widget = Align::right(scroll);
    aligned_widget
}

fn add_set_page_builder() -> impl Widget<AppState> {
    let error_label = Label::new(String::from("Set Name Cannot Be Empty"))
        .with_text_size(32.0)
        .with_text_color(Color::YELLOW);
    let set_name_input = TextBox::new()
        .with_placeholder("Enter Set Name")
        .with_text_size(24.0)
        .fix_width(300.0)
        .lens(AppState::new_set_name);
    let set_tag_input = TextBox::new()
        .with_placeholder("Enter Tag")
        .with_text_size(24.0)
        .fix_width(300.0)
        .lens(AppState::new_set_tag);
    let save_button = Button::new("Add Set").on_click(move |ctx, data: &mut AppState, _env| {
        let set_name = &data.new_set_name;
        let set_tag = &data.new_set_tag;
        if is_valid(set_name.clone()) {
            let mut new_set = StudySet::new(
                set_name.trim().to_string(),
                data.storage_unit.get_num_of_sets(),
            );
            if !set_tag.clone().is_empty() {
                new_set.add_tag(set_tag.trim().to_string());
            }
            data.storage_unit.add_set(new_set);
            let new_win = WindowDesc::new(start_page_builder(
                data.storage_unit.get_all_study_sets(),
                data.storage_unit.get_all_tags(),
            ))
            .title(MAIN_TITLE);
            data.new_set_name.clear();
            ctx.window().close();
            ctx.new_window(new_win);
        }
    });
    Flex::column()
        .with_child(error_label)
        .with_spacer(50.0)
        .with_child(set_name_input)
        .with_spacer(50.0)
        .with_child(set_tag_input)
        .with_spacer(50.0)
        .with_child(save_button)
        .center()
}

fn edit_set_page_builder(
    set_id: usize,
    curr_name: String,
    curr_tags: Vec<String>,
) -> impl Widget<AppState> {
    let return_to_main = Button::new("Return to Study Sets List").on_click(
        move |ctx: &mut druid::EventCtx<'_, '_>, data: &mut AppState, _env| {
            let new_win = WindowDesc::new(start_page_builder(
                data.storage_unit.get_all_study_sets(),
                data.storage_unit.get_all_tags(),
            ))
            .title(MAIN_TITLE);
            ctx.window().close();
            ctx.new_window(new_win);
        },
    );
    let error_label = Label::new(String::from("Enter Set Name"))
        .with_text_size(32.0)
        .with_text_color(Color::YELLOW);
    let set_name_input = TextBox::new()
        .with_placeholder(curr_name.clone())
        .with_text_size(24.0)
        .fix_width(300.0)
        .lens(AppState::new_set_name);
    let set_tag_input = TextBox::new()
        .with_placeholder("Add Tag")
        .with_text_size(24.0)
        .fix_width(300.0)
        .lens(AppState::new_set_tag);

    let curr_tag_label = Label::new("Tags:")
        .with_text_size(32.0)
        .with_text_color(Color::YELLOW);
    let mut tag_row = Flex::row();
    for tag in curr_tags {
        let tag_label = Label::new(tag)
            .with_text_size(20.0)
            .with_text_color(Color::rgba(0.5, 0.3, 0.7, 1.0))
            .border(Color::YELLOW, 1.0);
        tag_row = tag_row.with_child(tag_label).with_spacer(5.0);
    }

    let save_button =
        Button::new("Save Changes").on_click(move |ctx, data: &mut AppState, _env| {
            let new_set_name = place_holder_helper(curr_name.clone(), data.new_set_name.clone());
            let set_tag = &data.new_set_tag;
            let mut target_set = data.storage_unit.get_study_set(set_id);
            target_set.rename_set(new_set_name);
            if is_valid(set_tag.clone()) {
                target_set.add_tag(set_tag.trim().to_string());
            }
            data.storage_unit.update_set(set_id, target_set);
            let new_win = WindowDesc::new(start_page_builder(
                data.storage_unit.get_all_study_sets(),
                data.storage_unit.get_all_tags(),
            ))
            .title(MAIN_TITLE);
            data.new_set_name.clear();
            ctx.window().close();
            ctx.new_window(new_win);
        });
    Flex::column()
        .with_child(return_to_main)
        .with_spacer(50.0)
        .with_child(error_label)
        .with_spacer(50.0)
        .with_child(set_name_input)
        .with_spacer(50.0)
        .with_child(set_tag_input)
        .with_spacer(50.0)
        .with_child(curr_tag_label)
        .with_spacer(10.0)
        .with_child(tag_row)
        .with_spacer(50.0)
        .with_child(save_button)
        .center()
}

pub fn main() {
    let storage_unit = storage::Storage::new();
    let main_window = WindowDesc::new(start_page_builder(
        storage_unit.get_all_study_sets(),
        storage_unit.get_all_tags(),
    ))
    .title(MAIN_TITLE);
    AppLauncher::with_window(main_window)
        // .log_to_console()
        .configure_env(|env, _state| {
            env.set(theme::BUTTON_DARK, Color::rgba8(100, 100, 120, 0));
            env.set(theme::BUTTON_LIGHT, Color::rgba8(100, 100, 100, 100));
        })
        .launch(AppState::default(storage_unit))
        .unwrap();
}
