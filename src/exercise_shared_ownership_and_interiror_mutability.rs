use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Debug)]
struct Employee {
    employee_id: u64,
    name: RefCell<String>,
    on_vacation: Cell<bool>,
}

fn toggle_vacation(emp: &Employee) {
    // TODO: Flip on_vacation using Cell::set()
    emp.on_vacation.set(!emp.on_vacation.get());
}

fn append_title(emp: &Employee, title: &str) {
    // TODO: Borrow name mutably via RefCell and push_str the title
    emp.name.borrow_mut().push_str(title);
}

#[test]
fn main() {
    // TODO: Create an employee, wrap in Rc, clone into two Vecs,
    // call toggle_vacation and append_title, print results

    let employee = Rc::new(
        Employee{
            employee_id: 77,
            name: RefCell::new("awa".to_string()),
            on_vacation: Cell::new(false),
        }
    );
    let mut us_employee = vec![];
    let mut global_employee = vec![];
    us_employee.push(Rc::clone(&employee));
    global_employee.push(Rc::clone(&employee));

    toggle_vacation(&employee);
    println!("On vaccation : {}", employee.on_vacation.get());

    append_title(&employee, ", Sr. computer science");
    println!("name: {}", employee.name.borrow());

    println!("US: {}", us_employee[0].name.borrow());
    println!("US: {}", global_employee[0].name.borrow());
    println!("US: {}", Rc::strong_count(&employee));
}