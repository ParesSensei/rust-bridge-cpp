mod exercisse_string_manipulation;
mod eercise_vec_and_hashmap;
mod exercise_enum_and_match;

fn main() {
    println!("Hello, world!");
}

#[test]
fn slice_operations() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let full_slice = &data[..];        // [1,2,3,4,5,6,7,8]
    let partial_slice = &data[2..6];   // [3,4,5,6]
    let from_start = &data[..4];       // [1,2,3,4]
    let to_end = &data[3..];           // [4,5,6,7,8]

    println!("full_slice: {:?},\npartial_slice: {:?}", full_slice, partial_slice);
    println!("from_start: {:?},\nto_end: {:?}", from_start, to_end);
}


#[test]
fn ingteger() {
    let _x: i32 = 43;
    // These two assignments are logically equivalent
    let _y: u32 = 42;
    let _z = 32u32;
}

#[allow(dead_code)]
fn foo(x : u8) -> u32
{
    return x as u32 * x as u32;
}


fn secret_of_life_u32(x : u32) {
    println!("The u32 secret_of_life is {}", x);
}

fn secret_of_life_u8(x : u8) {
    println!("The u8 secret_of_life is {}", x);
}

#[allow(dead_code)]
fn mail() {
    let a = 42; // The let keyword assigns a value; type of a is u32
    let b = 42; // The let keyword assigns a value; inferred type of b is u8
    secret_of_life_u32(a);
    secret_of_life_u8(b);
}

#[test]
fn main1() {
    let x = 42;
    if x < 42 {
        println!("Smaller than the secret of life");
    } else if x == 42 {
        println!("Is equal to the secret of life");
    } else {
        println!("Larger than the secret of life");
    }
    let is_secret_of_life = if x == 42 {true} else {false};
    println!("{}", is_secret_of_life);
}

#[test]
fn main2() {
    let mut x = 40;
    while x != 42 {
        x += 1;
        println!("{}", x);
    }
}

#[test]
fn main3() {
    // Will not print 43; use 40..=43 to include last element
    for x in 40..43 {
        println!("{}", x);
    }
}

#[test]
fn main4() {
    let mut x = 40;
    // Change the below to 'here: loop to specify optional label for the loop
    loop {
        if x == 42 {
            break; // Use break x; to return the value of x
        }
        x += 1;
        println!("{}", x);
    }
}

#[allow(dead_code)]
fn get_index(y : usize) -> usize {
    y+1
}

#[test]
fn main5() {
    // Initializes an array of 3 elements and sets all to 42
    let a : [u8; 3] = [42; 3];
    // Alternative syntax
    // let a = [42u8, 42u8, 42u8];
    for x in a {
        println!("{x}");
    }
    let _y = get_index(a.len());
    // Commenting out the below will cause a panic
    //println!("{}", a[y]);

    let arr : [i32; 4] = [10; 4];
    let mut j = 0;
    for i in arr {
        j += i;
    }
    println!("{}", j);

    let sum: i32 = arr.iter().sum();
    println!("{sum}");
}

#[test]
fn main6() {
    let a = [
        [40, 0], // Define a nested array
        [41, 0],
        [42, 1],
    ];
    for x in a {
        println!("{x:?}");
    }
}

#[allow(dead_code)]
fn get_tuple() -> (u32, bool) {
    (42, true)
}

#[test]
fn main7() {
    let t : (u8, bool) = (42, true);
    let u : (u32, bool) = (43, false);
    println!("{}, {}", t.0, t.1);
    println!("{}, {}", u.0, u.1);
    let (num, flag) = get_tuple(); // Tuple destructuring
    println!("{num}, {flag}");
}

#[test]
fn main8() {
    let mut a = 42;
    {
        let b = &a;
        let c = b;
        println!("{} {}", *b, *c); // The compiler automatically dereferences *c

        let _d = &mut a;

        /*
         * Uncommenting the line below would cause the
         * program to not compile, because `b` is used
         * while the mutable reference `d` is live in the current scope
         *
         * You cannot have a mutable and immutable reference in use in the same scope
         * at the same time!
         */
        // println!("{}", *b);
    }
    let d = &mut a; // Ok: b and c are not in scope
    *d = 43;

    println!("{}", d);
}

#[test]
fn main9() {
    let a = [40, 41, 42, 43];
    let b = &a[1..a.len()]; // A slice starting with the second element in the original
    let c = &a[1..]; // Same as the above
    let d = &a[..]; // Same as &a[0..] or &a[0..a.len()]
    println!("{b:?} {c:?} {d:?}");
}


const SECRET_OF_LIFE: u32 = 42;
static GLOBAL_VARIABLE : u32 = 2;
#[test]
fn main10() {
    println!("The secret of life is {}", SECRET_OF_LIFE);
    println!("Value of global variable is {GLOBAL_VARIABLE}")
}

#[test]
fn main11() {
    // &str - string slice (borrowed, immutable, usually a string literal)
    let greeting: &str = "Hello";  // Points to read-only memory

    // String - owned, heap-allocated, growable
    let mut owned = String::from(greeting);  // Copies data to heap
    owned.push_str(", World!");        // Grow the string
    owned.push('!');                   // Append a single character

    // Converting between String and &str
    let slice: &str = &owned;          // String -> &str (free, just a borrow)
    let _owned2: String = slice.to_string();  // &str -> String (allocates)
    let _owned3: String = String::from(slice); // Same as above

    // String concatenation (note: + consumes the left operand)
    let hello = String::from("Hello");
    let world = String::from(", World!");
    let _combined = hello + &world;  // hello is moved (consumed), world is borrowed
    // println!("{hello}");  // Won't compile: hello was moved

    // Use format! to avoid move issues
    let a = String::from("Hello");
    let b = String::from("World");
    let combined = format!("{a}, {b}!");  // Neither a nor b is consumed

    println!("{combined}");
}

#[test]
fn main12() {
    struct MyStruct {
        num: u32,
        is_secret_of_life: bool,
    }

    let x = MyStruct {
        num: 42,
        is_secret_of_life: true,
    };

    let y = MyStruct {
        num: x.num,
        is_secret_of_life: x.is_secret_of_life,
    };
    let n = MyStruct {
        num: 22,
        is_secret_of_life: false,
    };
    let z = MyStruct { num: x.num, ..x }; // The .. means copy remaining
    println!("{} {} {}", n.num, y.is_secret_of_life, z.num);
}

struct WeightInGrams(u32);
#[allow(dead_code)]
struct WeightInMilligrams(u32);
fn to_weight_in_grams(kilograms: u32) -> WeightInGrams {
    WeightInGrams(kilograms * 1000)
}

fn to_weight_in_milligrams(w : WeightInGrams) -> WeightInMilligrams  {
    WeightInMilligrams(w.0 * 1000)
}

#[test]
fn main13() {
    let x = to_weight_in_grams(42);
    let _y = to_weight_in_milligrams(x);
    // let z : WeightInGrams = x;  // Won't compile: x was moved into to_weight_in_milligrams()
    // let a : WeightInGrams = y;   // Won't compile: type mismatch (WeightInMilligrams vs WeightInGrams)
}

#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }

#[test]
fn main14() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);           // Debug: works because of #[derive(Debug)]
    let p2 = p.clone();           // Clone: works because of #[derive(Clone)]
    assert_eq!(p, p2);            // PartialEq: works because of #[derive(PartialEq)]
}

#[test]
fn main15() {
    let mut v = Vec::new();    // Empty vector, type inferred from usage
    v.push(42);                // Add element to end - Vec<i32>
    v.push(43);

    // Safe iteration (preferred)
    for x in &v {              // Borrow elements, don't consume vector
        println!("{x}");
    }

    // Initialization shortcuts
    let mut v2 = vec![1, 2, 3, 4, 5];           // Macro for initialization
    let _v3 = vec![0; 10];                       // 10 zeros

    // Safe access methods (preferred over indexing)
    match v2.get(0) {
        Some(first) => println!("First: {first}"),
        None => println!("Empty vector"),
    }

    match v2.get(1) {
        Some(second) => println!("Second: {second}"),
        None => println!("Empty vector"),
    }

    match v2.get(2) {
        Some(thirddd) => println!("Third: {thirddd}"),
        None => println!("Empty vector"),
    }

    // Useful methods
    println!("Length: {}, Capacity: {}", v2.len(), v2.capacity());
    if let Some(last) = v2.pop() {             // Remove and return last element
        println!("Popped: {last}");
    }

    // Dangerous: direct indexing (can panic!)
    // println!("{}", v2[100]);  // Would panic at runtime
}

#[test]
fn main16() {
    use std::collections::HashMap;  // Need explicit import, unlike Vec
    let mut map = HashMap::new();       // Allocate an empty HashMap
    map.insert(40, false);  // Type is inferred as int -> bool
    map.insert(41, false);
    map.insert(42, true);
    for (key, value) in map {
        println!("{key} {value}");
    }
    let map = HashMap::from([(40, false), (41, false), (42, true)]);
    if let Some(x) = map.get(&43) {
        println!("43 was mapped to {x:?}");
    } else {
        println!("No mapping was found for 43");
    }
    let x = map.get(&43).or(Some(&false));  // Default value if key isn't found
    println!("{x:?}");
}

#[test]
#[allow(unused)]
fn main17() {
    enum Numbers {
        Zero,
        SmallNumber(u8),
        BiggerNumber(u32),
        EvenBiggerNumber(u64),
    }
    let a = Numbers::Zero;
    let b = Numbers::SmallNumber(42);
    let c : Numbers = a; // Ok -- the type of a is Numbers
    let d : Numbers = b; // Ok -- the type of b is Numbers
}

#[test]
fn main18() {
    let x = 42;
    // In this case, the _ covers all numbers except the ones explicitly listed
    let is_secret_of_life = match x {
        42 => true, // return type is boolean value
        _ => false, // return type boolean value
        // This won't compile because return type isn't boolean
        // _ => 0
    };
    println!("{is_secret_of_life}");
}

#[test]
fn main19() {
    let x = 42;
    match x {
        // Note that the =41 ensures the inclusive range
        0..=41 => println!("Less than the secret of life"),
        42 => println!("Secret of life"),
        _ => println!("More than the secret of life"),
    }
    let y = 100;
    match y {
        100 if x == 43 => println!("y is 100% not secret of life"),
        100 if x == 42 => println!("y is 100% secret of life"),
        _ => (),    // Do nothing
    }
}


#[test]
#[allow(unused)]
fn main20() {
    enum Numbers {
        Zero,
        SmallNumber(u8),
        BiggerNumber(u32),
        EvenBiggerNumber(u64),
    }
    let b = Numbers::SmallNumber(42);
    match b {
        Numbers::Zero => println!("Zero"),
        Numbers::SmallNumber(value) => println!("Small number {value}"),
        Numbers::BiggerNumber(_) | Numbers::EvenBiggerNumber(_) => println!("Some BiggerNumber or EvenBiggerNumber"),
    }

    // Boolean test for specific variants
    if matches!(b, Numbers::Zero | Numbers::SmallNumber(_)) {
        println!("Matched Zero or small number");
    }
}
#[test]
fn main21() {
    struct Foo {
        x: (u32, bool),
        y: u32
    }
    let f = Foo {x: (42, true), y: 100};
    match f {
        // Capture the value of x into a variable called tuple
        Foo{y: 100, x : tuple} => println!("Matched x: {tuple:?}"),
        _ => ()
    }
    let a = [40, 41, 42];
    match a {
        // Last element of slice must be 42. @ is used to bind the match
        [rest @ .., 42] => println!("{rest:?}"),
        // First element of the slice must be 42. @ is used to bind the match
        [42, rest @ ..] => println!("{rest:?}"),
        _ => (),
    }
}