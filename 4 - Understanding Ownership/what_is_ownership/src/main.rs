fn main() {
   {
       // s is not valid here, not yet declared
       let s = "hello"; // s is valid from here on
       println!("{s}, world!");
       // do stuff with s
   } // scope is now over, s is no longer valid

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, world!"); does not work, prevented from invalidated reference

    println!("{s2}, world!");

    let s1 = String::from("what's up");
    let s2 = s1.clone(); // expensive, but heap data is copied, so this works

    println!("{s1}, {s2}");

    //stack only data: copy

    let x =6;
    let y = x;
    println!("{x}, {y}"); // stored on stack, cheap, this works

    // ownership and functions

    let our_string = String::from("testing"); // string comes into scope
    takes_ownership(our_string);// string's value moves into the function
                                // string would no longer be valid here

    let numerical_value = 10; // number comes into scope
    makes_copy(numerical_value); // because i32 has the copy trait,
                                // the number itself doesn't move into the function
    println!("{numerical_value}"); //so it is okay to use the number afterwards
}

fn takes_ownership(some_string: String) { // some string comes into scope
    println!("{some_string}");
} // string is now out of scope and 'drop' is called, memory is freed

fn makes_copy(some_integer: i32){ //some integer comes into scope
    println!("{some_integer}");
}// integer goes out of scope, nothing happens.
