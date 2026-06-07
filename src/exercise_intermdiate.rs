#[derive(Debug)]
struct Point {x: u32, y: u32}
impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point {x, y}
    }
    #[allow(unused)]
    fn increment_x(&mut self) {
        self.x += 1;
    }

    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }

    fn transform(self) -> Self {
        let x1 = self.x * self.x;
        let y1 = self.y * self.y;
        Point::new(x1, y1)
    }
}

#[test]
fn main() {
    let mut p1 = Point::new(2, 3);
    let p2 = Point::new(10, 20);
    p1.add(&p2);
    println!("After add: x = {}, y= {}", p1.x, p1.y);           // x=12, y=23
    let p3 = p1.transform();
    println!("After transform: x={}, y={}", p3.x, p3.y);     // x=144, y=529
    // p1 is no longer accessible — transform() consumed it
}