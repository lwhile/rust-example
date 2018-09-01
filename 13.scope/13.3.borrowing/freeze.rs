fn main() {
    let mut _mutable_integer = 7i32;
    {
        // borrow `_mutable_integer`
        let _large_integer = &_mutable_integer;

        // error
        _mutable_integer = 50;

        // leave scope
    }
    _mutable_integer = 3;
}