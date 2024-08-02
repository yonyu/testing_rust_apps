> md testing_rust_apps
> 
> cd testing_rust_apps
> 
> cargo init
> 
> code .
>

>cargo new testlib --lib
>
>cd testlib/
>
> cargo test
> 


> cargo test -- --help

> cargo test -- --test-threads=1
> 
> show output
> 
> cargo test -- --nocapture
> 
> cargo test -- --show-output
> 
> cargo test -- --partial_name_in_test_functions
> 
> cargo test -- --exact_name_in_test_functions

> cargo add chrono


## Unit tests

Unit tests are used to test individual units of code. They are placed in the same file as the code they are testing.

It is a developer's description whether to test private functions or not. 

## Integration tests

Integration tests are used to test the interaction between multiple modules. They are placed in the tests directory. 
The tests directory is a sibling of the src directory.

You can create subdirectories in the tests directory to organize your tests.

Integration tests make sure that the modules work together as expected.

### Integration test example

Use [Chrono: Timezone-aware date and time handling](https://crates.io/crates/chrono) as an example. Get the [source code](https://github.com/chronotope/chrono) from GitHub. 
Explore its test layout and structure.


## Mocking in Rust

Mocking is a technique used to test code that depends on other code. It is used to simulate the behavior of the code 
that the code under test depends on.

### Reasons to use mocking
- Control over test environment
- Isolation of component for unit testing
- Improve test stability and reliability
- Efficiency and speed
- Enable testing of unavailable or incomplete components

- Test code that depends on external services, databases, or other code that is difficult to test.
- Test code that depends on code that is not yet implemented or not available.
- Test code that depends on code that is difficult to test.
- Improve test performance by using mock objects to simulate the behavior of the code that the code under test depends on.

Use the mockall crate to create mock objects for testing.

Use crates for test APIs

Use mockall crates

> cargo add mockall

