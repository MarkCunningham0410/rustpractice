fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    let x = five();
    println!("The value of x is: {x}");

    // let x = (let y = 6); Does not work in rust
    let y = return_function(5);
    println!("The value of x+1 is: {y}")
}
fn another_function(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}
fn five() -> i32 {
    5
}

fn return_function(x: i32) -> i32 {
    x+1
}