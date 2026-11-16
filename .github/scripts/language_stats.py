#!/usr/bin/env python3
"""Build an SVG language chart from cloc JSON and refresh the README block."""

import argparse
import html
import json
import pathlib
import re
import sys
import tempfile

ROOT_DIR = pathlib.Path(__file__).resolve().parents[2]

STATS = ROOT_DIR / "stats.json"
README = ROOT_DIR / "README.md"
SVG_PATH = ROOT_DIR / "docs" / "language-stats.svg"

START = "<!-- LANG_STATS_START -->"
END = "<!-- LANG_STATS_END -->"

# GitHub Linguist colours, keyed by cloc language names.
COLORS = {
    "Rust": "#dea584",
    "TypeScript": "#3178c6",
    "JavaScript": "#f1e05a",
    "JSX": "#f1e05a",
    "CSS": "#663399",
    "HTML": "#e34c26",
    "YAML": "#cb171e",
    "Python": "#3572a5",
    "Bourne Shell": "#89e051",
    "Dockerfile": "#384d54",
    "PostCSS": "#dc3a0c",
    "SQL": "#e38c00",
    "Go": "#00add8",
    "C": "#555555",
    "C++": "#f34b7d",
}

FALLBACKS = [
    "#8250df",
    "#bf3989",
    "#1a7f37",
    "#bc4c00",
    "#0969da",
    "#9a6700",
    "#cf222e",
]

OTHER_COLOR = "#8c959f"
MAX_ROWS = 8

W, PAD = 480, 20

BAR_X = 135
BAR_W = 205
BAR_H = 10

FIRST_ROW = 60
ROW_H = 36


def load_rows():
    """Load and validate cloc JSON statistics."""

    if not STATS.is_file():
        sys.exit(f"Statistics file not found: {STATS}")

    try:
        data = json.loads(STATS.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        sys.exit(f"Invalid JSON in {STATS}: {exc}")
    except OSError as exc:
        sys.exit(f"Could not read {STATS}: {exc}")

    if not isinstance(data, dict) or "SUM" not in data:
        sys.exit("cloc found no countable files")

    total_data = data["SUM"]

    if not isinstance(total_data, dict):
        sys.exit("Invalid SUM entry in cloc JSON")

    total = total_data.get("code")

    if not isinstance(total, int) or isinstance(total, bool):
        sys.exit("Invalid total code count in cloc JSON")

    if total <= 0:
        sys.exit("No code lines found")

    rows = []

    for name, values in data.items():
        if name in ("header", "SUM"):
            continue

        if not isinstance(values, dict):
            sys.exit(f"Invalid statistics entry for language: {name}")

        files = values.get("nFiles")
        code = values.get("code")

        if (
            not isinstance(files, int)
            or isinstance(files, bool)
            or not isinstance(code, int)
            or isinstance(code, bool)
            or files < 0
            or code < 0
        ):
            sys.exit(f"Invalid file or code count for language: {name}")

        if code > 0:
            rows.append((name, files, code))

    rows.sort(key=lambda row: -row[2])

    return rows, total


def chart_rows(rows):
    """Keep the top languages and group the remaining languages into Other."""

    if len(rows) > MAX_ROWS:
        tail = rows[MAX_ROWS - 1:]

        rows = rows[:MAX_ROWS - 1] + [
            (
                "Other",
                sum(row[1] for row in tail),
                sum(row[2] for row in tail),
            )
        ]

    out = []

    for i, (name, files, code) in enumerate(rows):
        if name == "Other":
            color = OTHER_COLOR
        else:
            color = COLORS.get(name, FALLBACKS[i % len(FALLBACKS)])

        out.append((name, files, code, color))

    return out



def render_svg(rows, total):
    """Generate a GitHub-dark SVG language chart."""

    height = FIRST_ROW + len(rows) * ROW_H + 4
    full = W - 2 * PAD

    p = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{height}" '
        f'viewBox="0 0 {W} {height}" role="img" aria-label="Language breakdown">',

        "<style>",
        'text{font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Helvetica,Arial,sans-serif}',
        ".name{fill:#e6edf3;font-size:13px;font-weight:600}",
        ".muted{fill:#8b949e;font-size:12px}",
        ".track{fill:#30363d}",
        "</style>",

        # Background
        f'<rect width="{W}" height="{height}" fill="#0d1117"/>',

        # Rounded overview bar
        f'<defs><clipPath id="round"><rect x="{PAD}" y="20" '
        f'width="{full}" height="12" rx="6"/></clipPath></defs>',

        '<g clip-path="url(#round)">',
    ]

    # Stacked overview bar
    x = float(PAD)

    for _, _, code, color in rows:
        width = full * code / total

        p.append(
            f'<rect x="{x:.2f}" y="20" width="{width:.2f}" '
            f'height="12" fill="{color}"/>'
        )

        x += width

    p.append("</g>")

    # Individual language progress bars
    for i, (name, _, code, color) in enumerate(rows):
        y = FIRST_ROW + i * ROW_H
        pct = code / total * 100

        fill_w = max(BAR_W * code / total, 4)
        rx = min(5, fill_w / 2)

        p += [
            f'<circle cx="{PAD + 5}" cy="{y + 5}" r="5" fill="{color}"/>',

            f'<text class="name" x="{PAD + 18}" y="{y + 10}">'
            f'{html.escape(name)}</text>',

            f'<rect class="track" x="{BAR_X}" y="{y}" '
            f'width="{BAR_W}" height="{BAR_H}" rx="5"/>',

            f'<rect x="{BAR_X}" y="{y}" width="{fill_w:.2f}" '
            f'height="{BAR_H}" rx="{rx:.2f}" fill="{color}"/>',

            f'<text class="name" x="{BAR_X + BAR_W + 14}" '
            f'y="{y + 10}">{pct:.1f}%</text>',

            f'<text class="muted" x="{W - PAD}" y="{y + 10}" '
            f'text-anchor="end">{code:,} lines</text>',
        ]

    p.append("</svg>")

    return "\n".join(p) + "\n"

def build_readme_block():
    """Build the README language statistics section."""

    return (
        f"{START}\n"
        f'<p align="center"><img src="docs/language-stats.svg" '
        f'alt="Language breakdown" width="{W}"></p>\n'
        f"{END}"
    )


def update_readme(block, check=False):
    """Replace exactly one statistics block in README."""

    if not README.is_file():
        sys.exit(f"README file not found: {README}")

    try:
        content = README.read_text(encoding="utf-8")
    except OSError as exc:
        sys.exit(f"Could not read README: {exc}")

    pattern = re.compile(
        re.escape(START) + r".*?" + re.escape(END),
        re.DOTALL,
    )

    matches = list(pattern.finditer(content))

    if len(matches) != 1:
        sys.exit(
            f"Expected exactly one statistics block, found {len(matches)}"
        )

    updated = pattern.sub(lambda _: block, content)

    if check:
        if updated != content:
            sys.exit("README statistics are outdated. Run the script to update.")
        return

    if updated != content:
        atomic_write(README, updated)


def atomic_write(path, content):
    """Write a file atomically using a temporary file in the same directory."""

    path.parent.mkdir(parents=True, exist_ok=True)

    temp_path = None

    try:
        with tempfile.NamedTemporaryFile(
            mode="w",
            encoding="utf-8",
            newline="\n",
            dir=path.parent,
            delete=False,
        ) as temp:
            temp_path = pathlib.Path(temp.name)
            temp.write(content)

        temp_path.replace(path)

    except OSError:
        if temp_path is not None:
            temp_path.unlink(missing_ok=True)
        raise


def main():
    parser = argparse.ArgumentParser(
        description="Generate language statistics from cloc JSON."
    )

    parser.add_argument(
        "--check",
        action="store_true",
        help="Check whether README statistics are up to date.",
    )

    args = parser.parse_args()

    rows, total = load_rows()

    svg = render_svg(chart_rows(rows), total)
    block = build_readme_block()

    if args.check:
        update_readme(block, check=True)

        if not SVG_PATH.is_file() or SVG_PATH.read_text(
            encoding="utf-8"
        ) != svg:
            sys.exit("SVG chart is outdated. Run the script to update.")

        print("Language statistics are up to date.")
        return

    try:
        atomic_write(SVG_PATH, svg)
        update_readme(block)
    except OSError as exc:
        sys.exit(f"Could not write generated files: {exc}")

    print("Language statistics updated successfully.")


if __name__ == "__main__":
    main()
