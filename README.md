# Mini Project 7 : Package a Rust Script into a Command-Line Tool
Katelyn Hucker (kh509)
This is my mini project which is an introduction to coding in Rust. My project is a command line tool, which adds and removes fruits to a fruit list, which is already created. The original project can be found on [click here](https://github.com/nogibjj/rust-data-engineering/blob/main/calc-cli-with-tests/src/lib.rs) Instead of randomly getting fruits, my project lets users interact by adding and removing so it is now has more functionality.

### Contents:
```
kh509_miniproject7/
├── .github/
│   └── workflows/actions.yml
├── .gitignore
├── src/
│   ├── lib.rs
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
├── Makefile
├── requirements.txt
└── README.md
```

### 1. actions.yml
This is a GitHub Actions workflow file named `Clippy`:
- **Name**: Clippy
- **Triggers**: It is triggered on both push events to the `main` branch and pull requests.
- **Jobs**:
  - **build**:
    - **Environment**: It runs on an Ubuntu environment.
    - **Steps**:
      - `actions/checkout@v1`: Checks out the repository.
      - `actions-rs/toolchain@v1`: Sets up the Rust toolchain with Clippy and Rustfmt.
      - **Format**: Executes the `make format` command.
      - **Lint**: Executes the `make lint` command.
      - **Test**: Executes the `make test` command.
This workflow automates the process of formatting, linting, and testing a Rust project using GitHub Actions whenever there is a push to the `main` branch or a pull request is opened.
As seen in the badge above, the project is passing the entire CI/CD build without any errors. Below is also a screenshot of the project passing the test function written in `lib.rs`.


### 2. src Folder:



### 3. [Cargo.toml](https://github.com/nogibjj/aad64_command_line/Cargo.toml)
This file is the `Cargo.toml` manifest for a Rust project named `cli-customize-cards` with dependencies on `clap`, `csv`, and `rand` libraries. It specifies the project version, edition, and library configuration.

### 4. [Makefile](https://github.com/nogibjj/aad64_command_line/Makefile)
This is a `Makefile` used for automating various tasks in a Rust project:
- `format`: Invokes `cargo fmt` to automatically format the code according to Rust style guidelines.
- `lint`: Executes `cargo clippy` to perform linting and static analysis to catch potential issues or non-idiomatic code.
- `test`: Runs the project's test suite using `cargo test`.
- `run`: Launches the project with `cargo run`.
- `all`: Combines the tasks `format`, `lint`, `test`, and `run` to perform all common project tasks in sequence.
