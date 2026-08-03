// Modify the Point type to use two different types (T and U) for x and y

#[derive(Debug)] // We will discuss this later
struct Point<T, U> {
    x : T,
    y : U,
}
impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Point {x, y}
    }
    fn set_x(&mut self, x: T) {
        self.x = x;
    }
    fn set_y(&mut self, y: U) {
        self.y = y;
    }
}
impl Point<f32, f32> {
    fn is_secret(&self) -> bool {
        self.x == 42.0
    }
}

#[test]
fn main() {
    let mut p = Point::new(2, 4); // i32
    let q = Point::new(2.0, 4.0); // f32
    p.set_x(42);
    p.set_y(43);
    println!("{p:?} {q:?} {}", q.is_secret());
}


// Returns a tuple of type <T> composed of left and right of type <T>
fn pick<T>(x: u32, left: T, right: T) -> (T, T) {
    if x == 43 {
        (left, right)
    } else {
        (right, left)
    }
}

#[test]
fn main1() {
    let a = pick(42, "ha", "hi");
    let b = pick(42, "hello", "world");
    println!("{a:?}, {b:?}");
}


// Exercise: Trait constraints and generics

trait CipherText {
    fn encrypt(&self);
}
// TO DO
// struct Cipher<>
// Next, implement a method called encrypt on the struct impl that invokes encrypt on cipher
// Next, implement CipherText on two structs called CipherOne and CipherTwo (just println() is fine).
// Create CipherOne and CipherTwo, and use Cipher to invoke them



struct Cipher <T: CipherText> {
    cipher : T,
}
impl <T: CipherText> Cipher<T>{
    fn encrypt(&self) {
        self.cipher.encrypt();
    }
}

struct CipherOne;
struct CipherTwo;


impl CipherText for CipherOne{
    fn encrypt(&self) {
        println!("Encrypting cipher...");
    }
}

impl CipherText for CipherTwo{
    fn encrypt(&self) {
        println!("Encrypting cipher...");
    }
}

#[test]
fn eercise() {
    let c1 = Cipher { cipher : CipherOne };
    let c2 = Cipher { cipher : CipherTwo };

    c1.encrypt();
    c2.encrypt();
}