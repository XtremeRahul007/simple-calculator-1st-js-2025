use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "github-language-stats",
    version,
    about = "Generate language statistics SVG from cloc JSON"
)]
pub struct Config {
    #[arg(short, long, default_value = "stats.json")]
    pub input: String,

    #[arg(short, long, default_value = "docs/language-stats.svg")]
    pub output: String,

    #[arg(short, long, default_value = "README.md")]
    pub readme: String,

    #[arg(
        short,
        long,
        default_value_t = 8,
        value_parser = |s: &str| -> Result<usize, String> {
            let val: usize = s.parse().map_err(|e| format!("{e}"))?;
            if val >= 1 {
                Ok(val)
            } else {
                Err(String::from("max_rows must be at least 1"))
            }
        }
    )]
    pub max_rows: usize,

    #[arg(long, value_delimiter = ',')]
    pub exclude_dir: Vec<String>,

    #[arg(long, value_delimiter = ',')]
    pub exclude_lang: Vec<String>,
}
