# TheSnakeGame

A simple snake game written in Rust.

# Libraries
- [`bracket-terminal`](https://crates.io/crates/bracket-terminal)
    - Used for rendering the game.
- [`rand`](https://crates.io/crates/rand)
    - Used for generating random numbers.

Cross-compilation was tested on macOS -> Windows.

# Compiling
- Install Rust (https://rustlang.org)
- Simply run `cargo build --release` in the root directory of the project.
    - Note that you should build in release mode for maximum performance. Only use debug mode for testing.
    - The executable will be located in `target/release/thesnakegame.exe`