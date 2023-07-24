// Variable bindings

// mutability
/* fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("Before mutation: {}", mutable_binding);
}
 */

// scope and shadowing

fn main() {
    let outer_variable = 5;
    {
        let outer_variable = "abc";
        println!("Inside block: {}", outer_variable);
    }
    println!("Outside block: {}", outer_variable);
    let outer_variable = "hello";
    println!("After shadowing: {}", outer_variable);

    let number;
    // if we use number before initialization, we get error

    {
        let x = 2;
        number = x * x;
    }

    println!("Number: {}", number);

    // Freezing

    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer;
        // now it is immutable, therefore frozen in this scope
    }
    _mutable_integer = 7;
    // not frozen in this scope
}
