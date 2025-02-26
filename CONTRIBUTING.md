# Contribution Guidelines

Thank you for considering contributing to **Move**! We welcome all contributions, whether it's fixing a bug, adding a feature, improving documentation, or suggesting enhancements.

## Getting Started

1. **Fork the Repository**
   - Click the "Fork" button on the repository page to create your own copy.

2. **Clone Your Fork**
   ```sh
   git clone https://github.com/your-username/move.git
   cd move
   ```

3. **Create a Branch**
   - Use a descriptive branch name related to the contribution:
   ```sh
   git checkout -b feature/new-budget-category
   ```

## Development Setup

Ensure you have Rust installed. If not, install it via [rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, build the project:

```sh
cargo build
```

Run the application:

```sh
cargo run -- --help
```

## Contribution Process

### 1. Reporting Issues
If you find a bug or have a feature request, please check the existing [issues](https://github.com/ibilalkayy/move/issues) before opening a new one. When reporting an issue, provide as much detail as possible, including steps to reproduce the problem.

### 2. Making Changes
- Follow the existing code style.
- Write meaningful commit messages.
- Test your changes locally before submitting.

### 3. Submitting a Pull Request (PR)
1. Push your changes to your fork:
   ```sh
   git push origin feature/new-budget-category
   ```
2. Go to the original repository and click "New Pull Request."
3. Select your branch and describe the changes you made.
4. Submit the PR and wait for a review.

## Code Style & Best Practices
- Follow Rust’s [official coding guidelines](https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html).
- Format code using:
  ```sh
  cargo fmt
  ```
- Write tests for new features:
  ```sh
  cargo test
  ```

## License
By contributing, you agree that your contributions will be licensed under the [Apache-2.0 License](LICENSE).

Thank you for helping improve **Move**!

