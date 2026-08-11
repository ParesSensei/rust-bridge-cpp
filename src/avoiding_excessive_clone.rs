
// when clone is wrong
#[test]
fn main() {
    // BAD: Cloning a String just to pass it to a function that only reads it
    fn log_message(msg: String) {  // Takes ownership unnecessarily
        println!("[LOG] {}", msg);
    }
    let message = String::from("GPU test passed");
    log_message(message.clone());  // Wasteful: allocates a whole new String
    log_message(message);           // Original consumed — clone was pointless

}

#[test]
fn main2() {
    // GOOD: Accept a borrow — zero allocation
    fn log_message(msg: &str) {    // Borrows, doesn't own
        println!("[LOG] {}", msg);
    }
    let message = String::from("GPU test passed");
    log_message(&message);          // No clone, no allocation
    log_message(&message);          // Can call again — message not consumed

}

// Checklist: Should I clone?
// 1. Can I accept &str / &T instead of String / T? → Borrow, don’t clone
// 2. Can I restructure to avoid needing two owners? → Pass by reference or use scopes
// 3. Is this Arc::clone()? → That’s fine, it’s O(1)
// 4. Am I moving data into a thread/closure? → Clone is necessary
// 5. Am I cloning in a hot loop? → Profile and consider borrowing or Cow<T>


// Cow (Clone on Write) is an enum that holds either a borrowed reference or an owned value.
// Without Cow — you must choose: always borrow OR always clone
fn normalize1(s: &str) -> String {          // Always allocates!
    if s.contains(' ') {
        s.replace(' ', "_")               // New String (allocation needed)
    } else {
        s.to_string()                     // Unnecessary allocation!
    }
}

// With Cow — borrow when unchanged, allocate only when modified
use std::borrow::Cow;

fn normalize2(s: &str) -> Cow<'_, str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_"))    // Allocates (must modify)
    } else {
        Cow::Borrowed(s)                   // Zero allocation (passthrough)
    }
}

#[test]
fn test_normalize() {
    let a = normalize1("hello world hmm  tcih huehue");
    println!("{a}");
    let b = normalize2("hello world a a a a");
    println!("{b}");
}

// How Cow works

// Cow<'a, str> is essentially:
// enum Cow<'a, str> {
//     Borrowed(&'a str),     // Zero-cost reference
//     Owned(String),         // Heap-allocated owned value
// }

fn greet(name: &str) -> Cow<'_, str> {
    if name.is_empty() {
        Cow::Borrowed("stranger")         // Static string — no allocation
    } else if name.starts_with(' ') {
        Cow::Owned(name.trim().to_string()) // Modified — allocation needed
    } else {
        Cow::Borrowed(name)               // Passthrough — no allocation
    }
}

#[test]
fn main3() {
    let g1 = greet("Alice");     // Cow::Borrowed("Alice")
    let g2 = greet("");          // Cow::Borrowed("stranger")
    let g3 = greet(" Bob ");     // Cow::Owned("Bob")

    // Cow<str> implements Deref<Target = str>, so you can use it as &str:
    println!("Hello, {g1}!");    // Works — Cow auto-derefs to &str
    println!("Hello, {g2}!");
    println!("Hello, {g3}!");
}


// Real-world use case: config value normalization

/// Normalize a SKU name: trim whitespace, lowercase.
/// Returns Cow::Borrowed if already normalized (zero allocation).
fn normalize_sku(sku: &str) -> Cow<'_, str> {
    let trimmed = sku.trim();
    if trimmed == sku && sku.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()) {
        Cow::Borrowed(sku)   // Already normalized — no allocation
    } else {
        Cow::Owned(trimmed.to_lowercase())  // Needs modification — allocate
    }
}

#[test]
fn main4() {
    let s1 = normalize_sku("server-x1");   // Borrowed — zero alloc
    let s2 = normalize_sku("  Server-X1 "); // Owned — must allocate
    println!("{s1}, {s2}"); // "server-x1, server-x1"
}