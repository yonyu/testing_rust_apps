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
_
Use [Chrono: Timezone-aware date and time handling](https://crates.io/crates/chrono) as an example. Get the [source code](https://github.com/chronotope/chrono) from GitHub. Explore its test layout and structure.

> cargo add chrono

Mocking in Rust

Use crates for test APIs

Use mockall crates

> cargo add mockall

