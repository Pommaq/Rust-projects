## Iterating over strings
This iterates over Unicode Scalar values:

``
for c in "whatever".chars()
``

This iterates over raw bytes (u8):
Remember that scalar values can be composed of more than one byte.

``
for b in "whatever".bytes()
``


# Error handling
Rust unwinds and cleans up variables when hitting a panic!() during normal circumstances.
We can change this behavior to simply aborting and letting OS do the cleanup (minimizing executable size)
by adding 
````
[profile]
panic = 'abort'

OR

[profile.release]
panic = 'abort'
````
in the Cargo.toml file.


# Running Tests in Parallel or Consecutively
When you run multiple tests, by default they run in parallel using threads. This means the
tests will finish running faster so you can get feedback quicker on whether or not your code
is working. Because the tests are running at the same time, make sure your tests don’t
depend on each other or on any shared state, including a shared environment, such as the
current working directory or environment variables.
If you don’t want to run the tests in parallel or if you want more fine-grained control over the
number of threads used, you can send the --test-threads flag and the number of threads
you want to use to the test binary. Take a look at the following example:
```
$ cargo test -- --test-threads=1
```

## Showing Function Output
By default, if a test passes, Rust’s test library captures anything printed to standard output.
For example, if we call println! in a test and the test passes, we won’t see the println!
output in the terminal; we’ll see only the line that indicates the test passed. If a test fails,
we’ll see whatever was printed to standard output with the rest of the failure message.

## Running a Subset of Tests by Name
Sometimes, running a full test suite can take a long time. If you’re working on code in a
particular area, you might want to run only the tests pertaining to that code. You can choose
which tests to run by passing cargo test the name or names of the test(s) you want to run
as an argument.
To only test function "one_hundred"
````
 $ cargo test one_hundred
````
We can’t specify the names of multiple tests in this way; only the first value given to cargo
test will be used. But there is a way to run multiple tests.
Filtering to Run Multiple Tests
We can specify part of a test name, and any test whose name matches that value will be run.
For example, because two of our tests’ names contain add , we can run those two by
running cargo test add :
```
$ cargo test add
```
outputs:
````
running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
````

## Ignoring Some Tests Unless Specifically Requested
Sometimes a few specific tests can be very time-consuming to execute, so you might want to
exclude them during most runs of cargo test . Rather than listing as arguments all tests
you do want to run, you can instead annotate the time-consuming tests using the ignore
attribute to exclude them, as shown here:
```
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```
The expensive_test function is listed as ignored . If we want to run only the ignored tests,
we can use 
```
$ cargo test -- --ignored
```

# Test Organization
As mentioned at the start of the chapter, testing is a complex discipline, and different people
use different terminology and organization. The Rust community thinks about tests in terms
of two main categories: unit tests and integration tests. Unit tests are small and more focused,
testing one module in isolation at a time, and can test private interfaces. Integration tests
are entirely external to your library and use your code in the same way any other external
code would, using only the public interface and potentially exercising multiple modules per
test.
Writing both kinds of tests is important to ensure that the pieces of your library are doing
what you expect them to, separately and together.
### Unit Tests
The purpose of unit tests is to test each unit of code in isolation from the rest of the code to
quickly pinpoint where code is and isn’t working as expected. You’ll put unit tests in the src
directory in each file with the code that they’re testing. The convention is to create a module
named tests in each file to contain the test functions and to annotate the module with cfg(test) .

### The Tests Module and #[cfg(test)]
The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test
code only when you run cargo test , not when you run cargo build . This saves compile
time when you only want to build the library and saves space in the resulting compiled
artifact because the tests are not included. You’ll see that because integration tests go in a
different directory, they don’t need the #[cfg(test)] annotation. However, because unit
tests go in the same files as the code, you’ll use #[cfg(test)] to specify that they shouldn’t
be included in the compiled result.

```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```
This code is the automatically generated test module. The attribute cfg stands for
configuration and tells Rust that the following item should only be included given a certain
configuration option. In this case, the configuration option is test , which is provided by
Rust for compiling and running tests. By using the cfg attribute, Cargo compiles our test
code only if we actively run the tests with cargo test . This includes any helper functions
that might be within this module, in addition to the functions annotated with #[test] .

## Integration Tests
In Rust, integration tests are entirely external to your library. They use your library in the
same way any other code would, which means they can only call functions that are part of
your library’s public API. Their purpose is to test whether many parts of your library work
together correctly. Units of code that work correctly on their own could have problems when
integrated, so test coverage of the integrated code is important as well. To create integration
tests, you first need a tests directory.
### The tests Directory
We create a tests directory at the top level of our project directory, next to src. Cargo knows
to look for integration test files in this directory. We can then make as many test files as we
want to in this directory, and Cargo will compile each of the files as an individual crate.

```
use adder;
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```
We’ve added use adder at the top of the code, which we didn’t need in the unit tests. The
reason is that each file in the tests directory is a separate crate, so we need to bring our
library into each test crate’s scope.
We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)] . Cargo
treats the tests directory specially and compiles files in this directory only when we run
cargo test.
```
$ cargo test
Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 1.31s
        Running unittests (target/debug/deps/adder-1082c4b063a8fbe6)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
finished in 0.00s
    
    Running tests/integration_test.rs (target/debug/deps/integration_test-1082c4b063a8fbe6)
    
running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
finished in 0.00s

    Doc-tests adder
    
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
finished in 0.00s
```
The three sections of output include the unit tests, the integration test, and the doc tests.
The first section for the unit tests is the same as we’ve been seeing: one line for each unit
test (one named internal that we added in Listing 11-12) and then a summary line for the
unit tests.

If you want common functions for the integration tests, instead of creating a file ~/tests/common.rs you should create ~/tests/common/mod.rs
This is to avoid having common.rs as a test output.

### Integration Tests for Binary Crates
If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs
file, we can’t create integration tests in the tests directory and bring functions defined in the
src/main.rs file into scope with a use statement. Only library crates expose functions that
other crates can use; binary crates are meant to be run on their own.