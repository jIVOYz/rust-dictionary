use std::{fs, path::PathBuf};

/// Returns path to the data directory
/// If does not exists will create a new directiory
pub fn data_dir() -> PathBuf {
    let path = dirs::data_dir().expect("not found data dir");

    match path.join("dictionary").exists() {
        true => return path.join("dictionary"),
        false => {
            let _ = fs::create_dir(path.join("dictionary"));
            return path.join("dictionary");
        }
    }
}

pub fn data_file() -> PathBuf {
    let data_dir = data_dir();

    match data_dir.join("dictionary.json").exists() {
        true => return data_dir.join("dictionary.json"),
        false => {
            let _ = fs::File::create_new(data_dir.join("dictionary.json"))
                .expect("failed to create file");
            return data_dir.join("dictionary.json");
        }
    }
}
