fn main() {
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);

    println!("tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);

    // array
    let _a = [0, 1, 2, 3, 4];
    let months = [
        "January", "February", "March", "April",
        "May", "June", "July", "August",
        "September", "October", "November", "December"
    ];
    let _first = months[0];
    let _second = months[1];
    // let x = months[15];  // panic
}
