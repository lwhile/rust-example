// this function own right of box and can destroy it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// this function borrow a i32 type
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    // create a boxed i32 type, and a i32 type existed in stack
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // borrow content of box, but not obtains it's right,so the content can be borrowed again
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // give a reference that point to data in box
        let _ref_to_i32: &i32 = &boxed_i32;

//        eat_box_i32(boxed_i32);
    }
    eat_box_i32(boxed_i32)
}