fn main() {
    let number = 3;
    // condition must be a bool; Rust doesn't try to convert
    if number < 5 {
        println!("first condition was true");
    } else if number < 7 {
        println!("second condition was true");
    } else {
        println!("both conditions were false");
    }

    // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
    // the values that have the potential to be results from each arm of the if must be the same type
    // This won’t work because variables must have a single type, and Rust needs to know at compile time what type the number variable is, definitively.
    //  Knowing the type of number lets the compiler verify the type is valid everywhere we use number. Rust wouldn’t be able to do that if the type of number was only determined at runtime; the compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!(number);
}
