// unit struct
struct Nil;

// tuple struct
struct Pair(i32, f32);

// normal struct
struct Point {
    x: f32,
    y: f32
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point
}

fn main() {
    //  instantiation struct `Point`
    let point: Point = Point{x: 0.3, y: 0.4};

    // access field of point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // use `let` binding to destruction point
    let Point{x: my_x, y:my_y} = point;

    let _rectangle = Rectangle{
        // struct instantiation also a statement
        p1: Point{x:my_x, y:my_y},
        p2: point,
    };

    // instantiation a unit struct
    let _nil = Nil;

    let pair = Pair(1,0.1);

    // access tuple struct fields
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // destruct a tuple struct
    let Pair(integer,decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal)
}