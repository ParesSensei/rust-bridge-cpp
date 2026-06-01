

fn count_word(text: &str) -> usize {
    text.split_whitespace().count()
}

fn longest_word(text: &str) -> &str {
    text.split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap_or("")
}

#[test]
fn main() {
    let text = "the quick brown fox jumps over the lazy dog";
    println!("Word count: {}", count_word(text));       // 9
    println!("Longest word: {}", longest_word(text));     // "jumps"
}