

fn generic_structs_definitions() {

    struct Point<T, U> {
        x: T,
        y: U,
    }

    let both_integer = Point { x: 5, y: 10 };
    println!("both_integer.x, both_integer.y: {}, {}", both_integer.x, both_integer.y);

    let both_float = Point { x: 1.0, y: 4.0 };
    println!("both_float.x, both_float.y: {}, {}", both_float.x, both_float.y);

    let integer_and_float = Point { x: 5, y: 4.0 };
    println!("integer_and_float.x, integer_and_float.y: {}, {}", integer_and_float.x, integer_and_float.y);
}

fn main() {
    generic_structs_definitions();
}