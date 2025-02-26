mod constant;
mod encrypt;
pub mod util;

use crate::config::MobaXterm;

pub fn run(config: &MobaXterm) -> anyhow::Result<()> {
    encrypt::encrypt(config)?;
    Ok(())
}
