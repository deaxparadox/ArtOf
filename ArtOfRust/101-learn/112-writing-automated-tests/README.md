# Writing Automated Tests

- [How to Write tests](101-how-to-write-tests/README.md)
- [Using Result<T, E> in Tests](101-how-to-write-tests/105-using-result-in-tests.md)


### Controlling How Tests are Run

- `cargo run`: compiles your code and then runs the resulting binary.
- `cargo test`: compiles your code in test mode and runs the resulting test binary.

The default of the behavior of the binary produced by `cargo test` is to run all the tests in parallel and capture output generated during test runs, preventing the output from being displayed and making it easier to rend the output related to the test results.

Some command line options go to `cargo test`, and some go to the resulting test binary. To separate these two types of arguments, you list the arguments that go to `cargo test` followed by the separator `--` and then the ones that go to the test binary. 

Running `cargo test --help` displays the options you can use with `cargo test`, and running `cargo test -- --help` displays the options you can use after the separator.

- [Running Tests in Parallel or Consecutively](102-running-tests-in-paralllel-or-consecutively.md)