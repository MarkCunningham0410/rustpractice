fn main() {

    let s1 = gives_ownership(); // moves return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_gives_Back, then returned to s3

    let (s4, len) = calculate_length(s1);

    println!("The length of '{s4}' is {len}.");
} // s3 is out of scope and dropped. s2 was moved, so nothing happens, s1 goes out of scope and dropped.

fn gives_ownership() -> String {
    let s = String::from("yours"); //some_string comes into scope

    s // returned and moves out to the calling_function
}

fn takes_and_gives_back(a_string: String) -> String { //a_string comes into scope

    a_string // returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}