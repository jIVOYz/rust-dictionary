mod config;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs, io::Write, path::PathBuf};

#[derive(Deserialize, Serialize, Debug)]
pub struct Dictionary {
    pub list: Vec<Word>,
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary { list: vec![] }
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Dictionary, std::io::Error> {
        let contents = fs::read_to_string(path)?;

        match contents.is_empty() {
            false => return Ok(serde_json::from_str::<Dictionary>(&contents)?),
            true => {
                let dictionary = Self::new();
                return Ok(dictionary);
            }
        }
    }

    fn update_index(self: &mut Self) {
        for (idx, word) in self.list.iter_mut().enumerate() {
            word.index = idx + 1
        }
    }

    pub fn add_word(self: &mut Self, word: Word) {
        self.list.push(word);
        self.update_index();
    }

    pub fn remove_word(self: &mut Self, index: usize) -> Result<(), &'static str> {
        let id = self.list.iter().position(|t| t.index == index);
        match id {
            Some(id) => {
                self.list.remove(id);
            }
            None => return Err("Word not found"),
        }
        self.update_index();
        Ok(())
    }

    pub fn save(self: Self) {
        let data_file = config::data_file();
        let mut file = fs::File::create(&data_file).unwrap();
        let json = serde_json::to_string(&self).unwrap();

        let _ = file.write_all(&json.as_bytes());
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Word {
    pub index: usize,
    pub word: String,
    pub meaning: Vec<String>,
    pub example: Option<Vec<String>>,
}

impl Word {
    pub fn new(word: String, meaning: Vec<String>, example: Option<Vec<String>>) -> Self {
        Word {
            index: 10,
            word,
            meaning,
            example,
        }
    }
}
