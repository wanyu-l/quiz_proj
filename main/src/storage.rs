use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

const DATA_PATH: &str = "./data/data.json";
const DATA_DIR_PATH: &str = "./data";

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    word: String,
    ans: String,
    remarks: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StudySet {
    name: String,
    cards: Vec<Card>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Storage {
    sets: Vec<StudySet>,
}

impl Storage {
    pub fn new() -> Storage {
        let dummy_card_1 = Card {
            word: String::from("楽しい"),
            ans: String::from("たのしい"),
            remarks: String::from(""),
        };
        let dummy_card_2 = Card {
            word: String::from("難しい"),
            ans: String::from("むずかしい"),
            remarks: String::from(""),
        };
        let study_set_1 = StudySet {
            name: String::from("L1"),
            cards: vec![dummy_card_1, dummy_card_2],
        };
        let dummy_card_3 = Card {
            word: String::from("美しい"),
            ans: String::from("うつくしい"),
            remarks: String::from(""),
        };
        let dummy_card_4 = Card {
            word: String::from("愛してる"),
            ans: String::from("あいしてる"),
            remarks: String::from(""),
        };
        let study_set_2 = StudySet {
            name: String::from("L2"),
            cards: vec![dummy_card_3, dummy_card_4],
        };
        let data_sets = Storage::read();
        println!("read:[{}]", data_sets);
        Storage {
            sets: vec![study_set_1, study_set_2],
        }
    }

    pub fn save(&self) -> () {
        let data = serde_json::to_string(&self.sets).expect("Error parsing data to json");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(DATA_PATH)
            .expect("Error opening data file");
        file.write_all(data.as_bytes())
            .expect("Error writing to file")
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
