struct Point {x: u32, y: u32}
struct TransposePoint {x:u32, y:u32}
// Construct a Point from a tuple
impl From<(u32, u32)> for Point {
    fn from(xy : (u32, u32)) -> Self {
        Point {x : xy.0, y: xy.1}       // Construct Point using the tuple elements
    }
}

// Implement a From trait for Point to convert into a type called TransposePoint.
// TransposePoint swaps the x and y elements of Point

impl From<Point> for TransposePoint {
    fn from(p : Point) -> Self {
        TransposePoint {x : p.y, y : p.x}
    }
}
#[test]
fn main() {
    let s = String::from("Rust");
    let x = u32::from(true);
    let p = Point::from((40, 42));

    // let p : Point = (40,42).into(); // Alternate form of the above
    println!("s: {s}\n x:{x}\n p.x:{} p.y {}", p.x, p.y);
    let n: TransposePoint = p.into();
    println!("Transposed : x = {}, y = {}", n.x, n.y);

    let p = Point { x: 10, y: 20 };
    let tp = TransposePoint::from(p);
    println!("Transposed: x={}, y={}", tp.x, tp.y);  // x=20, y=10

    // Using .into() — works automatically when From is implemented
    let p2 = Point { x: 3, y: 7 };
    let tp2: TransposePoint = p2.into();
    println!("Transposed: x={}, y={}", tp2.x, tp2.y);  // x=7, y=3
}


// rust default trait

#[derive(Debug)]
struct CustomPoint {x: u32, y: u32}
impl Default for CustomPoint {
    fn default() -> Self {
        CustomPoint {x: 42, y: 42}
    }
}

#[test]
fn main2() {
    let x = CustomPoint::default();
    // Override y, but leave rest of elements as the default
    let y = CustomPoint {y: 43, ..CustomPoint::default()};
    println!("{x:?} {y:?}");
    let z : Option<CustomPoint> = None;
    // Try changing the unwrap_or_default() to unwrap()
    println!("{:?}", z.unwrap_or_default());
}