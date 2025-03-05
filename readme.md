# Tetris Clone in Rust

A simple Tetris clone implemented in Rust. This project demonstrates basic game development concepts such as rendering, user input handling, and game logic in Rust.

## Features

- Classic Tetris gameplay
- Block rotation and movement
- Line clearing mechanics
- Score tracking
- Simple graphical interface

## Requirements

To use this project, ensure you have the following installed:

- **Rust** (latest stable version recommended)
- **Cargo** (Rust's package manager)

### Installation

Follow the official Rust and Cargo installation guide:

📖 [Rust and Cargo Installation Docs](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Tetris Installation

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

## Key Bindings and Their Functions

| Key           | Function                                                        |
|--------------|------------------------------------------------------------------|
| ⬅ Left Arrow  | Move the tetromino left.                                       |
| ➡ Right Arrow | Move the tetromino right.                                      |
| ⬆ Up Arrow    | Hard drop (instantly drops the piece to the lowest position).  |
| ⬇ Down Arrow  | Soft drop (accelerates fall speed while held).                 |
| Z            | Rotate the tetromino counterclockwise.                          |
| X            | Rotate the tetromino clockwise.                                 |
| C            | Hold piece (swap the current tetromino with the hold slot).     |
| Enter        | Pause/Resume the game.                                          |
| Space        | Start a new game (when not running).                            |
| N            | Change song (cycle through embedded MP3s).                      |
| M            | Mute/unmute music.                                              |

## Dependencies

This project uses the following Rust crates:

- `macroquad` game development library for creating 2D games
- `rand` for randomizing Tetrimino pieces
- `rodio` for playing audio files and streams

## Contributing

Pull requests are welcome! Please follow Rust's best practices and format your code with `cargo fmt` before submitting.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Acknowledgments

- Inspired by the original Tetris game by Alexey Pajitnov.
- Special thanks to the Rust community for their helpful libraries and documentation.



