# Advanced Calculator

A modern, precision-focused calculator built with **React, TypeScript, and Rust**. This project is a remastered version of my first JavaScript calculator, evolving from a simple two-number arithmetic tool into a more capable mathematical application.

The goal is to combine a responsive web interface with a reliable Rust-powered calculation engine, supporting exact arithmetic, arbitrary-precision calculations, and advanced mathematical operations.

## Features

### Mathematical Operations
- Addition
- Subtraction
- Multiplication
- Division
- Absolute value
- Square root, cube root, and nth root
- Integer and fractional powers
- Natural and common logarithms
- Exponential functions
- Trigonometric functions
- Inverse trigonometric functions
- Hyperbolic functions and their inverses
- Factorial
- Permutations and combinations
- Greatest common divisor (GCD)
- Least common multiple (LCM)
- Modulo
- Prime checking and factorization
- Floor, ceiling, rounding, and truncation
- Reciprocal and sign
- Minimum and maximum
- Percentage calculations

### Numerical Precision
- Arbitrary-precision integers and rational numbers
- Exact arithmetic where possible
- Arbitrary-precision decimal floating-point calculations
- Configurable calculation precision
- Explicit rounding behavior

## Technology Stack

### Frontend
- React
- TypeScript
- Tailwind CSS
- Vite

### Calculation Engine
- Rust
- Dashu - arbitrary-precision mathematics
- WebAssembly
- wasm-bindgen

### Planned Storage
- IndexedDB for browser-side persistence

## Architecture

The project follows a separation-of-concerns approach:

- **Frontend:** Handles the user interface and application interactions.
- **Calculation Engine:** Performs mathematical operations using Rust.
- **WebAssembly Adapter:** Exposes Rust functionality to the frontend.
- **Browser Storage:** Handles persistent application data.

The Rust calculation engine is designed to remain independent of browser-specific functionality, allowing it to be tested and potentially reused outside the web application.

## Project Structure

```text
.
├── apps/
│   └── web/                  # React frontend
├── crates/
│   ├── calculator-engine/    # Core Rust math engine
│   └── calculator-wasm/      # WebAssembly adapter
├── docs/                     # Project documentation
├── Cargo.toml                # Rust workspace
└── package.json              # Frontend dependencies and scripts
```

## Development

### Prerequisites
- Node.js
- npm
- Rust and Cargo
- wasm-pack
- wasm32-unknown-unknown Rust target

### Install Dependencies

```bash
npm install
```

### Run Development Server

```bash
npm run dev
```

### Build for Production

```bash
npm run build
```

### Run Rust Tests

```bash
cargo test -p calculator-engine
```

### Check WebAssembly Compatibility

```bash
cargo check -p calculator-engine --target wasm32-unknown-unknown
```

## Project Status

<text>Active development the Rust calculation engine and initial WebAssembly integration are underway. Expression parsing, the complete calculator interface, and browser persistence are planned development milestones.</text>

## Origin

This project started as my first JavaScript calculator, built to perform basic arithmetic operations between two numbers.

The remastered version is an opportunity to revisit that early project with a stronger focus on software architecture, numerical correctness, performance, and maintainability.

## License

MIT
