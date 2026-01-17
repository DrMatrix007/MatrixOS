use anyhow::Result;

use crate::runner::get_target_dir;

pub fn clean_taret() -> Result<()> {
    std::fs::remove_dir_all(get_target_dir()?)?;
    Ok(())
}
