fn main() {
    // integer
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // value operation
    let _add = 5 + 10;
    let _minus = 95.5 - 4.3;
    let _multiply = 4 * 30;
    let _division = 56.7 / 32.2;
    let _reminder = 54 % 5;
}
