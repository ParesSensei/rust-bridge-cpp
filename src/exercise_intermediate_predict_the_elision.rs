// 1. Can the compiler elide?
fn trim_prefix(s: &str) -> &str { &s[1..] }
// of course


// 2. Can the compiler elide?
fn pick(flag: bool, a: &str, b: &str) -> &str {
    if flag { a } else { b }
}
//nope the compiler wont know which lifetime the output from, solution add only one lifetime to a and b : 'a

// 3. Can the compiler elide?
struct Parser { data: String }
impl Parser {
    fn next_token(&self) -> &str { &self.data[..5] }
}
// of course because rule 3 that &self/ &mut self get all output lifetime same with self

// 4. Can the compiler elide?
fn split_at(s: &str, pos: usize) -> (&str, &str) {
    (&s[..pos], &s[pos..])
}

// yes because input only one reference which applied rule one 
