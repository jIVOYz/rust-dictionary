use anyhow::{bail, Result};
use di::Dictionary;

use crate::cmd::{Remove, Run};

impl Run for Remove {
    fn run(self: Self) -> Result<()> {
        let mut dictionary = Dictionary::load()?;

        if let Err(err) = dictionary.remove_word(&self.id) {
            bail!("{err}");
        };

        dictionary.save()?;
        Ok(())
    }
}
