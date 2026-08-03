#![allow(unused)]

mod exercisse_string_manipulation;
mod eercise_vec_and_hashmap;
mod exercise_enum_and_match;
mod exercise_intermdiate;
mod exercise_move_copy_drop;
mod exercise_starter_lifetime;
mod exercise_intermediate_predict_the_elision;
mod exercise_shared_ownership_and_interiror_mutability;
mod refcell_train;
mod errhndl;
mod exercise_erro_handling;
mod exercise_trait;
mod exercise_generic;

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

#[allow(unused)]
const SECRET_OF_LIFE: u32 = 42;
#[allow(unused)]
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

#[derive(Debug)]
#[allow(unused)]
struct Pointt {x: u32, y: u32}
impl Pointt {
    fn new(x: u32, y: u32) -> Self {
        Pointt {x, y}
    }
    fn increment_x(&mut self) {
        self.x += 1;
    }
}
#[test]
fn main22() {
    let mut p = Pointt::new(10, 20);
    p.increment_x();
    println!("{p:?}")
}


// ------- Rust Ownership ------------ //
#[allow(unused)]
fn main23() {
    let a = 42; // Owner
    let b = &a; // First borrow
    {
        let aa = 42;
        let c = &a; // Second borrow; a is still in scope
        // Ok: c goes out of scope here
        // aa goes out of scope here
    }
    // let d = &aa; // Will not compile unless aa is moved to outside scope
    // b implicitly goes out of scope before a
    // a goes out of scope last
}

fn fooo(x: &u32) {
    println!("{x}");
}
fn barr(x: u32) {
    println!("{x}");
}

#[test]
fn main24() {
    let a = 42;
    fooo(&a);    // By reference
    barr(a);     // By value (copy)
}


// fn no_dangling() -> &u32 {
//     // lifetime of a begins here
//     let a = 42;
//     // Won't compile. lifetime of a ends here
//     &a
// }

fn ok_reference(a: &u32) -> &u32 {
    // Ok because the lifetime of a always exceeds ok_reference()
    a
}
#[test]
fn main25() {
    let a = 42;     // lifetime of a begins here
    let b = ok_reference(&a);
    // lifetime of b ends here
    // lifetime of a ends here
}

#[test]
fn main26() {
    let s = String::from("Rust");    // Allocate a string from the heap
    let s1 = s; // Transfer ownership to s1. s is invalid at this point
    println!("{s1}");
    // This will not compile
    //println!("{s}");
    // s1 goes out of scope here and the memory is deallocated
    // s goes out of scope here, but nothing happens because it doesn't own anything
}

fn fou(s : String) {
    println!("{s}");
    // The heap memory pointed to by s will be deallocated here
}
fn bag(s : &String) {
    println!("{s}");
    // Nothing happens -- s is borrowed
}
#[test]
fn main27() {
    let s = String::from("Rust string move example");    // Allocate a string from the heap
    fou(s); // Transfers ownership; s is invalid now
    // println!("{s}");  // will not compile
    let t = String::from("Rust string borrow example");
    bag(&t);    // t continues to hold ownership
    println!("{t}");
}


struct Pointie {
    x: u32,
    y: u32,
}
fn consume_point(p: Pointie) {
    println!("{} {}", p.x, p.y);
}
fn borrow_point(p: &Pointie) {
    println!("{} {}", p.x, p.y);
}
#[test]
fn main28() {
    let p = Pointie {x: 10, y: 20};
    // Try flipping the two lines
    borrow_point(&p);
    consume_point(p);
}

#[test]
fn main29() {
    let s = String::from("Rust");    // Allocate a string from the heap
    let s1 = s.clone(); // Copy the string; creates a new allocation on the heap
    println!("{s1}");
    println!("{s}");
    // s1 goes out of scope here and the memory is deallocated
    // s goes out of scope here, and the memory is deallocated
}


// Try commenting this out to see the change in let p1 = p; below
#[derive(Copy, Clone, Debug)]   // We'll discuss this more later
struct Pointq{x: u32, y:u32}
#[test]
fn main30() {
    let p = Pointq {x: 42, y: 40};
    let p1 = p;     // This will perform a copy now instead of move
    println!("p: {p:?}");
    println!("p1: {p:?}");
    let p2 = p1.clone();    // Semantically the same as copy
}


struct Poin {x: u32, y:u32}

// Equivalent to: ~Point() { printf("Goodbye point x:%u, y:%u\n", x, y); }
impl Drop for Poin {
    fn drop(&mut self) {
        println!("Goodbye point x:{}, y:{}", self.x, self.y);
    }
}
#[test]
fn main31() {
    let p = Poin{x: 42, y: 42};
    {
        let p1 = Poin{x:43, y: 43};
        println!("Exiting inner block");
        // p1.drop() called here — like C++ end-of-scope destructor
    }
    println!("Exiting main");
    // p.drop() called here
}


// -------- Lifetime and borrowing deep dive -------- //


fn borrow_mut(x: &mut u32) {
    *x = 43;
}
#[test]
fn main32() {
    let mut x = 42;
    let y = &mut x;
    borrow_mut(y);
    let _z = &x; // Permitted because the compiler knows y isn't subsequently used
    //println!("{y}"); // Will not compile if this is uncommented
    borrow_mut(&mut x); // Permitted because _z isn't used
    let z = &x; // Ok -- mutable borrow of x ended after borrow_mut() returned
    println!("{z}");
}


#[derive(Debug)]
struct Points {x: u32, y: u32}

// Without lifetime annotation, this won't compile:
// fn left_or_right(pick_left: bool, left: &Point, right: &Point) -> &Point

// With lifetime annotation - all references share the same lifetime 'a
fn left_or_right<'a>(pick_left: bool, left: &'a Points, right: &'a Points) -> &'a Points {
    if pick_left { left } else { right }
}

// More complex: different lifetimes for inputs
fn get_x_coordinate<'a, 'b>(p1: &'a Points, _p2: &'b Points) -> &'a u32 {
    &p1.x  // Return value lifetime tied to p1, not p2
}

#[test]
fn main33() {
    let p1 = Points {x: 20, y: 30};
    let result;
    {
        let p2 = Points {x: 42, y: 50};
        result = left_or_right(false, &p1, &p2);
        // This works because we use result before p2 goes out of scope
        println!("Selected: {result:?}");
    }
    // This would NOT work - result references p2 which is now gone:
    // println!("After scope: {result:?}");
}


use std::collections::HashMap;
#[derive(Debug)]
struct Pointa {x: u32, y: u32}
struct Lookup<'a> {
    map: HashMap<u32, &'a Pointa>,
}

#[test]
fn main34() {
    let p = Pointa{x: 42, y: 42};
    let p1 = Pointa{x: 50, y: 60};
    let mut m = Lookup {map : HashMap::new()};
    m.map.insert(0, &p);
    m.map.insert(1, &p1);
    {
        let p3 = Pointa{x: 60, y:70};
        //m.map.insert(3, &p3); // Will not compile
        // p3 is dropped here, but m will outlive
    }
    for (k, v) in m.map {
        println!("{v:?}");
    }
    // m is dropped here
    // p1 and p are dropped here in that order
}



// -----------SMART POINTER AND INTERIOR MUTABILITY---------------//


#[test]
fn main35() {
    // Creates a pointer to an integer (with value 42) created on the heap
    let f = Box::new(42);
    println!("{} {}", *f, f);
    // Cloning a box creates a new heap allocation
    let mut g = f.clone();
    *g = 43;
    println!("{f} {g}");
    // g and f go out of scope here and are automatically deallocated
}

// Rust - Ownership system prevents these issues
#[test]
fn rust_ownership_safety() {
    let data = Box::new(42);  // data owns the heap allocation

    let moved_data = data;    // Ownership transferred to moved_data
    // data is no longer accessible - compile error if used

    let borrowed = &moved_data;  // Immutable borrow
    println!("{}", borrowed);    // Safe to use

    // moved_data automatically freed when it goes out of scope
}

//  Borrowing Rules Visualization
#[test]
fn borrowing_rules_example() {
    let mut data = vec![1, 2, 3, 4, 5];

    // Multiple immutable borrows - OK
    let ref1 = &data;
    let ref2 = &data;
    println!("{:?} {:?}", ref1, ref2);  // Both can be used

    // Mutable borrow - exclusive access
    let ref_mut = &mut data;
    ref_mut.push(6);
    // ref1 and ref2 can't be used while ref_mut is active

    // After ref_mut is done, immutable borrows work again
    let ref3 = &data;
    println!("{:?}", ref3);
}


use std::rc::Rc;
#[derive(Debug)]
struct Employee {employee_id: u64}
#[test]
fn main36() {
    let mut us_employees = vec![];
    let mut all_global_employees = vec![];
    let employee = Employee { employee_id: 42 };
    let employee_rc = Rc::new(employee);
    us_employees.push(employee_rc.clone());
    all_global_employees.push(employee_rc.clone());
    let employee_one = all_global_employees.get(0); // Shared immutable reference
    for e in us_employees {
        println!("{}", e.employee_id);  // Shared immutable reference
    }
    println!("{employee_one:?}");
}

use std::rc::{Weak};

struct Node {
    value: i32,
    parent: Option<Weak<Node>>,  // Weak reference — doesn't prevent drop
}

#[test]
fn main37() {
    let parent = Rc::new(Node { value: 1, parent: None });
    let child = Rc::new(Node {
        value: 2,
        parent: Some(Rc::downgrade(&parent)),  // Weak ref to parent
    });

    // To use a Weak, try to upgrade it — returns Option<Rc<T>>
    if let Some(parent_rc) = child.parent.as_ref().unwrap().upgrade() {
        println!("Parent value: {}", parent_rc.value);
    }
    println!("Parent strong count: {}", Rc::strong_count(&parent)); // 1, not 2
}


mod math {
    // TODO: implement pub fn add(a: u32, b: u32) -> u32
    pub fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}

fn greet(name: &str) -> String {
    // TODO: return "Hello, <name>! The secret number is <math::add(21,21)>"
    format!("Hello, {}! The secret number is {}", name, math::add(21, 21))
}

#[test]
fn main38() {
    println!("{}", greet("Rustacean"));
}