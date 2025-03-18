# Advent of Code 2024 🎄

Welcome to my Advent of Code 2024 repository! This repository contains my solutions to the Advent of Code 2024 challenges, implemented in Rust.

## About Advent of Code

[Advent of Code](https://adventofcode.com/) is an annual set of Christmas-themed programming challenges that follow an Advent calendar. Each day from December 1 to December 25, a new challenge is released.

## About This Repository

This repository is my attempt at tackling the Advent of Code 2024 challenges using Rust. It serves as a personal learning journey to improve my Rust skills and have fun solving interesting problems.

## Project Structure

The repository is organized as follows:

```
aoc_2024/
├── src/
│   ├── day01.rs
│   ├── day02.rs
│   ├── ...
│   └── day25.rs
├── input/
│   ├── day01.txt
│   ├── day02.txt
│   ├── ...
│   └── day25.txt
├── tests/
│   ├── day01_test.rs
│   ├── day02_test.rs
│   ├── ...
│   └── day25_test.rs
├── Cargo.toml
└── README.md
```

- **src/**: Contains the Rust source files for each day's challenge.
- **input/**: Contains the input files for each day's challenge.
- **tests/**: Contains the test files for each day's challenge.
- **Cargo.toml**: The Cargo configuration file.

## How to Run

To run the solutions for a specific day, use the following command:

```bash
cargo run --bin dayXX
```

Replace `XX` with the day number (e.g., `01` for day 1).

## How to Test

To run the tests for a specific day, use the following command:

```bash
cargo test --test dayXX_test
```

Replace `XX` with the day number (e.g., `01` for day 1).

## Dependencies

This project uses the following Rust dependencies:

```toml
[dependencies]
```

Add any dependencies you use in your solutions here.

## Contributing

If you have any suggestions or improvements, feel free to open an issue or create a pull request. Contributions are always welcome!

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Happy coding and happy holidays! 🎅🎄
