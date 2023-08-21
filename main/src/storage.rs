use druid::Data;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

const DATA_DIR_PATH: &str = "./data";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Card {
    id: usize,
    word: String,
    ans: String,
    remarks: String,
}

impl Data for Card {
    fn same(&self, other: &Self) -> bool {
        if self.word == other.word && self.ans == other.ans && self.remarks == other.remarks {
            return true;
        }
        false
    }
}

impl Card {
    pub fn new(
        new_card_id: usize,
        new_card_word: String,
        new_card_ans: String,
        new_card_remark: String,
    ) -> Card {
        Card {
            id: new_card_id,
            word: new_card_word,
            ans: new_card_ans,
            remarks: new_card_remark,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id.clone()
    }

    pub fn get_word(&self) -> String {
        self.word.clone()
    }

    pub fn get_ans(&self) -> String {
        self.ans.clone()
    }

    pub fn get_remarks(&self) -> String {
        self.remarks.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StudySet {
    id: usize,
    name: String,
    tags: Vec<String>,
    cards: Vec<Card>,
}

impl Data for StudySet {
    fn same(&self, other: &Self) -> bool {
        // Implement comparison logic for MyObject
        // Return true if the objects are considered the same, false otherwise
        // ...
        if self.name == other.name && self.id == other.id && self.cards.len() == other.cards.len() {
            for i in 0..self.cards.len() {
                if self.cards[i].same(&other.cards[i]) {
                    return true;
                }
            }
            for i in 0..self.tags.len() {
                if self.tags[i].same(&other.tags[i]) {
                    return true;
                }
            }
        }
        false
    }
}

impl StudySet {
    pub fn new(new_set_name: String, new_set_id: usize) -> StudySet {
        StudySet {
            id: new_set_id,
            name: new_set_name,
            tags: vec![],
            cards: vec![],
        }
    }

    pub fn rename_set(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    pub fn set_id(&mut self, new_id: usize) {
        self.id = new_id;
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn replace_card(&mut self, index: usize, card: Card) {
        self.cards[index] = card;
    }

    pub fn delete_card(&mut self, to_remove: usize) {
        self.cards.remove(to_remove);
    }

    pub fn get_set_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_all_cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    pub fn get_num_of_cards(&self) -> usize {
        self.cards.len()
    }

    pub fn get_card(&self, to_get: usize) -> Card {
        self.cards[to_get].clone()
    }

    pub fn get_all_tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn has_tag(&self, tag: String) -> bool {
        self.tags.contains(&tag)
    }

    pub fn has_any_tags(&self, tags: HashSet<String>) -> bool {
        for tag in tags {
            if self.has_tag(tag) {
                return true;
            }
        }
        false
    }

    pub fn has_all_tags(&self, tags: HashSet<String>) -> bool {
        for tag in tags {
            if !self.has_tag(tag) {
                return false;
            }
        }
        true
    }

    pub fn get_tag_index(&self, tag: String) -> i8 {
        for i in 0..self.tags.len() {
            if self.tags[i] == tag {
                return i as i8;
            }
        }
        return -1;
    }

    pub fn delete_tag(&mut self, tag: String) {
        if self.tags.contains(&tag) {
            let tag_ind = self.get_tag_index(tag);
            self.tags.remove(tag_ind as usize);
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Storage {
    sets: Vec<StudySet>,
}

impl Data for Storage {
    fn same(&self, other: &Self) -> bool {
        for i in 0..self.sets.len() {
            if self.sets[i].same(&other.sets[i]) {
                return true;
            }
        }
        false
    }
}

impl Storage {
    pub fn new() -> Storage {
        let data_sets = Storage::read();
        Storage { sets: data_sets }
    }

    pub fn get_all_tags(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for set in &self.sets {
            for tag in set.get_all_tags() {
                if !res.contains(&tag) {
                    res.push(tag);
                }
            }
        }
        res
    }

    pub fn get_all_study_sets(&self) -> Vec<StudySet> {
        self.sets.clone()
    }

    pub fn get_study_set(&self, to_get: usize) -> StudySet {
        self.sets[to_get].clone()
    }

    pub fn get_study_set_by_tag(&self, tag: String) -> Vec<StudySet> {
        let mut sets = Vec::new();
        for set in &self.sets {
            if set.has_tag(tag.clone()) {
                sets.push(set.clone());
            }
        }
        sets
    }

    pub fn get_study_set_by_tags(
        &self,
        tags: HashSet<String>,
        is_match_any: bool,
    ) -> Vec<StudySet> {
        let mut sets = Vec::new();
        if is_match_any {
            for set in &self.sets {
                if set.has_any_tags(tags.clone()) {
                    sets.push(set.clone());
                }
            }
        } else {
            for set in &self.sets {
                if set.has_all_tags(tags.clone()) {
                    sets.push(set.clone());
                }
            }
        }
        sets
    }

    pub fn get_num_of_sets(&self) -> usize {
        self.sets.len()
    }

    pub fn update_set(&mut self, set_id: usize, updated_set: StudySet) {
        if self.sets[set_id].get_set_name() != updated_set.get_set_name() {
            self.rename_set_file(self.sets[set_id].get_set_name(), updated_set.get_set_name());
        }
        self.sets[set_id] = updated_set.clone();
        self.update_set_file(updated_set);
    }

    pub fn add_set(&mut self, new_set: StudySet) {
        self.sets.push(new_set.clone());
        self.create_set_file(new_set);
    }

    pub fn delete_set(&mut self, set_id: usize) {
        for i in 0..self.sets.len() {
            if self.sets[i].get_id() == set_id {
                let set_name = self.sets[i].get_set_name();
                self.delete_set_file(set_name);
                self.sets.remove(i);
                break;
            }
        }
    }

    pub fn create_set_file(&self, set: StudySet) {
        let file_name = set.get_set_name();
        let set_data = serde_json::to_string_pretty(&set).expect("Error parsing data to json");
        let set_data_path = format!("{}/{}.json", DATA_DIR_PATH, file_name);
        let err_msg_create = format!("Failed to create set data file {}.json", file_name);
        File::create(set_data_path.clone()).expect(&err_msg_create);
        let err_msg_open = format!("Failed to open set data file {}.json", file_name);
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(set_data_path)
            .expect(&err_msg_open);
        let _ = file.write_all(set_data.as_bytes());
    }

    pub fn rename_set_file(&self, prev_set_name: String, new_set_name: String) {
        let prev_set_file_name = format!("{}/{}.json", DATA_DIR_PATH, prev_set_name);
        let new_set_file_name = format!("{}/{}.json", DATA_DIR_PATH, new_set_name);
        let err_msg = format!(
            "Failed to change name from [{}] to [{}]",
            prev_set_file_name, new_set_file_name
        );
        fs::rename(prev_set_file_name, new_set_file_name).expect(&err_msg);
    }

    pub fn update_set_file(&self, set: StudySet) {
        let file_name = set.get_set_name();
        let set_data = serde_json::to_string_pretty(&set).expect("Error parsing data to json");
        let set_data_path = format!("{}/{}.json", DATA_DIR_PATH, file_name);
        let err_msg_open = format!("Failed to open set data file {}.json", file_name);
        let err_msg_write = format!("Failed to write to set data file {}.json", file_name);
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(set_data_path)
            .expect(&err_msg_open);
        file.write_all(set_data.as_bytes()).expect(&err_msg_write);
    }

    pub fn delete_set_file(&self, set_name: String) {
        let set_data_path = format!("{}/{}.json", DATA_DIR_PATH, set_name);
        let err_msg_delete = format!("Failed to delete set data file {}.json", set_name);
        fs::remove_file(set_data_path).expect(&err_msg_delete);
    }

    pub fn read() -> Vec<StudySet> {
        if !fs::metadata(&DATA_DIR_PATH).is_ok() {
            fs::create_dir(DATA_DIR_PATH).expect("Failed to Create Data Folder");
        }
        let mut sets: Vec<StudySet> = Vec::new();
        match fs::read_dir(DATA_DIR_PATH) {
            Ok(dir_entries) => {
                for entry in dir_entries {
                    if let Ok(entry) = entry {
                        let set_data_file_path = entry.path();
                        let set_data = fs::read_to_string(set_data_file_path)
                            .expect("Failed to read set data file");
                        let read_data_json = serde_json::from_str(&set_data);
                        match read_data_json {
                            Ok(data) => {
                                sets.push(data);
                            }
                            Err(_) => {
                                println!("Error Reading Data");
                            }
                        }
                    } else {
                        eprintln!("Error reading directory entry");
                    }
                }
            }
            Err(err) => eprintln!("Error reading directory: {}", err),
        }
        sets.sort_by(|a, b| a.id.cmp(&b.id));
        sets
    }
}
