# Advanced Calculator

A modern, precision-focused calculator built with **React, TypeScript, and Rust**. This project is a remastered version of my first JavaScript calculator, evolving from a simple two-number arithmetic tool into a more capable mathematical application.

The goal is to combine a responsive web interface with a reliable Rust-powered calculation engine, supporting exact arithmetic, arbitrary-precision calculations, and advanced mathematical operations.

## Features

### Mathematical Operations

- Addition, subtraction, multiplication, and division
- Absolute value, reciprocal, and sign
- Square root, cube root, and nth root
- Integer and fractional powers
- Natural and common logarithms
- Exponential functions
- Trigonometric functions and their inverses
- Hyperbolic functions and their inverses
- Factorial, permutations, and combinations
- Greatest common divisor (GCD) and least common multiple (LCM)
- Modulo
- Prime checking and factorization
- Floor, ceiling, rounding, and truncation
- Minimum and maximum
- Percentage calculations

### Numerical Precision

- Arbitrary-precision integers and rational numbers
- Exact arithmetic where possible
- Arbitrary-precision decimal floating-point calculations
- Configurable calculation precision
- Explicit rounding behavior

## Technology Stack

| Layer | Technology |
| :--- | :--- |
| Frontend | React, TypeScript, Tailwind CSS, Vite |
| Calculation engine | Rust, [Dashu](https://github.com/cmpute/dashu) (arbitrary-precision math) |
| Bridge | WebAssembly, wasm-bindgen |
| Storage (planned) | IndexedDB |

## Architecture

The project follows a separation-of-concerns approach:

- **Frontend:** handles the user interface and application interactions.
- **Calculation engine:** performs mathematical operations in Rust.
- **WebAssembly adapter:** exposes Rust functionality to the frontend.
- **Browser storage:** handles persistent application data.

The Rust calculation engine is designed to remain independent of browser-specific functionality, so it can be tested on its own and potentially reused outside the web application.

## Project Structure

```text
.
├── web/                      # React frontend
├── crates/
│   ├── calculator-engine/    # Core Rust math engine
│   └── calculator-wasm/      # WebAssembly adapter
├── docs/                     # Project documentation
├── Cargo.toml                # Rust workspace
└── package.json              # Frontend dependencies and scripts
```

## Development

### Prerequisites

- Node.js and npm
- Rust and Cargo
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- The `wasm32-unknown-unknown` Rust target (`rustup target add wasm32-unknown-unknown`)

### Common Commands

| Task | Command |
| :--- | :--- |
| Install dependencies | `npm install` |
| Start the dev server | `npm run dev` |
| Build for production | `npm run build` |
| Run Rust tests | `cargo test -p calculator-engine` |
| Check WebAssembly compatibility | `cargo check -p calculator-engine --target wasm32-unknown-unknown` |

## Project Status

**In progress**

- Rust calculation engine
- Initial WebAssembly integration

**Planned**

- Complete calculator interface
- Browser persistence with IndexedDB

## Language Breakdown

> This table is generated automatically by a GitHub Actions workflow on every push to the `remastered` branch.

<!-- LANG_STATS_START -->
| Language | Files | Lines of code | Share |
| :--- | ---: | ---: | ---: |
| JSON | 6 | 2829 | 51.7% |
| Rust | 16 | 2356 | 43% |
| TypeScript | 4 | 107 | 2% |
| Markdown | 2 | 104 | 1.9% |
| TOML | 3 | 31 | 0.6% |
| SVG | 2 | 25 | 0.5% |
| HTML | 1 | 13 | 0.2% |
| CSS | 1 | 8 | 0.1% |
| **Total** | **35** | **5473** | **100%** |
<!-- LANG_STATS_END -->

## Origin

This project started as my first JavaScript calculator, built to perform basic arithmetic operations between two numbers.

The remastered version is an opportunity to revisit that early project with a stronger focus on software architecture, numerical correctness, performance, and maintainability.

## License

MIT
