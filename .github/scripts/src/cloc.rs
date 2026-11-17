use std::{error::Error, process::Command};

pub fn generate_stats(
    exclude_dirs: &[String],
    exclude_langs: &[String],
) -> Result<String, Box<dyn Error>> {
    let mut command = Command::new("cloc");

    command.arg(".");

    if !exclude_dirs.is_empty() {
        command.arg(format!("--exclude-dir={}", exclude_dirs.join(",")));
    }

    if !exclude_langs.is_empty() {
        command.arg(format!("--exclude-lang={}", exclude_langs.join(",")));
    }

    command.arg("--json");

    let output = command.output()?;

    if !output.status.success() {
        return Err(format!("cloc failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}
