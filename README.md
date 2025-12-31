# dep-why

Trace dependency chains in lock files to understand why packages are installed

## Features

- Parse package-lock.json (npm v1, v2, v3 formats)
- Parse yarn.lock files
- Parse pnpm-lock.yaml files
- Trace full dependency chain from target package to root dependencies
- Identify which direct dependencies require the target package
- Detect and highlight version conflicts across the tree
- Show all versions of a package if multiple exist
- Display dependency depth and path for each chain
- Support both package name and package@version queries
- Output as human-readable tree or JSON format
- Colorized terminal output for better readability
- Handle circular dependencies gracefully
- Fast parsing even for large monorepo lock files

## How to Use

Use this project when you need to:

- Quickly solve problems related to dep-why
- Integrate rust functionality into your workflow
- Learn how rust handles common patterns

## Installation

```bash
# Clone the repository
git clone https://github.com/KurtWeston/dep-why.git
cd dep-why

# Install dependencies
cargo build
```

## Usage

```bash
cargo run
```

## Built With

- rust

## Dependencies

- `clap`
- `serde`
- `serde_json`
- `colored`
- `anyhow`

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
