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