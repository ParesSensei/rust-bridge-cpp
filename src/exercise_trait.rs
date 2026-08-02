
#[test]
fn main() {
    trait Pet {
        fn speak(&self);

        fn eat(&self);
    }
    struct Cat;
    struct Dog;
    impl Pet for Cat {
        fn speak(&self) {
            println!("Meow");
        }
        fn eat(&self) {
            println!("meat");
        }
    }
    impl Pet for Dog {
        fn speak(&self) {
            println!("Woof!")
        }
        fn eat(&self) {
            println!("bone");
        }
    }
    let c = Cat{};
    let d = Dog{};
    c.speak();  // There is no "is a" relationship between Cat and Dog
    c.eat();
    d.speak(); // There is no "is a" relationship between Cat and Dog
}


// -------- ahahahahahaha -----------//

struct Person{
    first_name: String,
    last_name: String,
}

trait CanSayHello {
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        format!("Hello, {}!", self.first_name)
    }
    fn say_hello_to(&self, name: &str) -> String {
        format!("{}, {}", self.last_name, name)
    }
}

#[test]
fn main1() {
    let person = Person{
        first_name: "amau".to_string(),
        last_name: "pak mao".to_string(),
    };

    let result = person.say_hello();
    println!("{}", result);
    let result = person.say_hello_to("budi");
    println!("{}", result);
}

trait Animal {
    // Default implementation
    fn is_mammal(&self) -> bool {
        true
    }
}
trait Feline : Animal {
    // Default implementation
    fn is_feline(&self) -> bool {
        true
    }
}

struct Cat;
// Use default implementations. Note that all traits for the supertrait must be individually implemented
impl Feline for Cat {}
impl Animal for Cat {}

#[test]
fn main2() {
    let c = Cat{};
    println!("{} {}", c.is_mammal(), c.is_feline());
}

trait Log{
    fn log(&self, msg: u64);
}

struct SimpleLogger{}

struct ComplexLogger{}

impl Log for SimpleLogger{
    fn log(&self, msg: u64){
        println!("Simple logger : {}",  msg);
    }
}

impl Log for ComplexLogger{
    fn log(&self, msg: u64){
        println!("Complex Logger : {} (hex: 0x{msg:x}, binary: {msg:b})", msg)
    }
}
#[test]
fn tes_log() {
    let simple = SimpleLogger{};
    simple.log(3);

    let complex = ComplexLogger{};
    complex.log(45);
}

#[derive(Debug)]
struct Small(u32);
#[derive(Debug)]
struct Big(u32);

trait Double {
    type T;
    fn double(&self) -> Self::T;
}

trait Triple {
    type T;
    fn triple(&self) -> Self::T;
}

trait Quadra {
    type T;
    fn quadra(&self) -> Self::T;
}

impl Double for Small {
    type T = Big;
    fn double(&self) -> Self::T {
        Big(self.0 * 2)
    }
}

impl Triple for Small {
    type T = Big;
    fn triple(&self) -> Self::T {
        Big(self.0 * 3)
    }
}

impl Quadra for Small {
    type T = Big;
    fn quadra(&self) -> Self::T {
        Big(self.0 * 4)
    }
}
#[test]
fn main3() {
    let a = Small(10);
    println!("{:?}", a.double());
    println!("{:?}", a.triple());
    println!("{:?}", a.quadra());
}

trait Shape {
    fn area(&self) -> f64;
}
struct Circle {
    radius: f64
}
struct Rect {
    w: f64,
    h: f64
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}
impl Shape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}

// Static dispatch — compiler generates separate code for each type
fn print_area(s: &impl Shape) { println!("{}", s.area()); }

// Dynamic dispatch — one function, works with any Shape behind a pointer
fn print_area_dyn(s: &dyn Shape) { println!("{}", s.area()); }

// Enum — closed set, no trait needed
enum ShapeEnum { Circle(f64), Rect(f64, f64) }
impl ShapeEnum {
    fn area(&self) -> f64 {
        match self {
            ShapeEnum::Circle(r) => std::f64::consts::PI * r * r,
            ShapeEnum::Rect(w, h) => w * h,
        }
    }
}
