use std::io;    // prelude
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数字！");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("猜测一个数");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        println!("你猜测的数是：{}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
