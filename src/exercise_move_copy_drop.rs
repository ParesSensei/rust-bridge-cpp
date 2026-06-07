//
// #[derive(Debug, Clone)]
// struct Point{x: u32, y: u32}
//
//
// impl Point {
//     fn add(&self, other: &Point) -> Point {
//         Point {x: self.x + other.x, y: self.y + other.y}
//     }
//
//     fn substract(&self, other: &Point) -> Point {
//         Point {x: self.x - other.x, y: self.y - other.y}
//     }
// }
//
// impl Drop for Point {
//     fn drop(&mut self) {
//         println!("Goodbye! point drop coordinate x: {}, y: {}", self.x, self.y);
//     }
// }
//
// #[test]
// fn main() {
//     // Create Point, assign it to a different variable, create a new scope,
//     // pass point to a function, etc.
//     let p = Point{x: 5, y: 7};
//     println!("Created main p: {}, {}", p.x, p.y);
//
//     let y = p.clone();
//     println!("Created main y: {}, {}", y.x, y.y);
//     {
//         let p2 = Point{x: 3, y: 7};
//         println!("Created inner p2: {}, {}", p2.x, p2.y);
//
//         let p3 = p2.add(&p);
//         println!("Created inner p3: {}, {}", p3.x, p3.y);
//     }
//
//     let x = p.add(&y);
//     println!("Created main x: {}, {}", x.x, x.y);
//
//     let z = x.substract(&p);
//     println!("Created main z: {}, {}", z.x , z.y + 1);
// }


#[derive(Debug)]
struct Point { x: u32, y: u32 }

impl Drop for Point {
    fn drop(&mut self) {
        println!("Dropping Point({}, {})", self.x, self.y);
        self.x = 0;
        self.y = 0;
        // Note: setting to 0 in drop demonstrates the pattern,
        // but you can't observe these values after drop completes
    }
}

fn consume(p: Point) {
    println!("Consuming: {:?}", p);
    // p is dropped here
}


#[test]
fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1;  // Move — p1 is no longer valid
    // println!("{:?}", p1);  // Won't compile: p1 was moved

    {
        let p3 = Point { x: 30, y: 40 };
        println!("p3 in inner scope: {:?}", p3);
        // p3 is dropped here (end of scope)
    }

    consume(p2);  // p2 is moved into consume and dropped there
    // println!("{:?}", p2);  // Won't compile: p2 was moved

    // Now try: add #[derive(Copy, Clone)] to Point (and remove the Drop impl)
    // and observe how p1 remains valid after let p2 = p1;
}
// Output:
// p3 in inner scope: Point { x: 30, y: 40 }
// Dropping Point(30, 40)
// Consuming: Point { x: 10, y: 20 }
// Dropping Point(10, 20)