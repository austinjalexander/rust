// https://doc.rust-lang.org/std/prelude/index.html

// The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
use rand::Rng;
// The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.
use std::cmp::Ordering;
// or use std::io::stdin below
use std::io;


fn main() {
    loop {
        println!("Guess the number!");

        // we call the rand::thread_rng function that gives us the particular random number generator that we’re going to use: one that is local to the current thread of execution and seeded by the operating system. Then we call the gen_range method on the random number generator. This method is defined by the Rng trait that we brought into scope with the use rand::Rng statement. The gen_range method takes a range expression as an argument and generates a random number in the range. The kind of range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds, so we need to specify 1..=100 to request a number between 1 and 100.
        // Unless otherwise specified, Rust defaults to an i32, which is the type of secret_number unless you add type information elsewhere that would cause Rust to infer a different numerical type.
        let secret_number = rand::thread_rng().gen_range(1..=100);
    
        println!("The secret number is: {secret_number}");
    
        println!("Please input your guess.");
    
        // String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
        // The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string. You’ll find a new function on many types, because it’s a common name for a function that makes a new value of some kind.
        let mut guess = String::new();
    
        // The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.
        // like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable
        /*
        As mentioned earlier, read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value. Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.
    
        Chapter 6 will cover enums in more detail. The purpose of these Result types is to encode error-handling information.
    
        Result's variants are Ok and Err. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation failed.
    
        Values of the Result type, like values of any type, have methods defined on them. An instance of Result has an expect method that you can call. If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect. If the read_line method returns an Err, it would likely be the result of an error coming from the underlying operating system. If this instance of Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in the user’s input.
    
        If you don’t call expect, the program will compile, but you’ll get a warning:
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // We create a variable named guess. But wait, doesn’t the program already have a variable named guess? It does, but helpfully Rust allows us to shadow the previous value of guess with a new one. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess for example. this feature is often used when you want to convert a value from one type to another type.
        // We need to tell Rust the exact number type we want by using let guess: u32. The colon (:) after guess tells Rust we’ll annotate the variable’s type.
        // the u32 annotation in this example program and the comparison with secret_number means that Rust will infer that secret_number should be a u32 as well. So now the comparison will be between two values of the same type!
        // If parse returns an Err Result variant because it couldn’t create a number from the string, the expect call will crash the game and print the message we give it. If parse can successfully convert the string to a number, it will return the Ok variant of Result, and expect will return the number that we want from the Ok value.
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            // parse produced the num value and placed it inside Ok
            Ok(num) => num,
            // The underscore, _, is a catchall value; in this example, we’re saying we want to match all Err values, no matter what information they have inside them.
            Err(_) => continue,
        };
    
        // placeholder
        println!("You guessed: {guess}");
        // println!("You guessed: {}", guess)
    
        //  The cmp method compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with
        // Then it returns a variant of the Ordering enum we brought into scope with the use statement. We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number.
        // A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn. Patterns and the match construct are powerful Rust features that let you express a variety of situations your code might encounter and make sure that you handle them all.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
