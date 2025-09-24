
struct Point<T, U> {
    x: T,
    y: T,
    z: U,
}

fn main() {
    

    let both_integer = Point {x:5, y:10, z:15};
    println!("both_integer.x, y, z = {} {} {}", both_integer.x, both_integer.y, both_integer.z);

    let both_float = Point {x:1.0, y:4.0, z:5.0};
    println!("both_float.x, y, z = {} {} {}", both_float.x, both_float.y, both_float.z);

    let integer_and_float = Point {x:5, y:10, z:4.0};
    println!("integer_and_float.x, y, z = {} {} {}", integer_and_float.x, integer_and_float.y, integer_and_float.z);
}
