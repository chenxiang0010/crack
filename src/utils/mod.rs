use anyhow::Context;
use std::fs;
use std::path::Path;

fn init_output_dir() -> anyhow::Result<()> {
    create_dirs(&["output/jetbrains", "output/mobaxterm"])
}

fn create_dirs<P: AsRef<Path>>(dirs: &[P]) -> anyhow::Result<()> {
    for dir in dirs {
        fs::create_dir_all(dir)
            .with_context(|| format!("Failed to create {} directory", dir.as_ref().display()))?;
    }
    Ok(())
}

fn init_config_file() -> anyhow::Result<()> {
    let config_file = Path::new("config.json");
    if !config_file.exists() {
        let content = r#"{
  "mobaxterm": {
    "username": "test",
    "version": "23.1",
    "licenseType": "Professional",
    "count": 1
  },
  "jetbrains": {
    "licenseeName": "test",
    "assigneeName": "test",
    "expireAt": "2029-01-01",
    "updateCode": false
  }
}"#;
        fs::write(config_file, content.as_bytes())?;
    }
    Ok(())
}

pub fn init() -> anyhow::Result<()> {
    init_output_dir()?;
    init_config_file()?;
    Ok(())
}
