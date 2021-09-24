fn main() {
    // if expression
    let number = 3;

    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }

    let testif = if number == 3 { 5 } else { 6 };
    // let testif = if number == 3 { 5 } else { "6" };  // error
    println!("the value of testif is {}", testif);

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!");

    // for loop
    let nums = [10, 20, 30, 40, 50];
    for element in nums.iter() {
        println!("the value is {}", element);
    }

    // Range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
