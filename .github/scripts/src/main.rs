use std::{error::Error, fs, path::Path};

use clap::Parser;

mod cloc;
mod colors;
mod config;
mod readme;
mod stats;
mod svg;

fn main() -> Result<(), Box<dyn Error>> {
    let config = config::Config::parse();

    let json = if config.input == "-" {
        cloc::generate_stats(&config.exclude_dir, &config.exclude_lang)?
    } else {
        fs::read_to_string(&config.input)?
    };

    let stats = stats::parse_cloc_json(&json)?;

    let languages = stats::process_languages(stats.languages, config.max_rows);

    let svg = svg::render_svg(&languages);

    let output_path = Path::new(&config.output);

    if let Some(parent) = output_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)?;
    }

    fs::write(output_path, svg)?;

    readme::update_readme(Path::new(&config.readme), &config.output)?;

    println!("Language statistics updated successfully!");

    Ok(())
}
