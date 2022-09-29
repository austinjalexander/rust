fn main() {
    // The types covered previously are all a known size, can be stored on the stack and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same value in a different scope.

    let _s = "hello"; // string literal; hardcoded; immutable
    // In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable.

    // the String type manages data allocated on the heal and is able to store an amount of text unknown at compile time
    let _t = String::from("hello"); // :: operator for namespacing
    let mut u = String::from("hello"); // this type can be mutated
    u.push_str(", world!");
    println!("{}", u);
    // With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
    // The memory must be requested from the memory allocator at runtime.
    // We need a way of returning this memory to the allocator when we’re done with our String.
    // That first part is done by us: when we call String::from, its implementation requests the memory it needs. This is pretty much universal in programming languages.
    // However, the second part is different. In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it. In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and call code to explicitly free it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.
    // Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
    // There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.
    // Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.

    let _x = 5; // bind fixed value 5 to x; value is on the stack
    let _y = _x; // copy value and bind to y; value is on the stack
    // types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying and we can leave it out.

    let _s1 = String::from("yo"); // ptr, len, cap on stack; value stored on the heap; ptr --> index --> value
    let _s2 = _s1; // only values on stack are copied

    // ...both data pointers pointing to the same location. This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

    // To ensure memory safety, after the line let s2 = s1, Rust considers s1 as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what happens when you try to use s1 after s2 is created; it won’t work. You’ll get an error like this because Rust prevents you from using the invalidated reference:
    /*
    move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 | 
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  For more information about this error, try `rustc --explain E0382`
    */
    // If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of calling it a shallow copy, it’s known as a move. 
    // That solves our problem! With only s2 valid, when it goes out of scope, it alone will free the memory, and we’re done.
    // In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

    // If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
    let t1 = String::from("hello");
    let t2 = t1.clone();
    println!("{} {}", t1, t2);

    // Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are (we’ll talk more about traits in Chapter 10). If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
    // Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error. To learn about how to add the Copy annotation to your type to implement the trait, see “Derivable Traits” in Appendix C.
    /*
    So what types implement the Copy trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:
        All the integer types, such as u32.
        The Boolean type, bool, with values true and false.
        All the floating point types, such as f64.
        The character type, char.
        Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
    */

    let z = String::from("yolo");
    takes_ownership(z); // z's String value moves into the function, so z is no longer valid in this scope

    let i = 7;
    makes_copy(i); // i32 is Copy, so it's ok to use x later in this scope

    //println!("{}", z); // would throw: - move occurs because `z` has type `String`, which does not implement the `Copy` trait; ^ value borrowed here after move
    println!("{}", i);

    let z1 = gives_ownership(); // this func moves its return value into z1
    println!("{}", z1);

    let z2 = String::from("STRING");
    let z3 = takes_and_gives_back(z2); // z2 is moved into func, which moves its return value into z3
    println!("{}", z3);
    // z1 and z3 go out of scope after main and are dropped

    // While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.
    // Instead, we can provide a reference to the String value. A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
    // References allow you to refer to some value without taking ownership of it.
    // We call the action of creating a reference borrowing.

    let msg = String::from("please enter");
    // Rust does let us return multiple values using a tuple
    let (v, l) = calculate_length(msg);

    println!("length of '{}' is {}", v, l);

    let msg2 = String::from("please enter");
    let len = calculate_length_v2(&msg2);

    println!("length of '{}' is {}", msg2, len);

    let mut txt = String::from("hello");
    change(&mut txt);
    println!("{}", txt);

    // Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. 
    // can't do:
    /*
    let mut r_str = String::from("hello");
    let _r1 = &mut r_str;
    let _r2 = &mut r_str;
    println!("{}", _r1);

    This error says that this code is invalid because we cannot borrow s as mutable more than once at a time. The first mutable borrow is in r1 and must last until it’s used in the println!, but between the creation of that mutable reference and its usage, we tried to create another mutable reference in r2 that borrows the same data as r1.

The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with, because most languages let you mutate whenever you’d like. The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

Two or more pointers access the same data at the same time.
At least one of the pointers is being used to write to the data.
There’s no mechanism being used to synchronize access to the data.
Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!
    */
    let mut r_str = String::from("hello");
    // As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
    {
        let _r1 = &mut r_str;
        println!("{}", _r1);
    }
    let _r2 = &mut r_str;

    /*
    Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s // no problem
    let r3 = &mut s; // BIG PROBLEM
    println!("{}, {}, {}", r1, r2, r3);

    We also cannot have a mutable reference while we have an immutable one to the same value.

Users of an immutable reference don’t expect the value to suddenly change out from under them! However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
    */

// The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created. These scopes don’t overlap, so this code is allowed. The ability of the compiler to tell that a reference is no longer being used at a point before the end of the scope is called Non-Lexical Lifetimes (NLL for short), and you can read more about it in The Edition Guide.
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    
    let r3 = &mut s;
    println!("{}", r3);

    // dangling references
    // In languages with pointers, it’s easy to erroneously create a dangling pointer--a pointer that references a location in memory that may have been given to someone else--by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
    // let ref_to_nada = dangle();

    /*
    - At any given time, you can have either one mutable reference or any number of immutable references.
    - References must always be valid.
    */

    let s = String::from("testing string stuff");

    let _slice = &s[0..2];
    // can omit starting index, if i == 0
    let _slice = &s[..2];
    let _slice = &s[3..len];
    // can omit ending index, if i == len
    let _slice = &s[..len];
    // thus:
    let _slice = &s[0..len];
    // is equivalent to:
    let _slice = &s[..];

    let first = first_word(&s);
    // would throw an error if s was mutable
    // s.clear();

    println!("{}", first)

} 

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn makes_copy(int: i32) {
    println!("{}", int);
}

fn gives_ownership() -> String {
    let s = String::from("yours");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_v2(s: &String) -> usize {
    s.len()
    // since borrowing, attempting to modify (by default) would throw an error
    // s.push_str("blah blah");
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

fn change(s: &mut String) {
    s.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // reference is returned
// } // but here s goes out of scope and so is dropped; instead, s should simply be returned as a value

// because usize is only meaningful in the context of &String,
// there's no guarantee that it will be valid in the future
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         // byte literal syntax
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }
// instead can use string slices (where ending index is exclusive):
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
