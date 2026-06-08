

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap()
}

#[test]
fn aa() {
    let x = "hello world".split_whitespace();
}
