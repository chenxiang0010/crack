mod constant;
mod encrypt;
pub mod util;

use crate::config::MobaXterm;
use encrypt::encrypt;

pub fn run(config: &MobaXterm) -> anyhow::Result<()> {
    encrypt(config)?;
    Ok(())
}
