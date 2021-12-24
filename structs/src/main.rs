#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coordniates: ({:?}, {:?})", point.x, point.y);

    let p1 = Point { x: 5.2, ..point };

    println!("second point: ({:?}, {:?}", p1.x, p1.y);

    let Point { x: left_edge, y: top_edge } = point;
}
