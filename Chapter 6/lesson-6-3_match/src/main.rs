#[derive(Debug)]
enum UsState {
    _Alabama,
    Alaska,
}

enum Coin {
    _Penny,
    _Nichel,
    _Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::_Penny => 1,
        Coin::_Nichel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}!", state);
            25
        },
    }
}

fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(c));
}
