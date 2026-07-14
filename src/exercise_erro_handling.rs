// 🟡 Intermediate

// todo Implement a log() function with a single u32 parameter. If the parameter is not 42,
// return an error. The Result<> for success and error type is ()
// Invoke log() function that exits with the same Result<> type if log() return an error.
// Otherwise print a message saying that log was successfully called



fn log(x: u32) -> Result<u32, ()> {
    match x {
        42 => Ok(x),
        _ => Err(()),
    }
}

fn call_log(x: u32) -> Result<(), ()> {
// Call log(x), then exit immediately if it return an error
    log(x)?;
    println!("log was successfully called");
    Ok(())
}

fn main() {
    call_log(42);
    call_log(43);
}