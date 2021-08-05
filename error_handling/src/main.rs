use std::io::Read;

fn _panic() {
    panic!("Crash and burn bro");
}
fn _backtrace() {
    let v = vec![1, 2, 3];
    v[99];
}

fn _error_handling() {
    /*
    Shows one way of handling errors
    */
    use std::fs::File;
    use std::io::ErrorKind;
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn _better_error_handling() {
    /*
    Shows a cleaner method of handling errors, this is way more readable.
    */
    use std::fs::File;
    use std::io::ErrorKind;

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn _shortcut_error_handling() {
    /* unwrap and expect are two shortcuts for handling errors, signaling intent much better */
    use std::fs::File;
    // unwrap
    let f = File::open("hello.txt").unwrap();

    // expect
    /* We get to choose the panic! message, the error will still be printed for us too! */
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn _propagate_errors() -> Result<String, std::io::Error> {
    /*
    This function can be written in a much shorter way, but we’re going to start by doing a lot of
    it manually in order to explore error handling; at the end, we’ll show the shorter way. Let’s
    look at the return type of the function first: Result<String, io::Error> . This means the
    function is returning a value of the type Result<T, E> where the generic parameter T has
    been filled in with the concrete type String and the generic type E has been filled in with
    the concrete type io::Error .

     If this function succeeds without any problems, the code that
    calls this function will receive an Ok value that holds a String —the username that this
    function read from the file. If this function encounters any problems, the code that calls this
    function will receive an Err value that holds an instance of io::Error that contains more
    information about what the problems were.

    We chose io::Error as the return type of this
    function because that happens to be the type of the error value returned from both of the
    operations we’re calling in this function’s body that might fail: the File::open function and
    the read_to_string method.

    We don’t have enough information on what the calling code is
    actually trying to do, so we propagate all the success or error information upward for it to
    handle appropriately.
    This pattern of propagating errors is so common in Rust that Rust provides the question
    mark operator ? to make this easier.

    */

    use std::fs::File;

    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn _propagate_errors_questionmark() -> Result<String, std::io::Error> {
    /*
    If the value of the Result is an Ok , the value inside the Ok will get returned from
    this expression, and the program will continue.
    If the value is an Err , the Err will be returned from the whole
    function as if we had used the return keyword so the error value gets propagated to the
    calling code.

    There is a difference between what the match expression from Listing 9-6 does and what
    the ? operator does: error values that have the ? operator called on them go through the
    from function, defined in the From trait in the standard library, which is used to convert
    errors from one type into another. When the ? operator calls the from function, the error
    type received is converted into the error type defined in the return type of the current
    function. This is useful when a function returns one error type to represent all the ways a
    function might fail, even if parts might fail for many different reasons. As long as each error
    type implements the from function to define how to convert itself to the returned error
    type, the ? operator takes care of the conversion automatically.

    */
    use std::fs::File;
    use std::io::Read;

    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn _short_progate_errors_questionmark() -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn _shortest_propagate_errors() -> Result<String, std::io::Error> {
    std::fs::read_to_string("hello.txt")
}

fn _implement_guessing_game() {
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess { value }
        }
        pub fn value(&self) -> i32 {
            self.value
        }

    }
}

fn main() {}
