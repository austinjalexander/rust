// Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. 
fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    // assignment statements do not evaluate as return values
    // thus, no let x = y = 6
    // Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression
    // Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.
    let y = {
        x + 1
    };
    another_function(x + y);
}

// the unit type:
// https://doc.rust-lang.org/std/primitive.unit.html
// The () type has exactly one value (), and is used when there is no other meaningful value that could be returned. () is most commonly seen implicitly: functions without a -> ... implicitly have return type ()
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}