# Individual Project #2: Rust CLI Binary with SQLite
## Katelyn Hucker (kh509) [Build binary release](https://github.com/nogibjj/kh509_individualproject2/actions/workflows/release.yml/badge.svg) [![format, lint, test](https://github.com/nogibjj/kh509_individualproject2/actions/workflows/format,lint,test.yml/badge.svg)](https://github.com/nogibjj/kh509_individualproject2/actions/workflows/format,lint,test.yml)

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
The github workflows completes all formatting linting, and testing requirements:

### Rust Source Code 

My rust source code is in the src folder. It contains two files: lib.rs and main.rs. Lib.rs holds all the utility of the functions, and the tests for the code. Main.rs calls all the functions in the lib.rs and it also tells us the time it took to complete the CRUD operation. 

### SQLite Database: CRUD Operations

  ##### CREATE 
  If the database is not already created we create it in the lib.rs create_db function. We called this database 'flower.' See this in the code under the READ         section
  ##### READ 
  We can access or read the database by performing the command: "cargo run -- --query 'SELECT * FROM iris_info'"
  ![image](https://github.com/nogibjj/kh509_individualproject2/assets/143521756/84a1d67f-5fc5-428d-8039-d68a11d2f03a)
  ##### UPDATE
  We updates species to mytosa if they were listed as a setosa then re-read the iris_info table
  ![image](https://github.com/nogibjj/kh509_individualproject2/assets/143521756/5e05b16c-b00b-4673-ab4f-ce6b7fc26b0d)
  ##### DELETE
  Here we deleted species which are virginica
  ![image](https://github.com/nogibjj/kh509_individualproject2/assets/143521756/ce259f74-6765-4b80-a780-420ed8073926)
  
Use of GitHub Copilot:

Github Copilot was used to help autocomplete what dependencies were needed. It was also helpful for converting from python to rust as I am not fluent in coding in rust. I was able to write in python or describe the functioanlity as it would be in python the copilot would conver it to rust. 

Optimized Rust Binary: 

We created a release exectuable called sql runner. This distrubted all the functionality we see in the program in one file for anyone to use. We created this by running 'make release' which includes all the required dependencins and builds the rust project. 

![image](https://github.com/nogibjj/kh509_individualproject2/assets/143521756/c74b116e-64bd-4469-83bc-9db65307850b)


Demo Video:
