// // Starter code
// use std::{fmt, io};
//
// // TODO: Define AppError with variants:
// //   Io(io::Error), Json(serde_json::Error), Validation(String)
// // TODO: Implement Display and Error traits
// // TODO: Implement From<io::Error> and From<serde_json::Error>
// // TODO: Define type alias: type Result<T> = std::result::Result<T, AppError>;
//
// #[derive(Debug)]
// enum AppError{
//     IO(io::Error),
//     Json(serde_json::Error),
//     Validation(String),
// }
//
// impl fmt::Display for AppError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             AppError::IO(e) => e.fmt(f),
//             AppError::Json(e) => e.fmt(f),
//             AppError::Validation(e) => e.fmt(f),
//         }
//     }
// }
//
// impl std::error::Error for AppError {}
//
// pub type Result<T> = std::result::Result<T, AppError>;
// #[derive(serde::Deserialize)]
// struct Config {
//     name: String,
//     port: u16,
// }
// fn load_config(path: &str) -> Result<Config> {
//     let content = std::fs::read_to_string(path)?;  // io::Error → AppError
//     let config: Config = serde_json::from_str(&content)?;  // serde error → AppError
//     if config.name.is_empty() {
//         return Err(AppError::Validation("name cannot be empty".into()));
//     }
//     Ok(config)
// }


// This is literally how Option is defined in std:
// enum Option<T> {
//     Some(T),  // Contains a value
//     None,     // No value
// }
//
// // And Result:
// enum Result<T, E> {
//     Ok(T),    // Success with value
//     Err(E),   // Error with details
// }

#[test]
fn main() {
    // Returns Option<usize>
    let a = "1234".find("1");
    match a {
        Some(a) => println!("Found 1 at index {a}"),
        None => println!("Couldn't find 1")
    }
}

#[test]
fn main2() {
    // This return an Option<usize>
    let a = "1234".find("1");
    println!("{a:?} {}", a.unwrap());
    let a = "1234".find("5").or(Some(42));
    println!("{a:?}");
    if let Some(a) = "1234".find("1") {
        println!("{a}");
    } else {
        println!("Not found in string");
    }
    // This will panic
    // "1234".find("5").unwrap();
}

use std::num::ParseIntError;
#[test]
fn main3() {
    let a : Result<i32, ParseIntError>  = "1234z".parse();
    match a {
        Ok(n) => println!("Parsed {n}"),
        Err(e) => println!("Parsing failed {e:?}"),
    }
    let a : Result<i32, ParseIntError>  = "1234z".parse().or(Ok(-1));
    println!("{a:?}");
    if let Ok(a) = "1234".parse::<i32>() {
        println!("Let OK {a}");
    }
    // This will panic
    //"1234z".parse().unwrap();
}

#[test]
fn main4() {
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = opt.ok_or("value was None");  // Option → Result

    let res: Result<i32, &str> = Ok(42);
    let opt: Option<i32> = res.ok();  // Result → Option (discards error)

    // They share many of the same methods:
    // .map(), .and_then(), .unwrap_or(), .unwrap_or_else(), .is_some()/is_ok()
}


// exercise

// Todo Implement a log() function that accepts an Option<&str> parameter.
// If the parameter is None, it should print a default string
// Todo The function should return a Result with () for both success and error
// (in this case we’ll never have an error)


fn log(log: Option<&str>) -> Result<(), ()> {
    match log {
        Some(hmm) => println!("{hmm}"),
        None => println!("No log"),
    }
    Ok(())
}

#[test]
fn main5() {
    let _ = log(Some("System initialized"));
    let _ = log(None);

    // Alternative using unwrap_or:
    let msg: Option<&str> = None;
    println!("LOG: {}", msg.unwrap_or("(default message)"));
}

#[test]
fn main6() {
    let x : Option<u32> = None;
    // println!("{x}", x.unwrap()); // Will panic
    println!("{}", x.unwrap_or(0));  // OK -- prints 0
    let x = 41;
    //assert!(x == 42); // Will panic
    //panic!("Something went wrong"); // Unconditional panic
    let _a = vec![0, 1];
    // println!("{}", a[2]); // Out of bounds panic; use a.get(2) which will return Option<T>
}


// Rust error handling - comprehensive and forced
use std::fs::File;
use std::io::Read;

fn read_file_content(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;  // ? automatically propagates errors
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)  // Success case
}

#[test]
fn main7() {
    match read_file_content("example.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Failed to read file: {}", error),
        // Compiler forces us to handle both cases!
    }
}

#[test]
fn main8() {
    let x = "1234x".parse::<u32>();
    match x {
        Ok(x) => println!("Parsed number {x}"),
        Err(e) => println!("Parsing error {e:?}"),
    }
    let x  = "1234".parse::<u32>();
    // Same as above, but with valid number
    if let Ok(x) = &x {
        println!("Parsed number {x}")
    } else if let Err(e) = &x {
        println!("Error: {e:?}");
    }
}

fn double_string_number(s : &str) -> Result<u32, std::num::ParseIntError> {
    let x = s.parse::<u32>()?; // Returns immediately in case of an error
    Ok(x*2)
}

// Changes the error type to () in case of error
fn double_string_numberss(s : &str) -> Result<u32, ()> {
    let x = s.parse::<u32>().map_err(|_|())?; // Returns immediately in case of an error
    Ok(x*2)
}

fn double_string_numbers(s : &str) -> Result<u32, ()> {
    let x = s.parse::<u32>().unwrap_or_default(); // Defaults to 0 in case of parse error
    Ok(x*2)
}

fn double_optional_number(x : Option<u32>) -> Result<u32, ()> {
    // ok_or converts Option<None> to Result<u32, ()> in the below
    x.ok_or(()).map(|x|x*2) // .map() is applied only on Ok(u32)
}

#[test]
fn main9() {
    let result = double_string_number("1234");
    println!("{result:?}");
    let result = double_string_number("1234x");
    println!("{result:?}");
    let result = double_string_numberss("1234x");
    println!("{result:?}");
    let result = double_string_numbers("1234x");
    println!("{result:?}");
    let result = double_optional_number(Some(12234));
    println!("{result:?}");
}