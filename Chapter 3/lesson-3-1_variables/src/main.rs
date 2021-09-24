const MAX_POINTS: u32 = 2;

fn main() {
    // let x = 5;   // immutable
    println!("Test 1: mutable");
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}\n", x);

    // Shadowing
    println!("Test 2: shadow");
    let y = 5;
    println!("The value of y is {}", y);
    let y = y + 1;
    println!("The value of y is {}", y);
    let y = y * MAX_POINTS;
    println!("The value of y is {}", y);
}
