
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