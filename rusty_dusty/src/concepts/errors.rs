use std::{
    fmt,
    fs::File,
    io::{self, ErrorKind, Read},
};

/// Demonstrates various error handling mechanisms including recoverable and custom errors.
pub fn main() {
    recoverable();

    match read_file_contents("files/text.txt") {
        Ok(contents) => println!("File contents: {contents}"),
        Err(err) => println!("Error: {err}"),
    }

    let result = divide(10, 2);
    match result {
        Ok(result) => println!("Result: {result}\n"),
        Err(err) => println!("Error: {err}"),
    }

    match read_and_parse_file("files/numbers.txt") {
        Ok(number) => println!("The number is: {}", number),
        Err(e) => println!("An error occurred: {}", e),
    }
}

/// Demonstrates handling a recoverable error when trying to open a file.
/// If the file doesn't exist, it attempts to create it.
/// Panics if an unexpected error occurs during opening or creating the file.
fn recoverable() {
    println!("Recoverable Error!");

    let file_result = File::open("files/hello.txt");

    let _ = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("files/hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    println!();
}

/// Attempts to divide two integers, returning a `Result`.
///
/// # Arguments
///
/// * `a` - Dividend.
/// * `b` - Divisor.
///
/// # Returns
///
/// * `Ok(i32)` if division is successful.
/// * `Err(&'static str)` if `b` is zero.
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Cannot divide by zero")
    } else {
        Ok(a / b)
    }
}

/// Reads the entire contents of a file into a `String`.
///
/// # Arguments
///
/// * `filename` - Path to the file.
///
/// # Returns
///
/// * `Ok(String)` containing the file contents.
/// * `Err(io::Error)` if reading fails.
fn read_file_contents(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

/// Custom error type that can wrap `io::Error` and `ParseIntError`.
#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::Parse(error)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Io(err) => write!(f, "IO error: {}", err),
            MyError::Parse(err) => write!(f, "Parse error: {}", err),
        }
    }
}

/// Reads a number from a file, parsing it as an `i32`.
///
/// # Arguments
///
/// * `path` - Path to the file.
///
/// # Returns
///
/// * `Ok(i32)` if the file is successfully read and parsed.
/// * `Err(MyError)` on IO or parsing error.
fn read_and_parse_file(path: &str) -> Result<i32, MyError> {
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let number: i32 = contents.trim().parse()?;
    Ok(number)
}
