# Tetris Clone in Rust

A simple Tetris clone implemented in Rust. This project demonstrates basic game development concepts such as rendering, user input handling, and game logic in Rust.

## Features

- Classic Tetris gameplay
- Block rotation and movement
- Line clearing mechanics
- Score tracking
- Simple graphical interface

## Requirements

- Rust (latest stable version recommended)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/tetris-rust.git
   cd tetris-rust
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

3. Run the game:

   ```sh
   cargo run
   ```

## Controls

- **Arrow Left/Right**: Move block left or right
- **Arrow Down**: Soft drop
- **Space**: Hard drop
- **Up Arrow**: Rotate block
- **P**: Pause game
- **Q**: Quit game

## Dependencies

This project uses the following Rust crates:

- `sdl2` for graphics and input handling
- `rand` for randomizing Tetrimino pieces

## Contributing

Pull requests are welcome! Please follow Rust's best practices and format your code with `cargo fmt` before submitting.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Acknowledgments

- Inspired by the original Tetris game by Alexey Pajitnov.
- Special thanks to the Rust community for their helpful libraries and documentation.



