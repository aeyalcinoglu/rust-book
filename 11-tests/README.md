# Tests
- See [The Humble Programmer - Edsger W. Dijkstra, 1972](https://www.cs.utexas.edu/~EWD/transcriptions/EWD03xx/EWD340.html)
- At its simplest, a test in Rust is a function that’s annotated with the `test` attribute.
- When one runs tests with the `cargo test` command, Rust builds a test runner binary that runs the functions annotated with the `test` attribute.
- Tests fail when something in the test function panics.
- Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.
- Some command line options go to `cargo test`, and some go to the resulting test binary. They are separated by `--`.
- To tell the program not to use any parallelism while running tests, use
`cargo test -- --test-threads=1`.
- If you want to see printed values for passing tests (as well), use `cargo test -- --show-output`.
- Run a single test via `cargo test test_name`.
- Using `ignore` attribute on a test ignores it. To run only the ignored tests, use `cargo test -- --ignored`.
- The Rust community thinks about tests in terms of two main categories: *unit tests* and *integration tests*:
	- Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces.
	- Integration tests are entirely external to your library and use your code in the same way any other external code would.
- Put unit tests in the *src* directory in each file with the code that they’re testing. The convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`.
- To create integration tests, use the *tests* directory at the top level of the project.
- In order to be able to run integration test where unit tests are failing, use `cargo test --no-fail-fast` or to run only (and all) integration tests use `cargo test --test '*'`.
- Files in subdirectories of the *tests* directory don’t get compiled as separate crates or have sections in the test output.
- If our project is a binary crate that only contains a *src/main.rs* file and doesn’t have a *src/lib.rs* file, we can’t create integration tests in the *tests* directory.
