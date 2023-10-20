@katelyn-hucker ➜ /workspaces/kh509_miniproject7 (main) $ cargo run
   Compiling calc-cli-with-tests v0.1.0 (/workspaces/kh509_miniproject7)
error[E0597]: `fruit` does not live long enough
  --> src/main.rs:36:23
   |
35 |         Subcommand::Add { fruit } => {
   |                           ----- binding `fruit` declared here
36 |             add_fruit(&fruit);
   |             ----------^^^^^^-
   |             |         |
   |             |         borrowed value does not live long enough
   |             argument requires that `fruit` is borrowed for `'static`
37 |             println!("Added fruit: {}", fruit);
38 |         }
   |         - `fruit` dropped here while still borrowed

error[E0597]: `fruit` does not live long enough
  --> src/main.rs:40:26
   |
39 |         Subcommand::Remove { fruit } => {
   |                              ----- binding `fruit` declared here
40 |             remove_fruit(&fruit);
   |             -------------^^^^^^-
   |             |            |
   |             |            borrowed value does not live long enough
   |             argument requires that `fruit` is borrowed for `'static`
41 |             println!("Removed fruit: {}", fruit);
42 |         }
   |         - `fruit` dropped here while still borrowed

For more information about this error, try `rustc --explain E0597`.
error: could not compile `calc-cli-with-tests` (bin "calc-cli-with-tests") due to 2 previous errors