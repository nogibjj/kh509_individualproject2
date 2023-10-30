# Individual Project #2: Rust CLI Binary with SQLite
## Katelyn Hucker (kh509) 

This is my individual project 2. In this project we combine the last few miniprojects into 1 project. Using Rust we create a command line tool which performs CRUD operations on a SQL Database. This creates a downloadable Rust binary file in Github Actions so that we can easily distrubte this rust code system. As always, I used Github Copilot and also performed code linting, formatting, and testing. 

### Contents:
```
kh509_individualproject2/
├── .github/
│   └── workflows/
          ├── release.yml
          └── clippy.yml
├── .gitignore
├── src/
│   ├── lib.rs
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
├── flower.db
├── iris.csv
├── Makefile
├── requirements.txt
└── README.md
```

### Rust Source Code 

My rust source code is in the src folder. It contains two files: lib.rs and main.rs. Lib.rs holds all the utility of the functions, and the tests for the code. Main.rs calls all the functions in the lib.rs and it also tells us the time it took to complete the CRUD operation. 

### SQLite Database: CRUD Operations

  ##### CREATE 

  If the database is not already created we create it in the lib.rs create_db function. We called this database 'flower.'
  ##### READ 
  We can access or read the database by performing the command: "cargo run -- --query 'SELECT * FROM iris_info'"

 
  ##### UPDATE
  ##### DELETE


Use of GitHub Copilot (10 points):



Optimized Rust Binary: 



Demo Video:
