fn main() {
    // Mutability
    let mut x = 5; // must declare as mut so that the value can be changed later
    println!("The value of x is: {x}");

    x = 6; // this would flag an error if we hadn't used 'mut' in line 3
    println!("The value of x is: {x}");

    //Constants
    const MAX_POINTS: u32 = 100_000; // note the convention of capitals and underscore
    println!("The value of MAX_POINTS is: {MAX_POINTS}");

    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60; // easier to understand and verify than 10,800
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing - the second variable overshadows the first

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    } // when this scope is over, value of y returns to 6

    println!("The value of y is: {y}");

}
