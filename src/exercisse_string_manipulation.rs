


fn count_word(text: &str) -> usize {
    let conv = String::from(text);
    conv.len()
}

#[test]
fn yeah_test_count_word() {
    assert_eq!(count_word("Hello"), 5);
}