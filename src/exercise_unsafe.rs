
unsafe fn harmless() {}

#[test]
fn main() {
    // Safety: We are calling a harmless unsafe function
    unsafe {
        harmless();
    }
    let a = 42u32;
    let p = &a as *const u32;
    // Safety: p is a valid pointer to a variable that will remain in scope
    unsafe {
        println!("{}", *p);
    }
    // Safety: Not safe; for illustration purposes only
    let dangerous_buffer = 0xb8000 as *mut u32;
    unsafe {
        println!("About to go kaboom!!!");
        *dangerous_buffer = 0; // This will SEGV on most modern machines
    }
}