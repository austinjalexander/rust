fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // you can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it
            // Austin: this seems odd, since the other block example seemed to indicate that adding a ; would prevent a value from being returned
            // 'break expressions': https://doc.rust-lang.org/std/keyword.break.html
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // If you have loops within loops, break and continue apply to the innermost loop at that point. You can optionally specify a loop label on a loop that we can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote. 
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    // as an alternative approach to a while loop, where the index of the array was incremented in each iteration, the for loop approach prevents the compiler from adding runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop
    // also eliminated the possible dev error of not indexing properly
    for element in a {
        println!("the value is: {element}");
    }

    // uses a Range, which is provided by the standard library
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
