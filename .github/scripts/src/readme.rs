use std::fs;
use std::io;
use std::path::Path;

const START: &str = "<!-- LANG_STATS_START -->";
const END: &str = "<!-- LANG_STATS_END -->";

pub fn update_readme(path: &Path, image_path: &str) -> io::Result<()> {
    let content = fs::read_to_string(path)?;

    let starts = find_markers(&content, START);
    let ends = find_markers(&content, END);

    if starts.len() != 1 || ends.len() != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "README must contain exactly one start and end marker outside code blocks",
        ));
    }

    let (start, _) = starts[0];
    let (end, end_line_end) = ends[0];

    if start >= end {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "README markers are incorrectly ordered",
        ));
    }

    let replacement = format!(
        "{START}\n<p align=\"center\"><img src=\"{image_path}\" alt=\"Language breakdown\" width=\"480\"></p>\n{END}\n"
    );

    let updated = format!(
        "{}{}{}",
        &content[..start],
        replacement,
        &content[end_line_end..]
    );

    let temp_path = path.with_extension("tmp");
    fs::write(&temp_path, updated)?;
    fs::rename(&temp_path, path)?;

    Ok(())
}

// Returns (line_start, line_end) for matching markers outside fenced code blocks.
fn find_markers(content: &str, marker: &str) -> Vec<(usize, usize)> {
    let mut markers = Vec::new();
    let mut fence: Option<(char, usize)> = None;
    let mut offset = 0;

    for line in content.split_inclusive('\n') {
        let trimmed_line = line.trim();

        if let Some((fence_char, fence_length)) = fence {
            if let Some((character, length, trailing)) = parse_fence(line)
                && character == fence_char
                && length >= fence_length
                && trailing.trim().is_empty()
            {
                fence = None;
            }
        } else if let Some((character, length, _)) = parse_fence(line) {
            fence = Some((character, length));
        } else if trimmed_line == marker {
            markers.push((offset, offset + line.len()));
        }

        offset += line.len();
    }

    markers
}

// Detects Markdown fenced code block delimiters.
fn parse_fence(line: &str) -> Option<(char, usize, &str)> {
    let line = line.trim_end_matches(&['\r', '\n'][..]);

    let indentation = line.len() - line.trim_start().len();

    if indentation > 3 {
        return None;
    }

    let trimmed = line.trim_start();
    let character = trimmed.chars().next()?;

    if character != '`' && character != '~' {
        return None;
    }

    let length = trimmed.chars().take_while(|&c| c == character).count();

    if length < 3 {
        return None;
    }

    let trailing = &trimmed[length..];

    // A backtick fence's info string cannot contain backticks.
    if character == '`' && trailing.contains('`') {
        return None;
    }

    Some((character, length, trailing))
}
