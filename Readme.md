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

Use crates for test APIs

Use mockall crates

> cargo add mockall

