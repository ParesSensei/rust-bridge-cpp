#[derive(Debug)]
struct Point {x: u32, y: u32}
impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point {x, y}
    }
    fn increment_x(&mut self) {
        self.x += 1;
    }

    fn add(&mut self, other: Point) {
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
    let mut p = Point::new(10, 20);
    p.increment_x();


    let mut tes = Point::new(10, 11);
    tes.add(p);
    // println!("{tes:?}")

    let transformed = tes.transform();

    println!("{:?}", transformed);
}