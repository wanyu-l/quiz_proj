use druid::Data;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

const DATA_DIR_PATH: &str = "./data";
const INVENTORY_DIR_PATH: &str = "./inventory";
const INVENTORY_FILE_PATH: &str = "./inventory/inventory.json";

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

    pub fn set_id(&mut self, new_card_id: usize) {
        self.id = new_card_id;
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
    name: String,
    tags: Vec<String>,
    cards: Vec<Card>,
}

impl Data for StudySet {
    fn same(&self, other: &Self) -> bool {
        if self.name == other.name && self.cards.len() == other.cards.len() {
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
    pub fn new(new_set_name: String) -> StudySet {
        StudySet {
            name: new_set_name,
            tags: vec![],
            cards: vec![],
        }
    }

    pub fn clean_up_set(&mut self) {
        let mut count = 0;
        let mut new_cards = Vec::new();
        for card in &self.cards {
            let mut temp = card.clone();
            temp.set_id(count);
            new_cards.push(temp);
            count += 1;
        }
        self.cards = new_cards;
    }

    pub fn rename_set(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn replace_card(&mut self, index: usize, card: Card) {
        self.cards[index] = card;
    }

    pub fn delete_card(&mut self, to_remove: usize) {
        self.cards.remove(to_remove);
        self.clean_up_set();
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
}

pub struct Storage;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListItem {
    item_id: usize,
    item_name: String,
    item_tags: Vec<String>,
    num_of_cards: usize,
}

impl ListItem {
    fn new(id: usize, name: String, tags: Vec<String>, num: usize) -> ListItem {
        ListItem {
            item_id: id,
            item_name: name,
            item_tags: tags,
            num_of_cards: num,
        }
    }

    pub fn get_id(&self) -> usize {
        self.item_id.clone()
    }

    pub fn set_id(&mut self, id: usize) {
        self.item_id = id;
    }

    pub fn rename(&mut self, new_name: String) {
        self.item_name = new_name;
    }

    pub fn get_name(&self) -> String {
        self.item_name.clone()
    }

    pub fn get_all_tags(&self) -> Vec<String> {
        self.item_tags.clone()
    }

    pub fn get_num_of_cards(&self) -> usize {
        self.num_of_cards
    }

    pub fn has_tag(&self, tag: String) -> bool {
        self.item_tags.contains(&tag)
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
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Catalogue {
    inventory: Vec<ListItem>,
}

impl Catalogue {
    pub fn get_inventory(&self) -> Vec<ListItem> {
        self.inventory.clone()
    }

    pub fn get_num_of_items(&self) -> usize {
        self.inventory.len()
    }

    pub fn get_all_names(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for item in &self.inventory {
            res.push(item.get_name());
        }
        res
    }

    pub fn get_all_tags(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for item in &self.inventory {
            for tag in item.get_all_tags() {
                if !res.contains(&tag) {
                    res.push(tag);
                }
            }
        }
        res
    }

    pub fn get_items_by_tags(&self, tags: HashSet<String>, is_match_any: bool) -> Vec<ListItem> {
        let mut res = Vec::new();
        if is_match_any {
            for item in &self.inventory {
                if item.has_any_tags(tags.clone()) {
                    res.push(item.clone());
                }
            }
        } else {
            for item in &self.inventory {
                if item.has_all_tags(tags.clone()) {
                    res.push(item.clone());
                }
            }
        }
        res
    }

    pub fn get_all_untagged_study_sets(&self) -> Vec<ListItem> {
        let mut res = Vec::new();
        for item in &self.inventory {
            if item.get_all_tags().is_empty() {
                res.push(item.clone());
            }
        }
        res
    }

    pub fn update_set(&mut self, set_id: usize, updated_set: StudySet) {
        for i in 0..self.inventory.len() {
            if self.inventory[i].get_id() == set_id {
                if self.inventory[i].get_name() != updated_set.get_set_name() {
                    Storage::rename_set_file(
                        self.inventory[i].get_name(),
                        updated_set.get_set_name(),
                    );
                }
                Storage::update_set_file(updated_set.clone());
                let item = ListItem::new(
                    i,
                    updated_set.get_set_name(),
                    updated_set.get_all_tags(),
                    updated_set.get_num_of_cards(),
                );
                self.inventory[i] = item;
                break;
            }
        }
    }

    pub fn add_study_set(&mut self, study_set: StudySet, item_id: usize) {
        let item = ListItem::new(
            item_id,
            study_set.get_set_name(),
            study_set.get_all_tags(),
            study_set.get_num_of_cards(),
        );
        self.inventory.push(item);
        Storage::create_set_file(study_set);
    }

    pub fn get_item_by_id(&self, id: usize) -> Vec<ListItem> {
        let mut res = Vec::new();
        for item in &self.inventory {
            if item.get_id() == id {
                res.push(item.clone());
                break;
            }
        }
        res
    }

    pub fn delete_item_by_id(&mut self, id: usize) {
        let mut res = Vec::new();
        let mut count = 0;
        for item in &self.inventory {
            if item.get_id() != id {
                let mut new_item = ListItem::new(
                    item.get_id(),
                    item.get_name(),
                    item.get_all_tags(),
                    item.get_num_of_cards(),
                );
                new_item.set_id(count);
                res.push(new_item.clone());
                count += 1;
            } else {
                Storage::delete_set_file(item.get_name());
            }
        }
        self.inventory = res;
    }
}

impl Storage {
    pub fn create_set_file(set: StudySet) {
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

    pub fn rename_set_file(prev_set_name: String, new_set_name: String) {
        let prev_set_file_name = format!("{}/{}.json", DATA_DIR_PATH, prev_set_name);
        let new_set_file_name = format!("{}/{}.json", DATA_DIR_PATH, new_set_name);
        let err_msg = format!(
            "Failed to change name from [{}] to [{}]",
            prev_set_file_name, new_set_file_name
        );
        fs::rename(prev_set_file_name, new_set_file_name).expect(&err_msg);
    }

    pub fn update_set_file(set: StudySet) {
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

    pub fn delete_set_file(set_name: String) {
        let set_data_path = format!("{}/{}.json", DATA_DIR_PATH, set_name);
        let err_msg_delete = format!("Failed to delete set data file {}.json", set_name);
        fs::remove_file(set_data_path).expect(&err_msg_delete);
    }

    // Initialize files and folders to read/save data from/to
    pub fn set_up() {
        if !fs::metadata(&DATA_DIR_PATH).is_ok() {
            fs::create_dir(DATA_DIR_PATH).expect("Failed to Create Data Folder");
        }
        if !fs::metadata(&INVENTORY_DIR_PATH).is_ok() {
            fs::create_dir(INVENTORY_DIR_PATH).expect("Failed to Create Inventory Folder");
        }
        if !fs::metadata(&INVENTORY_FILE_PATH).is_ok() {
            File::create(INVENTORY_FILE_PATH).expect("Failed to Create Inventory File");
        }
    }

    pub fn clean_up_inventory() {}

    pub fn read_set_file(file_name: String) -> StudySet {
        let set_data_path = format!("{}/{}.json", DATA_DIR_PATH, file_name);
        let set_data = fs::read_to_string(set_data_path).expect("Failed to read set data file");
        let read_data: StudySet = serde_json::from_str(&set_data).expect("Error parsing set file");
        read_data
    }

    pub fn read_inventory_file() -> Catalogue {
        let data = fs::read_to_string(INVENTORY_FILE_PATH).expect("Failed to read inventory file");
        let catalogue: Catalogue = serde_json::from_str(&data).expect("Error parsing file");
        catalogue
    }

    pub fn update_inventory(inventory: Catalogue) {
        let data = serde_json::to_string_pretty(&inventory).expect("Error parsing data to json");
        let err_msg_open = format!("Failed to open file [{}]", INVENTORY_FILE_PATH);
        let err_msg_write = format!("Failed to write to file [{}]", INVENTORY_FILE_PATH);
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(INVENTORY_FILE_PATH)
            .expect(&err_msg_open);
        file.write_all(data.as_bytes()).expect(&err_msg_write);
    }
}
