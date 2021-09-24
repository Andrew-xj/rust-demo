fn main() {
    another_function(5);    // argument

    // expression & statement
    let x = 5;

    let y = {
        let x = x + 1;
        x + 3
    };

    println!("The value of y is {}", y);

    // return value
    let x = plus_five(6);
    println!("The value of x is {}", x);
}

fn another_function(x: i32){    // parameter
    println!("the value of x is {}", x);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}
