use crate::{colors::language_color, stats::LanguageEntry};

const WIDTH: u32 = 480;
const PADDING: u32 = 20;

const BAR_X: u32 = 145;
const BAR_WIDTH: u32 = 165;
const BAR_HEIGHT: u32 = 10;

const PERCENTAGE_X: u32 = 325;
const CODE_COUNT_X: u32 = WIDTH - PADDING;

const STACKED_BAR_X: u32 = PADDING;
const STACKED_BAR_Y: u32 = 85;
const STACKED_BAR_WIDTH: u32 = WIDTH - (PADDING * 2);
const STACKED_BAR_HEIGHT: u32 = 12;

const HEADER_HEIGHT: u32 = 120;
const ROW_HEIGHT: u32 = 36;

const SEPARATOR_COLOR: &str = "#21262d";

pub fn render_svg(entries: &[LanguageEntry]) -> String {
    let height = HEADER_HEIGHT + entries.len() as u32 * ROW_HEIGHT + PADDING;

    let total_code: u64 = entries
        .iter()
        .map(|entry| u64::from(entry.stats.code))
        .sum();

    let mut svg = String::new();

    // SVG root
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{height}" viewBox="0 0 {WIDTH} {height}">"#
    ));

    // Background
    svg.push_str(&format!(
        r##"<rect width="{WIDTH}" height="{height}" rx="8" fill="#0d1117"/>"##
    ));

    // Styles
    svg.push_str(
        r#"<style>
            text {
                font-family: Inter, -apple-system, BlinkMacSystemFont,
                             "Segoe UI", sans-serif;
            }

            .title {
                fill: #e6edf3;
                font-size: 17px;
                font-weight: 700;
            }

            .name {
                fill: #e6edf3;
                font-size: 13px;
                font-weight: 600;
            }

            .muted {
                fill: #8b949e;
                font-size: 12px;
                font-weight: 400;
            }

            .track {
                fill: #30363d;
            }
        </style>"#,
    );

    // Header
    svg.push_str(&format!(
        r#"<text x="{PADDING}" y="32" class="title">Language Statistics</text>"#
    ));

    svg.push_str(&format!(
        r#"<text x="{PADDING}" y="52" class="muted">Breakdown by source code</text>"#
    ));

    svg.push_str(&format!(
        r#"<text x="{}" y="32" text-anchor="end" class="muted">{total_code} lines</text>"#,
        WIDTH - PADDING
    ));

    render_stacked_bar(&mut svg, entries);

    // Empty state
    if entries.is_empty() {
        svg.push_str(&format!(
            r#"<text x="{}" y="100" text-anchor="middle" class="muted">No language statistics available</text>"#,
            WIDTH / 2
        ));
    }

    // Language rows
    for (index, entry) in entries.iter().enumerate() {
        let y = HEADER_HEIGHT + index as u32 * ROW_HEIGHT;

        let percentage = if entry.percentage.is_finite() {
            entry.percentage.clamp(0.0, 100.0)
        } else {
            0.0
        };

        let bar_width = BAR_WIDTH as f64 * percentage / 100.0;

        let color = language_color(&entry.name);
        let display_name = truncate_name(&entry.name, 16);
        let escaped_name = escape_xml(&display_name);

        if index < entries.len() - 1 {
            svg.push_str(&format!(
                r#"<line
                    x1="{PADDING}"
                    y1="{}"
                    x2="{}"
                    y2="{}"
                    stroke="{SEPARATOR_COLOR}"
                    stroke-width="1"
                />"#,
                y + 12,
                WIDTH - PADDING,
                y + 12
            ));
        }

        // Language color indicator
        svg.push_str(&format!(
            r#"<circle
                cx="{}"
                cy="{}"
                r="4"
                fill="{color}"
            />"#,
            PADDING + 4,
            y - 4
        ));

        // Language name
        svg.push_str(&format!(
            r#"<text x="{}" y="{y}" class="name"><title>{}</title>{escaped_name}</text>"#,
            PADDING + 14,
            escape_xml(&entry.name)
        ));

        // Background track
        svg.push_str(&format!(
            r#"<rect x="{BAR_X}" y="{}" width="{BAR_WIDTH}" height="{BAR_HEIGHT}" rx="5" class="track"/>"#,
            y - 10
        ));

        // Language-colored bar
        svg.push_str(&format!(
            r#"<defs>
                <clipPath id="language-bar-clip-{index}">
                    <rect
                        x="{BAR_X}"
                        y="{}"
                        width="{BAR_WIDTH}"
                        height="{BAR_HEIGHT}"
                        rx="5"
                    />
                </clipPath>
            </defs>
            <rect
                x="{BAR_X}"
                y="{}"
                width="{bar_width:.2}"
                height="{BAR_HEIGHT}"
                fill="{color}"
                clip-path="url(#language-bar-clip-{index})"
            />"#,
            y - 10,
            y - 10
        ));

        // Percentage
        svg.push_str(&format!(
            r#"<text x="{PERCENTAGE_X}" y="{y}" class="name">{percentage:.1}%</text>"#
        ));

        // Code count
        svg.push_str(&format!(
            r#"<text x="{CODE_COUNT_X}" y="{y}" text-anchor="end" class="muted">{} lines</text>"#,
            entry.stats.code
        ));
    }

    svg.push_str("</svg>");

    svg
}

fn render_stacked_bar(svg: &mut String, entries: &[LanguageEntry]) {
    svg.push_str(&format!(
        r#"<defs>
            <clipPath id="stacked-bar-clip">
                <rect
                    x="{STACKED_BAR_X}"
                    y="{STACKED_BAR_Y}"
                    width="{STACKED_BAR_WIDTH}"
                    height="{STACKED_BAR_HEIGHT}"
                    rx="6"
                />
            </clipPath>
        </defs>"#
    ));

    svg.push_str(r#"<g clip-path="url(#stacked-bar-clip)">"#);

    let mut current_x = STACKED_BAR_X as f64;

    for entry in entries {
        let percentage = if entry.percentage.is_finite() {
            entry.percentage.clamp(0.0, 100.0)
        } else {
            0.0
        };

        let width = STACKED_BAR_WIDTH as f64 * percentage / 100.0;
        let color = language_color(&entry.name);

        if width > 0.0 {
            svg.push_str(&format!(
                r##"<rect
                    x="{current_x:.2}"
                    y="{STACKED_BAR_Y}"
                    width="{width:.2}"
                    height="{STACKED_BAR_HEIGHT}"
                    fill="{color}"
                />"##
            ));
        }

        current_x += width;
    }

    svg.push_str("</g>");

    // Draw the border after clipping the colored segments.
    svg.push_str(&format!(
        r##"<rect
            x="{STACKED_BAR_X}"
            y="{STACKED_BAR_Y}"
            width="{STACKED_BAR_WIDTH}"
            height="{STACKED_BAR_HEIGHT}"
            rx="6"
            fill="none"
            stroke="#30363d"
        />"##
    ));
}

fn truncate_name(name: &str, max_chars: usize) -> String {
    let mut chars = name.chars();

    let truncated: String = chars.by_ref().take(max_chars).collect();

    if chars.next().is_some() {
        format!("{truncated}…")
    } else {
        truncated
    }
}

fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
