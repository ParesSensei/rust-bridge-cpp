

enum Operation {
    Add(u64, u64),
    Subtract(u64, u64),
}

enum CalcResult {
    Ok(u64),                    // Successful result
    Invalid(String),            // Error message for invalid operations
}

fn calculate(op: Operation) -> CalcResult {
    match op {
        Operation::Add(a, b) => CalcResult::Ok(a + b),
        Operation::Subtract(a, b) => if a >= b {
            CalcResult::Ok(a - b)
        } else {
            CalcResult::Invalid("Underflow".to_string())
        }
    }
}

#[test]
fn testing() {
    calculate(Operation::Add(1, 2));
    calculate(Operation::Subtract(1, 2));
    calculate(Operation::Subtract(2, 1));
}