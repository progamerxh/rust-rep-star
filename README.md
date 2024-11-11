# Rust Rep Star

A Rust project for demonstrating full stack Rust Web.

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Introduction

This project is designed to showcase how to build full stack Rust Web.

## Installation

To install the project, clone the repository and build it using Cargo:

```sh
git clone https://github.com/yourusername/rust-rep-star.git
cd rust-rep-star
cargo build
```

## Backend 

Run the project using Cargo:

```sh
cargo run --bin api-actix
```

... or in watch mode:
```sh
cargo watch -x "run --bin api-actix"
```

Run the project using Shuttle:

```sh
shuttle run --working-directory=api/shuttle
```

## Frontend

Run the project using Dioxus-cli:

```sh
cd front/
dx serve
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.