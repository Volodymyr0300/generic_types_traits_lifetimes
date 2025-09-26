fn generic_structs_definitions() {
    println!("Generic Struct Definitions:");
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

fn generic_enum_definitions() {
    println!("Generic Enum Definitions:");
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

fn generic_in_method_definitions() {
    println!("Generic in Method Definitions:");
    
    fn generic_in_method_definitions_T() {
        struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x(): {}", p.x());
    }
    
    fn generic_in_method_definitions_T_U() {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> { // Note the <T, U> here as well
            fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> { // Note the <V, W> here as well 
                Point { // Note the <T, W> here as well
                    x: self.x, // T from self
                    y: other.y, // W from other
                }
            }
        }

        let p1 = Point { x: 5, y: 10.4 }; // T is i32, U is f64
        let p2 = Point { x: "Hello", y: 'c' }; // V is &str, W is char
        let p3 = p1.mixup(p2); // T is i32, W is char from p2
        println!("p3.x, p3.y: {}, {}", p3.x, p3.y);
        
    }

    generic_in_method_definitions_T();
    generic_in_method_definitions_T_U();
}


fn main() {
    generic_structs_definitions();
    generic_enum_definitions();
    generic_in_method_definitions();
}