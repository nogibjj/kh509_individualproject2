# Mini Project 7 : Package a Rust Script into a Command-Line Tool
## Katelyn Hucker (kh509) [![Clippy](https://github.com/nogibjj/kh509_miniproject7/actions/workflows/clippy.yml/badge.svg)]
This is my mini project which is an introduction to coding in Rust. My project is a command line tool, which adds and removes fruits to a fruit list, which is already created. The original project can be found on [here](https://github.com/nogibjj/rust-data-engineering/blob/main/calc-cli-with-tests/src/lib.rs). Instead of randomly getting fruits, my project lets users interact by adding and removing so it is now has more functionality.

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

### 1. [actions.yml](https://github.com/nogibjj/kh509_miniproject7/.github/workflows/clippy.yml)
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


### 2. [src folder](https://github.com/nogibjj/kh509_miniproject7/src)

Here is my lib.rs and main.rs functionality in use:

##### Step 1: Let's see what is currently in our fruits list. 

![image](https://github.com/nogibjj/kh509_miniproject7/assets/143521756/9adad8c9-d8dd-48fb-b6b8-be4259a814f7)


##### Step 2: Let's add a fruit to the list. 
![image](https://github.com/nogibjj/kh509_miniproject7/assets/143521756/79f603d3-c131-4b58-8ee2-f14281c68cbb)

##### Step 3: Let's remove a fruit from the list. 

![image](https://github.com/nogibjj/kh509_miniproject7/assets/143521756/86aa312c-3022-4c78-9061-4497b0d417eb)


### 3. [Cargo.toml](https://github.com/nogibjj/kh509_miniproject7/Cargo.toml)
This file is the `Cargo.toml` manifest for a Rust project named `calc-cli-with-tests` with dependencies on `clap`, `lazy_static`, `structopt` and `rand` libraries. It specifies the project version, edition, and library configuration.

### 4. [Makefile](https://github.com/nogibjj/kh509_miniproject7/Makefile)
This is a `Makefile` used for automating various tasks in a Rust project:
- `format`: Invokes `cargo fmt` to automatically format the code according to Rust style guidelines.
- `lint`: Executes `cargo clippy` to perform linting and static analysis to catch potential issues or non-idiomatic code.
- `test`: Runs the project's test suite using `cargo test`.
- `run`: Launches the project with `cargo run`.
- `all`: Combines the tasks `format`, `lint`, `test`, and `run` to perform all common project tasks in sequence.

### Further Work:
As this was an introduction project, I am still beginning to understand Rust's syntax. I would have wanted to allow the fruits list to keep its additions and removals so one can forever,add, and remove and then save the list. This would be a great tool for someone to hold their grocery list somewhere. 
