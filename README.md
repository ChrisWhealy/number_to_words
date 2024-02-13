# Number to Words

This app has been written in Rust and transforms an integer in the range 0 to 999,999,999 into the corresponding number words. 

## Prerequisites

* If you have not already done so, please install the Rust toolchain using this command:
   
   `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

* Once Rust has been installed, you can optionally install my preferred Rust test tool called `nextest`:

   `cargo install nextest`

   Once installed, all tests can be run using `cargo nextest run`, rather than the more familiar `cargo test`

## Test & Execution

To test this application, run either `cargo test` or `cargo nextest run`

To run this application, you must first build the binary using the build shell script:

`./build.sh`

then you can run `./bin/numbers_to_words <some-number>`

```shell
╰→  ./bin/numbers_to_words 17942  
seventeen thousand nine hundred and forty-two
╰→  ./bin/numbers_to_words 918273645
nine hundred and eighteen million two hundred and seventy three thousand six hundred and forty five
```

## Program Behaviour

If the program runs successfully, it will write the number as words to `stdout` and terminate with an exit code of `0`.

If the program does not run successfully, it will terminate with an error message and an exit code of `1`.
The following conditions will cause the program to terminate with an error message:

1. The numeric argument is missing
2. There is more than one argument
3. The argument cannot be parsed as an integer
4. The argument can be parsed as an integer, but lies outside the bounds `0 ≤ arg ≤ 999999999`
