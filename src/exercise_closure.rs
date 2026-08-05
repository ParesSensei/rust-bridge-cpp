
fn add_one(x: u32) -> u32 {
    x + 1
}

#[test]
fn main() {
    let add_one_v1 = |x : u32| {x + 1}; // Explicitly specified type
    let add_one_v2 = |x| {x + 1};   // Type is inferred from call site
    let add_one_v3 = |x| x+1;   // Permitted for single line functions
    println!("{} {} {} {}", add_one(42), add_one_v1(42), add_one_v2(42), add_one_v3(42) );

    let mut greeting = "Hello".to_string();
    let mut append = |suffix: &str| {
        greeting.push_str(suffix);
    };

    append(", World");
    append(" !");
    println!("{}", greeting);

    let operation: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),  // add 1
        Box::new(|x| x * 2),  // multiply by 2
        Box::new(|x| x * x),  // square
    ];

    let input = 5;
    for (i, op) in operation.iter().enumerate() {
        println!(" Operation {} on {}: {}", i, input, op(input));
    }
}


// ----- iterator -------//

#[test]
fn main2() {
    let a = [1, 2, 3, 4, 5, 67, 68];
    for x in &a {
        if *x >= 67 {
            println!("{}", x);
        }
    }

    // same as above
    a.iter().filter(|&x| *x >= 67).for_each(|x| println!("{}", x));
}

#[test]
fn main3() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8];
    let (even, odd): (Vec<i32>, Vec<i32>) = a.iter().partition(|&x| x % 2 == 0);
    println!("{:?}", even);
    println!("{:?}", odd);

    // manual
    let mut evens = Vec::new();
    let mut odds = Vec::new();
    for n in a {
        if n % 2 == 0 {
            evens.push(n);
        } else {
            odds.push(n);
        }
    }
    println!("Evens: {evens:?}");
    println!("Odds:  {odds:?}");
}

// exercise

// ひ
fn alarm_report(sensors: &[(String, f64)]) -> Vec<String> {
    let mut hot: Vec<_> = sensors
        .iter()
        .filter(|(_, temp)| *temp > 80.0)
        .collect();

    hot.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    hot.into_iter()
        .map(|(name, temp)| format!("{name}: {temp}°C [ALARM]"))
        .collect()
}

#[test]
fn main4() {
    let sensors = vec![
        ("gpu0".to_string(), 72.5),
        ("gpu1".to_string(), 85.3),
        ("gpu2".to_string(), 91.0),
        ("gpu3".to_string(), 78.0),
        ("gpu4".to_string(), 88.7),
    ];
    for line in alarm_report(&sensors) {
        println!("{line}");
    }
}
