mod config;
use anyhow::{bail, Result};
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

    /// Load `Dictionary` from `$XDG_DATA_HOME`.
    /// Will return `Dictionary::new()` if `dictionary.json` is empty
    pub fn load() -> Result<Dictionary> {
        let contents = fs::read_to_string(config::data_file())?;

        match contents.is_empty() {
            false => return Ok(serde_json::from_str::<Dictionary>(&contents)?),
            true => {
                let dictionary = Self::new();
                return Ok(dictionary);
            }
        }
    }

    /// Load `Dictionary` from `path`.
    pub fn load_from_file(path: &PathBuf) -> Result<Dictionary> {
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

    pub fn remove_word(self: &mut Self, index: &[usize]) -> Result<()> {
        for idx in index {
            let id = self.list.iter().position(|t| t.index == *idx);

            if let Some(i) = id {
                self.list.remove(i);
            } else {
                bail!("word does not exist");
            }
        }
        self.update_index();
        Ok(())
    }

    fn search_by_name(self: &Self, query: &str) -> Vec<&Word> {
        let results: Vec<&Word> = self
            .list
            .iter()
            .filter(|word| word.name.contains(&query.to_string()))
            .collect();

        return results;
    }

    fn search_by_definition(self: &Self, query: &str) -> Vec<&Word> {
        let results = self
            .list
            .iter()
            .filter(|word| word.definition.join(", ").contains(query))
            .collect();

        return results;
    }

    pub fn search_word(self: &Self, query: &str) -> Option<Vec<&Word>> {
        let by_name = self.search_by_name(query);
        let by_definition = self.search_by_definition(query);

        let results = vec![by_name, by_definition].concat();

        if !results.is_empty() {
            return Some(results);
        } else {
            return None;
        }
    }

    pub fn save(self: Self) -> Result<()> {
        let data_file = config::data_file();
        let mut file = fs::File::create(&data_file)?;
        let json = serde_json::to_string(&self)?;

        file.write_all(&json.as_bytes())?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Word {
    pub index: usize,
    pub name: String,
    pub definition: Vec<String>,
    pub example: Option<Vec<String>>,
}

impl Word {
    pub fn new(word: String, definition: Vec<String>, example: Option<Vec<String>>) -> Self {
        Word {
            index: 10,
            name: word,
            definition,
            example,
        }
    }
}
