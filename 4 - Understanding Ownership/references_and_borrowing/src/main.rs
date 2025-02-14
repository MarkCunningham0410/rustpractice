fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2 = {s2}");

    // let mut error_string = String::from("error");
//     let r1 = &mut error_string;
//     let r2 = &mut error_string;
//     println!("r1 = {r1}, r2 = {r2}");
//  This won't work - cannot have more than one mutable references to a value in the same scope

    let mut works_string = String::from("works");
    {
        let r1 = &mut works_string;
        println!("r1 = {r1}");
    }
    let r2 = &mut works_string;
    println!("r2 = {r2}");
    // this will work


    // dangling references

    let reference_to_nothing = dangle();
    println!("reference_to_nothing = {reference_to_nothing}");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a string
    s.len()
} //  here, s goes out of scope. But because it doesn't have ownership of the value, the value
// is not dropped
// this reference is immutable, therefore cannot be edited as it doesn't own the value.
// we're not allowed to modify something we have a reference to.


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");
    //&s // this will break, need to return the string directly (cannot dangle reference to nothing)
    s
}