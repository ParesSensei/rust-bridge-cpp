use std::cell::RefCell;

struct Counter {
    value: RefCell<i32>,
}

// soal 1
#[test]
fn main() {
    let cont = Counter{
        value: RefCell::new(0),
    };

    *cont.value.borrow_mut() += 1;
    println!("{}", cont.value.borrow());
    *cont.value.borrow_mut() += 1;
    println!("{}", cont.value.borrow());
}

// soal 2
struct User {
    name: RefCell<String>,
}
#[test]
fn main2() {
    let user = User{
        name: RefCell::new(String::from("Jack"))
    };
    println!("{}", user.name.borrow());
    {
        *user.name.borrow_mut() = "alice".to_string();
    }
    println!("{}", user.name.borrow());
    {
        *user.name.borrow_mut() = "bob".to_string();
    }
    println!("{}", user.name.borrow());
    {
        *user.name.borrow_mut() = "charlie".to_string();
    }
    println!("{}", user.name.borrow());

}

// soal 3

impl Counter {
    fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }

    fn get (&self) -> i32 {
        *self.value.borrow()
    }
}
#[test]
fn main3() {
    let counter = Counter {
        value: RefCell::new(0),
    };

    counter.increment();
    counter.increment();
    counter.increment();

    println!("{}", counter.get());
}

// soal 4
use std::rc::Rc;

#[test]
fn main4() {
    let a = Rc::new(String::from("Rust"));

    // cetak strong count
    println!("Strong count setelah a dibuat : {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);

    // cetak strong count
    println!("Strong count setelah b dibuat : {}", Rc::strong_count(&a));

    let c = Rc::clone(&a);

    // cetak strong count
    println!("Strong count setelah c dibuat : {}", Rc::strong_count(&a));

    drop(c);

    // cetak strong count
    println!("Strong count setelah c di-drop : {}", Rc::strong_count(&a));

    drop(b);

    // cetak strong count
    println!("Strong count setelah b di-drop : {}", Rc::strong_count(&a));

    println!("{}", a);
}

//soal 5
#[test]
fn main5() {

    let counter = Rc::new(
        RefCell::new(0)
    );

    let a = Rc::clone(&counter);
    let b = Rc::clone(&counter);

    // a menambah 5
    {
        *a.borrow_mut() += 5;
    }

    // b menambah 10
    *b.borrow_mut() += 10;
    drop(b);

    println!("{}", counter.borrow());
}


// -------------- mini project -------------//
// 1 todo list

struct Todolist {
    todos: RefCell<Vec<String>>,
}

impl Todolist {
    fn add(self: &Todolist, new_todo: String) {
        self.todos.borrow_mut().push(new_todo);
    }


}