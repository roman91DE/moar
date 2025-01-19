# Moar

`Moar` is a terminal-based pager application, similar to `more` or `less`, designed for scrolling through files in the terminal. It supports basic navigation and is written in Rust.

## Features

- **File Viewing**: Display the contents of a file in the terminal.
- **Scrolling**: Navigate through the file using the keyboard.
  - Scroll down: `Down Arrow`
  - Scroll up: `Up Arrow`
  - Quit: `q`
- **Terminal-Friendly**: Optimized for terminal dimensions with dynamic resizing.

## Installation

To use `Moar`, you need to have Rust installed. You can install Rust from [rust-lang.org](https://www.rust-lang.org/).

1. Clone the repository:
   ```bash
   git clone https://github.com/roman91DE/moar.git
   cd moar
   ```
2. Build the application:
   ```bash
   make release
   ```
3. Run the application:
   ```bash
   ./target/release/moar <filepath>
   ```

## Usage

### Command Syntax
```bash
moar <filepath>
```
- `<filepath>`: Path to the file you want to view.

### Example
```bash
moar example.txt
```

## Keybindings

- **`q`**: Quit the application.
- **`Up Arrow`**: Scroll up.
- **`Down Arrow`**: Scroll down.

## Dependencies

- [crossterm](https://crates.io/crates/crossterm): For terminal interaction and handling input/output.

## Contributing

Contributions are welcome! Feel free to submit a pull request or open an issue.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

Inspired by the classic `more` and `less` pager utilities.

