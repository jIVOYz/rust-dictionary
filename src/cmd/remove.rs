use anyhow::{bail, Result};
use di::Dictionary;

use crate::{
    cmd::{Remove, Run},
    config,
};

impl Run for Remove {
    fn run(self: Self) -> Result<()> {
        let mut dictionary =
            Dictionary::load_from_file(&config::data_file()).expect("failed to read data file");

        if let Err(err) = dictionary.remove_word(&self.id) {
            bail!("{err}");
        };

        dictionary.save();
        Ok(())
    }
}
