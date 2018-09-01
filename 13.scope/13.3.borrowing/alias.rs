struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point{x:1, y:1, z:1};
    {
        let borrowed_point = &point;
        let another_point = &point;

        // access data by reference and origin date
        println!("Point has coordinates: ({}, {}, {})", borrowed_point.x,
                 another_point.y,
                 another_point.z);

        // can't borrow point as mutable content, because it has been become immutable content
        // let mutable_borrow = &mut point;
    }

    {
        let mutable_borrow = &mut point;
        mutable_borrow.x = 2;
        mutable_borrow.z = 2;
        mutable_borrow.y = 2;

    }
}

