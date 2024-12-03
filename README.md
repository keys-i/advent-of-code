# Advent of Code
[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/keys-i/advent-of-code) [![CodSpeed](https://github.com/keys-i/advent-of-code/actions/workflows/codspeed.yml/badge.svg)](https://github.com/keys-i/advent-of-code/actions/workflows/codspeed.yml)

This repository contains solutions for the [Advent of Code](https://adventofcode.com/) challenges, implemented in Rust. It includes benchmarking capabilities using [CodSpeed](https://codspeed.io/) to ensure optimal performance of the solutions.

## Project Structure

```plaintext
advent-of-code
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── benches
│   └── year2024
│       └── year2024.rs
├── input
│   └── 2024
│       └── day1.txt
└── src
    ├── lib.rs
    ├── main.rs
    └── year2024
        ├── day1.rs
        ├── day2.rs
        └── mod.rs
```

- **Cargo.lock & Cargo.toml**: Manage dependencies and project metadata.
- **LICENSE**: Project's licensing information.
- **README.md**: This file.
- **benches/**: Contains benchmarking scripts.
- **input/**: Stores input data for each day's challenge.
- **src/**: Source code for solutions.

## Getting Started

### Prerequisites

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/advent-of-code.git
   cd advent-of-code
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```

### Usage

To run the solutions for a specific day:
```bash
cargo run --bin day1
```

Replace `day1` with the desired day's solution.

## Benchmarking

This project utilizes [CodSpeed](https://codspeed.io/) for benchmarking. To run benchmarks:

1. Install `cargo-codspeed`:
   ```bash
   cargo install cargo-codspeed --locked
   ```
2. Build the benchmarks:
   ```bash
   cargo codspeed build
   ```
3. Run the benchmarks:
   ```bash
   cargo codspeed run
   ```

For more information, refer to the [CodSpeed documentation](https://docs.codspeed.io/benchmarks/rust).

## Benchmark Results
<!-- BENCHMARK_RESULTS -->
### Year 2024

| Day Number | Part 1 Time (µs) | Part 2 Time (µs) | Total Time (µs) |
|------------|------------------|------------------|-----------------|
| 1          | 51.860           | 46.532           | 98.39           |

<!-- END_BENCHMARK_RESULTS -->

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Advent of Code](https://adventofcode.com/) for the challenges.
- [CodSpeed](https://codspeed.io/) for benchmarking tools.
