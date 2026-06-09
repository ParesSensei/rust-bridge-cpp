

fn first_word(s: &str) -> &str {
    // The compiler applies elision rules:
    // Rule 1: input &str gets lifetime 'a → fn first_word(s: &'a str) -> &str
    // Rule 2: single input lifetime → output gets same → fn first_word(s: &'a str) -> &'a str
    match s.find(' ') {
        Some(pos) => &s[..pos],
        None => s,
    }
}
#[test]
fn main2() {
    let text = "hello world foo";
    let word = first_word(text);
    println!("First word: {word}");  // "hello"

    let single = "onlyone";
    println!("First word: {}", first_word(single));  // "onlyone"
}

// ------ Exercise2: Slice storage with lifetimes ------- //
// --- 🟡 Intermediate — your first encounter with lifetime annotations ---//

// TODO: Create a structure to store a reference to a slice
struct SliceStore<'a> {
    slice: & 'a str
}

impl<'a> SliceStore<'a> {
    fn new(slice: &'a str) -> Self {
        SliceStore { slice }
    }

    fn get_slice(&self) -> &'a str {
        self.slice
    }
}

#[test]
fn main3() {
    let s = "This is a long string";
    let store1 = SliceStore::new(&s[0..4]);   // "This"
    let store2 = SliceStore::new(&s[5..7]);   // "is"
    println!("store1: {}", store1.get_slice());
    println!("store2: {}", store2.get_slice());
}