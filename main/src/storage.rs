use druid::Data;
use serde::{ Deserialize, Serialize };
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

const DATA_PATH: &str = "./data/data.json";
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
    pub fn new(id: usize, word: String, ans: String, remark: String) -> Card {
        Card {
            id: id,
            word: word,
            ans: ans,
            remarks: remark,
        }
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
        }
        false
    }
}

impl StudySet {
    pub fn new(name: String, id: u16) {}

    pub fn add_card(&self, card: Card) {}

    pub fn get_card(&self) -> Card {
        self.cards[0].clone()
    }

    pub fn get_desc(&self) -> String {
        self.name.clone()
    }

    pub fn get_all_cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    pub fn get_num_of_cards(&self) -> usize {
        self.cards.len()
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
        let read_data: Vec<StudySet> = serde_json::from_str(&data_sets).unwrap();
        Storage {
            sets: read_data,
        }
    }

    pub fn get_all(&self) -> Vec<StudySet> {
        self.sets.clone()
    }

    pub fn get_study_set(&self, to_get: String) -> StudySet {
        for set in &self.sets {
            if to_get == set.name {
                return set.clone();
            }
        }
        StudySet {
            name: "Empty".to_string(),
            cards: vec![],
            id: 0,
        }
    }

    pub fn get_num_of_sets(&self) -> usize {
        self.sets.len()
    }

    pub fn save(&self) -> () {
        let data = serde_json::to_string(&self.sets).expect("Error parsing data to json");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(DATA_PATH)
            .expect("Error opening data file");
        file.write_all(data.as_bytes()).expect("Error writing to file")
    }

    pub fn read() -> String {
        let data = match fs::read_to_string(DATA_PATH) {
            Ok(content) => content,
            Err(_) => {
                println!("No data found, creating data file....");
                fs::create_dir(DATA_DIR_PATH).expect("Failed to create directory for data");
                File::create(DATA_PATH).expect("Failed to create data file");
                fs::read_to_string(DATA_PATH).expect("Failed to read created data file")
            }
        };
        data
    }
}
