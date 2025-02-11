fn main() {
    let x = 2.0; // f64 default

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    //Boolean
    let t = true;
    let f:bool = false; // explicit type annotation

    //char
    let c ='z';
    let z: char = 'Z'; // explicit type annotation

    //compound
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // you can also index into the tuple
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    //arrays
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b = [3; 5]; // will mean b = [3,3,3,3,3]

    //indexing into arrays
    let first = a[0];
    let second = a[1];
    // indexing into an element (from user input) that doesn't exist will cause an error at runtime

}