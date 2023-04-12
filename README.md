# GLSweeper-RS, a Rust implementation of Minesweeper using OpenGL

## Minesweeper clone written in Rust using OpenGL for rendering

![Screenshot_20230319_184629](https://user-images.githubusercontent.com/90425800/226214486-a6f50905-2f5b-4d10-a72d-5977c92c136b.png)

## How to play

Mines are hidden in the grid. Click on a tile to reveal it. If it is a mine, you lose. If it is not a mine, the number on the tile indicates how many mines are in the 8 tiles around it. If you reveal all tiles that are not mines, you win.

- Left click to reveal a tile
- Right click to flag a tile
- Left click on a number to reveal all tiles around it if the number of flags around it is equal to the number on the tile
- Press <kbd>Space</kbd> to reveal all tiles around a tile or flag the tile if it is not revealed
- Press <kbd>R</kbd> to start a new game
- Press <kbd>Escape</kbd> to quit the game
- Press <kbd>+</kbd> to increase the grid size
- Press <kbd>-</kbd> to decrease the grid size

## How to build

### Linux

- Install GLFW (`pacman -S glfw` on Arch Linux, `apt install libglfw3-dev` on Debian/Ubuntu)
- [Install Rust](https://rustup.rs/)
- Run `cargo build --release` or `cargo run --release`

### Windows

- [Install Visual Studio](https://visualstudio.microsoft.com/fr/downloads/) with C++ support
- [Install CMake](https://cmake.org/download/)
- [Install Rust](https://rustup.rs/)
- Run `cargo build --release` or `cargo run --release`

### MacOS

- Rip Bozo idk how
