mod exercisse_string_manipulation;

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