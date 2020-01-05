enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
    V8()
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number = None;

    let x: i8 = 5;
    let y = Some(5);

    let sum = x + y;
}

fn route(ip_kind: IpAddr) {}

enum Option<T> {
    Some(T),
    None,
}