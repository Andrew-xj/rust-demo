enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    _kind: IpAddrKind,
    _address: String,
}

enum Ipaddrkind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { _x: i32, _y: i32 },
    Write(String), 
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    // example 1
    let _home1 = IpAddr {
        _kind: IpAddrKind::V4,
        _address: String::from("127.0.0.1"),
    };

    let _loopback1 = IpAddr {
        _kind: IpAddrKind::V6,
        _address: String::from("::1"),
    };

    // example 2
    let _home2 = Ipaddrkind::V4(127, 0, 0, 1);
    let _loopback2 = Ipaddrkind::V6(String::from("::1"));

    // example 3
    let _q = Message::Quit;
    let _m = Message::Move { _x: 12, _y: 24 };
    let _w = Message::Write(String::from("Hello"));
    let _c = Message::ChangeColor(0, 255, 255);
    
    // method
    _m.call();
}
